use crate::components::forms;
use crate::config;
use crate::custom_error::CustomError;
use crate::layouts;
use actix_identity::Identity;
use actix_web::{http, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::borrow::Cow;
use std::default::Default;
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Serialize, Validate, Deserialize, Default)]
pub struct Registration {
    #[validate(email(message = "Email is not valid"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password is too short"))]
    pub password: String,
    #[validate(must_match(other = "password", message = "The passwords don't match"))]
    pub confirm_password: String,
    #[serde(rename = "h-captcha-response")]
    pub h_captcha_response: Option<String>,
}

pub async fn registration(config: web::Data<config::Config>) -> Result<HttpResponse> {
    let body = RegistrationPage {
        form: &Registration::default(),
        hcaptcha_config: &config.hcaptcha_config,
        errors: &ValidationErrors::default(),
    };

    Ok(layouts::session_layout(
        "Registration",
        &body.to_string(),
        config.hcaptcha_config.is_some(),
    ))
}

pub async fn process_registration(
    pool: web::Data<PgPool>,
    config: web::Data<config::Config>,
    form: web::Form<Registration>,
    identity: Identity,
) -> Result<HttpResponse, CustomError> {
    let registration = Registration {
        email: form.email.clone(),
        ..Default::default()
    };

    let valid = super::verify_hcaptcha(&config.hcaptcha_config, &form.h_captcha_response).await;

    if valid {
        match form.validate() {
            Ok(_) => {
                let hashed_password = crate::encryption::password_hash(
                    &form.password,
                    config.use_bcrypt_instead_of_argon,
                )
                .await?;

                let registered_user = sqlx::query_as::<_, InsertedUser>(&format!(
                    "
                        INSERT INTO {} (email, hashed_password)
                        VALUES($1, $2) RETURNING id
                    ",
                    config.user_table_name
                ))
                .bind(&form.email.to_lowercase())
                .bind(hashed_password)
                .fetch_one(pool.get_ref())
                .await;

                if let Ok(registered_user) = registered_user {
                    super::login::create_session(&config, pool, identity, registered_user.id, None)
                        .await?;
                } else {
                    // Looks like the user already exists.
                    // create a fake session, we have to make it look like everything is normal
                    // do defend against account enumeration attacks.
                    super::login::create_session(
                        &config,
                        pool,
                        identity,
                        crate::auth::email_otp::INVALID_USER_ID,
                        None,
                    )
                    .await?;
                }

                return Ok(HttpResponse::SeeOther()
                    .append_header((http::header::LOCATION, config.redirect_url.clone()))
                    .finish());
            }
            Err(validation_errors) => {
                let body = RegistrationPage {
                    form: &registration,
                    hcaptcha_config: &config.hcaptcha_config,
                    errors: &validation_errors,
                };

                Ok(layouts::session_layout(
                    "Registration",
                    &body.to_string(),
                    config.hcaptcha_config.is_some(),
                ))
            }
        }
    } else {
        // The captcha failed

        let mut validation_errors = ValidationErrors::default();

        validation_errors.add(
            "confirm_password",
            ValidationError {
                message: Some(Cow::from("Invalid hCaptcha")),
                code: Cow::from("0"),
                params: Default::default(),
            },
        );

        let body = RegistrationPage {
            form: &registration,
            hcaptcha_config: &config.hcaptcha_config,
            errors: &validation_errors,
        };

        Ok(layouts::session_layout(
            "Registration",
            &body.to_string(),
            config.hcaptcha_config.is_some(),
        ))
    }
}
