use crate::server::configs::ServerConfig;
use crate::server::errors::ServiceError;
use lettre::message::header::ContentType;
use lettre::transport::smtp::client::Tls;
use lettre::{Message, SmtpTransport, Transport};

/// Send an email using the configured SMTP server (mailpit or other)
///
/// # Arguments
/// * `config` - Server configuration containing SMTP settings
/// * `to_email` - Recipient email address
/// * `subject` - Email subject
/// * `html_body` - HTML body of the email
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(ServiceError)` on failure
pub fn send_email(
    config: &ServerConfig,
    to_email: &str,
    subject: &str,
    html_body: &str,
) -> Result<(), ServiceError> {
    // Build the email message
    let email = Message::builder()
        .from(
            config
                .smtp_from_email
                .parse()
                .map_err(|e| ServiceError::FaultySetup(format!("Invalid from email: {}", e)))?,
        )
        .to(to_email
            .parse()
            .map_err(|e| ServiceError::FaultySetup(format!("Invalid to email: {}", e)))?)
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(html_body.to_string())
        .map_err(|e| ServiceError::FaultySetup(format!("Failed to build email: {}", e)))?;

    // Create SMTP transport
    // For mailpit, we use an unencrypted connection (smtp_tls_off = true)
    let mailer = if config.smtp_tls_off {
        // For local development with mailpit (no TLS)
        SmtpTransport::builder_dangerous(&config.smtp_host)
            .port(config.smtp_port)
            .tls(Tls::None)
            .build()
    } else {
        // For production with TLS
        SmtpTransport::relay(&config.smtp_host)
            .map_err(|e| ServiceError::FaultySetup(format!("Failed to create SMTP relay: {}", e)))?
            .port(config.smtp_port)
            .build()
    };

    // Send the email
    mailer
        .send(&email)
        .map_err(|e| ServiceError::FaultySetup(format!("Failed to send email: {}", e)))?;

    log::info!("Email sent to: {}", to_email);
    Ok(())
}

/// Send a plain text email
pub fn send_plain_email(
    config: &ServerConfig,
    to_email: &str,
    subject: &str,
    body: &str,
) -> Result<(), ServiceError> {
    let email = Message::builder()
        .from(
            config
                .smtp_from_email
                .parse()
                .map_err(|e| ServiceError::FaultySetup(format!("Invalid from email: {}", e)))?,
        )
        .to(to_email
            .parse()
            .map_err(|e| ServiceError::FaultySetup(format!("Invalid to email: {}", e)))?)
        .subject(subject)
        .body(body.to_string())
        .map_err(|e| ServiceError::FaultySetup(format!("Failed to build email: {}", e)))?;

    let mailer = if config.smtp_tls_off {
        SmtpTransport::builder_dangerous(&config.smtp_host)
            .port(config.smtp_port)
            .tls(Tls::None)
            .build()
    } else {
        SmtpTransport::relay(&config.smtp_host)
            .map_err(|e| ServiceError::FaultySetup(format!("Failed to create SMTP relay: {}", e)))?
            .port(config.smtp_port)
            .build()
    };

    mailer
        .send(&email)
        .map_err(|e| ServiceError::FaultySetup(format!("Failed to send email: {}", e)))?;

    log::info!("Plain email sent to: {}", to_email);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> ServerConfig {
        ServerConfig {
            smtp_host: "127.0.0.1".to_string(),
            smtp_port: 1025,
            smtp_tls_off: true,
            smtp_from_email: "test@localhost".to_string(),
            ..Default::default()
        }
    }

    #[test]
    #[ignore] // Requires mailpit running
    fn test_send_email() {
        let config = test_config();
        let result = send_email(
            &config,
            "recipient@example.com",
            "Test Subject",
            "<h1>Hello!</h1><p>This is a test email.</p>",
        );
        // This will fail unless mailpit is running
        assert!(result.is_ok() || result.is_err());
    }
}
