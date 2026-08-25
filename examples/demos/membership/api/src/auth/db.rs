use deadpool_postgres::Client;

use sha2::{Digest, Sha256};

use tokio_pg_mapper::FromTokioPostgresRow;

use crate::errors::ServiceError;

use super::model::*;

// Constant time string compare.
pub fn constant_time_compare(a: &str, b: &str) -> bool {
    a.bytes()
        .zip(b.bytes())
        .fold(0, |acc, (a, b)| acc | (a ^ b))
        == 0
}

pub async fn add_user(client: &Client, usr: CreateUser) -> Result<CreatedUser, ServiceError> {
    let statement = client
        .prepare("INSERT INTO public.users (email, hashed_password) VALUES ($1, $2 ) RETURNING id")
        .await?;

    let result = client
        .query_one(&statement, &[&usr.email, &usr.hashed_password])
        .await?;
    let user = CreatedUser::from_row_ref(&result).unwrap(); // or from_row_ref(&result)
    Ok(user)
}

pub async fn add_session(
    client: &Client,
    sess: SessionAdd,
) -> Result<CreatedSession, ServiceError> {
    let statement = client
        .prepare(
            "INSERT INTO public.sessions (user_id, session_verifier, otp_code_encrypted)
            VALUES($1, $2, $3) RETURNING id",
        )
        .await
        .unwrap();

    let result = client
        .query_one(
            &statement,
            &[&sess.user_id, &sess.session_verifier, &sess.otp_code_encr],
        )
        .await?;
    // println!("Added Session: {:#?}", &sess);
    let sess = CreatedSession::from_row_ref(&result).unwrap(); // or from_row_ref(&result)
    Ok(sess)
}

pub async fn delete_session(client: &Client, sess: Session) -> Result<(), ServiceError> {
    let statement = client
        .prepare("DELETE FROM public.sessions WHERE id = $1")
        .await
        .unwrap();

    client
        .execute(&statement, &[&sess.session_id])
        .await
        .unwrap();
    Ok(())
}

pub async fn find_user_by_session(client: &Client, session: Session) -> Option<UserSession> {
    let statement = client
        .prepare(
            " SELECT
            id,
            user_id,
            session_verifier,
            otp_code_confirmed,
            otp_code_encrypted,
            otp_code_attempts,
            otp_code_sent FROM public.sessions WHERE id = $1",
        )
        .await
        //  .map_err(|e| format!("Error preparing statement: {}", e))?;
        .unwrap();

    let maybe_session = client
        .query_opt(&statement, &[&session.session_id])
        .await
        // .ok()
        .expect("User from session not found")
        .map(|row| UserSession::from_row_ref(&row).unwrap());
    // .map(|row| UserSession::from_row(row))
    // .filter(|ses| constant_time_compare(&sess.session_verifier, &sess.session_verifier));

    // println!(
    //     "DB Session Error: -1 {:#?} -2 {:#?}",
    //     &session.session_verifier, &maybe_session
    // );
    maybe_session.and_then(|sess| {
        let decoded_hash = hex::decode(&session.session_verifier);
        let mut new_hasher = Sha256::new();
        match decoded_hash {
            Ok(bytes) => {
                new_hasher.update(bytes);
                let ver_hash = hex::encode(new_hasher.finalize());
                // println!(
                //     "DB Session Error: -1 {:#?} -2 {:#?}",
                //     &sess.session_verifier, &ver_hash
                // );
                if constant_time_compare(&ver_hash, &sess.session_verifier) {
                    // println!("{:?}", &sess);
                    return Some(sess);
                } else {
                    None
                }
            }
            _ => None,
        }
    })
}

pub async fn find_user_passowrd_by_mail(
    client: &Client,
    email: String,
) -> Result<UserPw, ServiceError> {
    let statement = client
        .prepare("SELECT id, hashed_password FROM public.users WHERE email = $1")
        .await
        .unwrap();

    let maybe_user = client
        .query_opt(&statement, &[&email])
        .await
        .expect("Cannt select user password")
        .map(|row| UserPw::from_row_ref(&row).unwrap());

    match maybe_user {
        Some(user) => Ok(user),
        None => Err(ServiceError::NotFound("User Pw not found".to_string())),
    }
}

pub async fn find_user_mail_by_id(client: &Client, id: i32) -> Result<UserMail, ServiceError> {
    let statement = client
        .prepare("SELECT email FROM public.users WHERE id = $1")
        .await
        .map_err(|e| {
            ServiceError::DatabaseError(format!("Error preparing email statement: {}", e))
        })?;

    let maybe_user = client
        .query_opt(&statement, &[&id])
        .await
        .map_err(|e| ServiceError::DatabaseError(format!("Error fetching mail: {}", e)))?
        .map(|row| UserMail::from_row_ref(&row).unwrap());

    match maybe_user {
        Some(user) => Ok(user),
        None => Err(ServiceError::BadId),
    }
}

pub async fn session_otp_update_true(client: &Client, id: i32) -> Result<(), ServiceError> {
    let statement = client
        .prepare("update public.sessions set otp_code_sent = true where id = $1")
        .await
        .map_err(|e| ServiceError::DatabaseError(format!("Error preparing statement: {}", e)))?;

    let result = client
        .execute(&statement, &[&id])
        .await
        .map_err(|e| ServiceError::DatabaseError(format!("Error updating session: {}", e)))?;

    match result {
        ref updated if *updated == 1 => Ok(()),
        _ => Err(ServiceError::DatabaseError(
            "Failed to update session".to_string(),
        )),
    }
}

pub async fn session_otp_update_confirm_true(client: &Client, id: i32) -> Result<(), ServiceError> {
    let statement = client
        .prepare(
            "update public.sessions SET otp_code_confirmed = true AND otp_code_attempts = 0 WHERE id = $1",
        )
        .await
        .unwrap();

    let result = client
        .execute(&statement, &[&id])
        .await
        .expect("Error getting todo lists");

    match result {
        ref updated if *updated == 1 => Ok(()),

        _ => Err(ServiceError::DatabaseError(
            "Session Confirmation not Updated".to_string(),
        )),
    }
}

pub async fn session_otp_set_attempts(client: &Client, id: i32) -> Result<(), ServiceError> {
    let statement = client
        .prepare(
            "update public.sessions SET otp_code_attempts = otp_code_attempts + 1 where id = $1",
        )
        .await
        .unwrap();

    let result = client
        .execute(&statement, &[&id])
        .await
        .expect("Error getting todo lists");

    match result {
        ref updated if *updated == 1 => Ok(()),

        _ => Err(ServiceError::DatabaseError(
            "Code Attempt Update Failed".to_string(),
        )),
    }
}

pub async fn user_reset_password_update(
    client: &Client,
    email: UserMail,
    params: Params,
) -> Result<(), ServiceError> {
    let statement = client
        .prepare("UPDATE public.users SET reset_password_selector = $1, reset_password_validator_hash = $2, reset_password_sent_at = now() WHERE email = $3",)
        .await
        .unwrap();

    let result = client
        .execute(
            &statement,
            &[
                &params.reset_password_selector,
                &params.reset_password_validator,
                &email.email,
            ],
        )
        .await
        .expect("Error updating password");

    match result {
        ref updated if *updated == 1 => Ok(()),

        _ => Err(ServiceError::DatabaseError(
            "Password Reset Update Attempt Failed".to_string(),
        )),
    }
}

pub async fn find_user_password_validated_hash(
    client: &Client,
    pw_selector: String,
) -> Result<UserValidateHash, ServiceError> {
    let statement = client
        .prepare("SELECT id, reset_password_validator_hash FROM public.users WHERE reset_password_selector = $1",)
        .await
        .map_err(|e| ServiceError::DatabaseError(format!("Error preparing password reset statement: {}", e)))?;

    let maybe_user = client
        .query_opt(&statement, &[&pw_selector])
        .await
        .map_err(|e| ServiceError::DatabaseError(format!("Error fetching user pass hash: {}", e)))?
        .map(|row| UserValidateHash::from_row_ref(&row).unwrap());

    match maybe_user {
        Some(user) => Ok(user),
        None => Err(ServiceError::BadId),
    }
}

pub async fn user_update_password_hash(
    client: &Client,
    usr_pw: UserPw,
) -> Result<(), ServiceError> {
    let statement = client
        .prepare(
            "UPDATE public.users SET hashed_password = $1, reset_password_selector = NULL, reset_password_validator_hash = NULL
                    WHERE id = $2",
        )
        .await
        .unwrap();

    let result = client
        .execute(&statement, &[&usr_pw.id, &usr_pw.hashed_password])
        .await
        .expect("Error updating hashed password ");

    match result {
        ref updated if *updated == 1 => Ok(()),

        _ => Err(ServiceError::DatabaseError(
            "Hash Update Attempt Failed".to_string(),
        )),
    }
}
