use base64::{engine::general_purpose, Engine as _};
use encryption::password_hash;
use futures::future::Ready;
// use paperclip::actix::api_v2_operation;
use actix_identity::Identity;
use lettre::message::header::ContentType;
use rand::Rng;
// use schemars::schema_for;

use sha2::{Digest, Sha256};

use actix_web::{
    delete, error, get, patch, post, web, FromRequest, HttpMessage, HttpRequest, HttpResponse,
    Responder, Result,
};
use deadpool_postgres::{Client, Pool};
use unicode_normalization::UnicodeNormalization;
//use sqlx::PgPool;

use crate::auth::db;
use crate::auth::model::{CreateUser, Session, SessionAdd};
use crate::configs::{self, Config};
// use crate::mail::model::Message;
use lettre::Message;
// use crate::auth::{db, UISchemaField, UserUISchema};
use crate::errors::ServiceError;
use crate::mail::send_email;

use super::{
    add_session, delete_session, encryption, find_user_by_session, find_user_mail_by_id,
    find_user_passowrd_by_mail, find_user_password_validated_hash, hex_to_bytes,
    session_otp_set_attempts, session_otp_update_confirm_true, session_otp_update_true,
    user_reset_password_update, user_update_password_hash, Otp, Params, Reset, UserMail, UserPw,
};

impl FromRequest for Session {
    type Error = ServiceError;
    type Future = Ready<Result<Session, ServiceError>>;

    fn from_request(req: &HttpRequest, pl: &mut actix_web::dev::Payload) -> Self::Future {
        if let Ok(identity) = Identity::from_request(req, pl).into_inner() {
            if let Some(session_id_and_verifier) = identity.id().ok() {
                let parsed_cookie: Result<Session, serde_json::Error> =
                    serde_json::from_str(&session_id_and_verifier);
                if let Ok(parsed_cookie) = parsed_cookie {
                    let mut hasher = Sha256::new();
                    let bytes = hex::decode(&parsed_cookie.session_verifier);
                    if let Ok(bytes) = bytes {
                        hasher.update(bytes);
                        let hex_hashed_session_verifier = hex::encode(hasher.finalize());
                        // println!(
                        //     "From Request: {:#?} | requests: {:#?}",
                        //     &parsed_cookie, &req
                        // );
                        return futures::future::ok(Session {
                            session_id: parsed_cookie.session_id,
                            session_verifier: hex_hashed_session_verifier,
                            master_key_hash: parsed_cookie.master_key_hash,
                        });
                    }
                }
            }
        }
        futures::future::err(ServiceError::Unauthorized)
    }
}

pub async fn session_create(
    pool: web::Data<Pool>,
    reqs: HttpRequest,
    user_id: i32,
    master_key_hash: Option<String>,
) -> Result<Session, ServiceError> {
    let config = configs::Config::from_env().unwrap();
    let hex = &config.srv_cnf.secret_key;
    let hex = encryption::hex_to_bytes(&hex).expect("SECRET_KEY could not parse");
    let mut rng = rand::thread_rng();
    let otp_code: u32 = rng.gen_range(10000..99999);
    let otp_encrypted =
        encryption::encrypt(&format!("{}", otp_code), &format!("{}", user_id), &hex)?;

    // Create a random session verifier
    let random_bytes = rand::thread_rng().gen::<[u8; 32]>();
    let mut hasher = Sha256::new();

    // Hash it to avoid exposing it in the database.
    hasher.update(random_bytes);
    let hex_hashed_session_verifier = hex::encode(hasher.finalize());

    let sess = SessionAdd {
        user_id,
        session_verifier: hex_hashed_session_verifier,
        otp_code_encr: otp_encrypted,
    };

    // println!("Sess: {:#?}", &sess);

    let client: Client = pool.get().await.expect("Error connecting to the database");
    let sess_res = add_session(&client, sess).await?;

    // println!("Sess_res: {:#?}", &sess_res);

    let session = Session {
        session_id: sess_res.id,
        session_verifier: hex::encode(random_bytes),
        master_key_hash,
    };

    let serialized =
        serde_json::to_string(&session).map_err(|e| ServiceError::FaultySetup(e.to_string()))?;

    // println!(
    //     "Created User Session: {:#?}  Session: {:#?}",
    //     &sess_res, &session
    // );
    // identity.remember(serialized);
    Identity::login(&reqs.extensions(), serialized)
        .map_err(|e| ServiceError::FaultySetup(format!("Error logging in: {}", e.to_string())))?;

    Ok(session)
}

// /// Create User | Top

/// Create an Account
#[utoipa::path(
    context_path = "/auth",
    request_body(content = CreateUser, description = "Create User", content_type = "application/json",  example = json!({"id": 1, "name": "bob the cat"})),
    responses(
        (status = 201, description = "User created successfully", body = CreateUser),
        (status = 409, description = "User with id already exists", body = ServiceError)
    )
)]
#[post("/")]
pub async fn register_user(
    db_pool: web::Data<Pool>,
    jsonusr: web::Json<CreateUser>,
    req: HttpRequest,
) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let config = Config::from_env().unwrap();

    let pwd = password_hash(&jsonusr.hashed_password, config.srv_cnf.bcrypt_or_argon).await;

    let usr = CreateUser {
        email: jsonusr.email.clone(),
        hashed_password: pwd.unwrap(),
    };

    let result = db::add_user(&client, usr).await;
    match result {
        Ok(registered_user) => {
            match session_create(db_pool.clone(), req.clone(), registered_user.id, None).await {
                Ok(ses) => {
                    // if let Err(err) = email_otp(db_pool, Some(ses)).await {
                    //     eprintln!("Email OTP Error: {:?}", err);
                    // }
                    // eprintln!("Session Error: {:?}", Some(&ses));

                    email_otp(db_pool, Some(ses.clone()))
                        .await
                        .expect("Error Sending Mail");
                    HttpResponse::Ok().json(ses)
                }
                Err(err) => {
                    eprintln!("Session Error: {:?}", err);
                    HttpResponse::InternalServerError().json(format!("{:#?}", err))
                }
            }
        }
        Err(err) => {
            session_create(
                db_pool.clone(),
                req.clone(),
                config.srv_cnf.user_invalid_id,
                None,
            )
            .await
            .expect("Error creating session");
            HttpResponse::Conflict().json(format!("{:#?}", err))
        }
    }
}

// async fn ping() -> Result<HttpResponse> {
//     // Create a JSON response
//     // Return the JSON response with HTTP 200 OK status
//     Ok(HttpResponse::Ok()
//         .content_type("application/json")
//         .json("This isa test"))
// }

#[get("/")]
async fn profile(
    identity: Option<Identity>,
    pool: web::Data<Pool>,
) -> actix_web::Result<impl Responder> {
    let client: Client = pool.get().await.expect("Error connecting to the database");
    let id = match identity.map(|id| id.id()) {
        None => "anonymous".to_owned(),
        Some(Ok(mut id)) => {
            let session_data: Session = serde_json::from_str(&id).expect("JSON parsing failed");
            if let Some(user_session) = find_user_by_session(&client, session_data).await {
                let mail_id = find_user_mail_by_id(&client, user_session.user_id).await;
                if let Ok(user) = mail_id {
                    id = user.email;
                }
                //     .content_type("application/json")
                //     .json(user_session));
            }
            id
        }
        Some(Err(err)) => return Err(error::ErrorInternalServerError(err)),
    };

    // Ok(format!("Hello {id}"))
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(format!("{:#?}", &id)))
}

/// Login | Top
///
/// Login your account
#[utoipa::path(
    context_path = "/auth",
    request_body = CreateUser,
    responses(
        (status = 201, description = "User logged successfully", body = CreateUser),
        (status = 409, description = "Authentication Failure", body = ServiceError)
    )
)]
#[post("/login")]
pub async fn process_login(
    pool: web::Data<Pool>,
    req: HttpRequest,
    login: web::Json<CreateUser>,
) -> Result<HttpResponse, ServiceError> {
    let client: Client = pool.get().await.expect("Error connecting to the database");

    let email = &login.email;
    let config = configs::Config::from_env().unwrap();

    match find_user_passowrd_by_mail(&client, email.to_string()).await {
        Ok(user) => {
            if encryption::verify_hash(
                &login.hashed_password.nfkc().collect::<String>(),
                &user.hashed_password,
                config.srv_cnf.bcrypt_or_argon,
            )
            .await?
            {
                match session_create(pool, req.clone(), user.id, None).await {
                    Ok(ses) => {
                        println!("{:#?}", &req);
                        return Ok(HttpResponse::Ok().json(ses));
                    }
                    Err(err) => {
                        eprintln!("Session Error: {:?}", err);
                        return Ok(HttpResponse::InternalServerError().json(format!("{:#?}", err)));
                    }
                }
            } else {
                return Ok(HttpResponse::Unauthorized().json("Authentication failure"));
            }
        }
        Err(_) => {
            return Ok(HttpResponse::NotFound().json("Account does not exist"));
        }
    }
}

pub async fn email_otp(
    pool: web::Data<Pool>,
    session: Option<Session>,
) -> Result<(), ServiceError> {
    let client: Client = pool.get().await.expect("Error connecting to the database");
    let config = configs::Config::from_env().unwrap();
    let secret = hex_to_bytes(&config.srv_cnf.secret_key).expect("SECRET_KEY could not parse");

    // println!("User Session from OTP: {:?}", &session);

    // println!("Email OTP Error: {:?}", &usr_sess);
    if let Some(sess) = session {
        if let Some(user_session) = find_user_by_session(&client, sess).await {
            // println!("Email OTP Error: {:?}", &user_session);
            // session_otp_update_true(&client, user_session.id).await?;
            if !user_session.otp_code_sent {
                session_otp_update_true(&client, user_session.id).await?;
                let mail_id = find_user_mail_by_id(&client, user_session.user_id).await;
                let otp_code = encryption::decrypt(
                    &user_session.otp_code_encrypted,
                    &format!("{}", user_session.user_id),
                    &secret,
                )?;
                if let Ok(db_user) = mail_id {
                    let body =
                        format!(" <p>Welcome: Your Confirmation code is : {} </p>", otp_code);
                    // let message = Message {
                    //     email: db_user.email,
                    //     subject: "Registration: Your Confirmation Code".to_owned(),
                    //     msg: body,
                    // };
                    let message = Message::builder()
                        .from("NoBody <nobody@domain.tld>".parse().unwrap())
                        .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
                        .to(db_user.email.parse().unwrap())
                        .subject("Account Creation")
                        .header(ContentType::TEXT_HTML)
                        .body(String::from(body))
                        .unwrap();

                    send_email(message);
                } else if user_session.user_id == config.srv_cnf.user_invalid_id {
                    // Looks like the an attempt to register a duplicate user
                    // There may be a timing attack here.
                    return Err(ServiceError::BadRequest(
                        "Attempt to register a duplicate user".to_string(),
                    ));
                }
            }
        }
    }

    Ok(())
}

#[post("/confirm")]
pub async fn confirm_otp(
    pool: web::Data<Pool>,
    // session: Option<Session>,
    otp: web::Json<Otp>,
) -> Result<HttpResponse, ServiceError> {
    let config = configs::Config::from_env().unwrap();
    let client: Client = pool.get().await.expect("Error connecting to the database");
    let secret = hex_to_bytes(&config.srv_cnf.secret_key).expect("SECRET_KEY could not parse");
    // println!("OTP Code{:#?}", &otp);
    let new_sess: Option<Session> = Some(Session {
        session_id: otp.session_id,
        session_verifier: otp.session_verifier.clone(),
        master_key_hash: None,
    });

    if let Some(session) = new_sess {
        if let Some(user_session) = find_user_by_session(&client, session).await {
            // If we have more than 1 attempt we need to apply the Hcaptcha
            if user_session.otp_code_attempts > 6 {
                // The hCaptcha was invalid send them back.
                return Ok(HttpResponse::ExpectationFailed().json("OTP Too much attempt"));
            }

            // Brute force detection
            if user_session.otp_code_attempts > config.srv_cnf.max_otp_attempts {
                // In the case of what looks like a brute force, log them out.
                return Ok(HttpResponse::Gone().json("Authentication failure"));
            }

            let otp_code = encryption::decrypt(
                &user_session.otp_code_encrypted,
                &format!("{}", user_session.user_id),
                &secret,
            )?;
            if otp_code == otp.code {
                session_otp_update_confirm_true(&client, user_session.id).await?;

                return Ok(HttpResponse::Accepted().json("Accepted"));
            } else {
                session_otp_set_attempts(&client, user_session.id).await?;

                return Ok(HttpResponse::PreconditionFailed().json("Retry your OTP"));
            }
        }
    }
    return Ok(HttpResponse::Unauthorized().json("Please Login"));
}

pub fn compare_hash_constant_time(x: &[u8], y: &[u8]) -> bool {
    let length = x.len();

    if length != y.len() {
        return false;
    }

    let mut result: u8 = 0;

    for n in 0..length {
        result |= x[n] ^ y[n];
    }

    result == 0
}

#[post("/reset")]
pub async fn reset_pw_request(
    pool: web::Data<Pool>,
    reset: web::Json<UserMail>,
) -> Result<HttpResponse, ServiceError> {
    let client: Client = pool.get().await.expect("Error connecting to the database");
    // let config = configs::Config::from_env().unwrap();

    let email = reset.email.clone();

    let invitation_selector = rand::thread_rng().gen::<[u8; 8]>();
    let invitation_selector_base64 = general_purpose::URL_SAFE_NO_PAD.encode(invitation_selector);
    let invitation_verifier = rand::thread_rng().gen::<[u8; 24]>();
    let invitation_verifier_hash = Sha256::digest(&invitation_verifier);
    let invitation_verifier_hash_base64 =
        general_purpose::URL_SAFE_NO_PAD.encode(invitation_verifier_hash);
    let invitation_verifier_base64 = general_purpose::URL_SAFE_NO_PAD.encode(invitation_verifier);

    let param = Params {
        reset_password_selector: invitation_selector_base64.clone(),
        reset_password_validator: invitation_verifier_hash_base64,
    };

    user_reset_password_update(&client, reset.0, param).await?;
    let server_url = "http://localhost";
    let body = format!(
        "
                        If you requested a password reset please follow this link
                        \n{}/auth/change?reset_password_selector={}&reset_password_validator={}
                        ",
        server_url, invitation_selector_base64, invitation_verifier_base64
    );

    // let message = Message {
    //     email,
    //     subject: "Did you request a password reset?".to_string(),
    //     msg: body,
    // };
    let message = Message::builder()
        .from("NoBody <nobody@domain.tld>".parse().unwrap())
        .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
        .to(email.parse().unwrap())
        .subject("Password Request, did you?")
        .header(ContentType::TEXT_HTML)
        .body(String::from(body))
        .unwrap();

    // let message = Message::builder()
    //     .from("NoBody <nobody@domain.tld>")
    //     .to(email)
    //     .subject("Auth: your Confirmation")
    //     .body(body);

    send_email(message);
    Ok(HttpResponse::Accepted().finish())
}

#[get("/change")]
pub async fn password_change(
    pool: web::Data<Pool>,
    reset: web::Query<Reset>,
) -> Result<HttpResponse, ServiceError> {
    let config = Config::from_env().unwrap();
    let client: Client = pool.get().await.expect("Error connecting to the database");

    let reset = Reset {
        reset_password_selector: reset.reset_password_selector.clone(),
        reset_password_validator: reset.reset_password_validator.clone(),
        ..Default::default()
    };
    let pwd = password_hash(&reset.password, config.srv_cnf.bcrypt_or_argon).await?;

    let user_hash =
        find_user_password_validated_hash(&client, reset.reset_password_selector).await?;

    let reset_password_verifier = &general_purpose::STANDARD_NO_PAD
        .decode(&reset.reset_password_validator)
        .map_err(|e| ServiceError::DecryptError(e.to_string()))?;

    // base64::decode_config(&form.reset_password_validator, base64::URL_SAFE)
    //     .map_err(|e| CustomError::FaultySetup(e.to_string()))?;
    let reset_password_verifier_hash = Sha256::digest(reset_password_verifier);

    let reset_password_verifier_hash_from_db = &general_purpose::STANDARD_NO_PAD
        .decode(&user_hash.reset_password_validator_hash)
        .map_err(|e| ServiceError::DecryptError(e.to_string()))?;

    let usr_pwd = UserPw {
        id: user_hash.id,
        hashed_password: pwd,
    };

    if compare_hash_constant_time(
        &reset_password_verifier_hash,
        &reset_password_verifier_hash_from_db,
    ) {
        user_update_password_hash(&client, usr_pwd).await?;
    }

    return Ok(HttpResponse::Accepted().finish());
}

pub async fn logout(
    id: Identity,
    session: Option<Session>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let client: Client = pool.get().await.expect("Error connecting to the database");

    if let Some(session) = session {
        delete_session(&client, session).await?;
    }

    id.logout();

    return Ok(HttpResponse::Ok().json("Logout Successfully"));
}
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register_user);
    cfg.service(confirm_otp);
    cfg.service(process_login);
    cfg.service(reset_pw_request);
    cfg.service(profile);
}
