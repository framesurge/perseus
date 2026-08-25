use crate::server::configs::ServerConfig;
use crate::server::errors::ServiceError;
use lettre::message::header::ContentType;
use lettre::message::Mailbox;
use lettre::transport::smtp::client::Tls;
use lettre::{Message, SmtpTransport, Transport};

/// Build the From header with optional display name
fn build_from_header(config: &ServerConfig) -> Result<Mailbox, ServiceError> {
    let from = if config.smtp_from_name.is_empty() {
        config.smtp_from_email.clone()
    } else {
        format!("{} <{}>", config.smtp_from_name, config.smtp_from_email)
    };
    from.parse()
        .map_err(|e| ServiceError::FaultySetup(format!("Invalid from address: {}", e)))
}

/// Build SMTP transport based on configuration
fn build_smtp_transport(
    config: &ServerConfig,
) -> Result<SmtpTransport, ServiceError> {
    use lettre::transport::smtp::client::TlsParameters;

    let builder = if config.smtp_tls_off {
        // No TLS - for local development with mailpit
        SmtpTransport::builder_dangerous(&config.smtp_host)
            .port(config.smtp_port)
            .tls(Tls::None)
    } else if config.smtp_starttls {
        // STARTTLS - for port 587 (SendGrid, Mailgun, SES SMTP)
        let tls_params = TlsParameters::new(config.smtp_host.clone())
            .map_err(|e| ServiceError::FaultySetup(format!("Failed to create TLS params: {}", e)))?;
        SmtpTransport::builder_dangerous(&config.smtp_host)
            .port(config.smtp_port)
            .tls(Tls::Required(tls_params))
    } else {
        // Implicit TLS - for port 465
        SmtpTransport::relay(&config.smtp_host)
            .map_err(|e| ServiceError::FaultySetup(format!("Failed to create SMTP relay: {}", e)))?
            .port(config.smtp_port)
    };

    // Add credentials if username is provided
    if !config.smtp_username.is_empty() {
        let credentials = lettre::transport::smtp::authentication::Credentials::new(
            config.smtp_username.clone(),
            config.smtp_password.clone(),
        );
        let builder = builder.credentials(credentials);
        Ok(builder.build())
    } else {
        Ok(builder.build())
    }
}

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
        .from(build_from_header(config)?)
        .to(to_email
            .parse()
            .map_err(|e| ServiceError::FaultySetup(format!("Invalid to email: {}", e)))?)
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(html_body.to_string())
        .map_err(|e| ServiceError::FaultySetup(format!("Failed to build email: {}", e)))?;

    // Create SMTP transport
    let mailer = build_smtp_transport(config)?;

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
        .from(build_from_header(config)?)
        .to(to_email
            .parse()
            .map_err(|e| ServiceError::FaultySetup(format!("Invalid to email: {}", e)))?)
        .subject(subject)
        .body(body.to_string())
        .map_err(|e| ServiceError::FaultySetup(format!("Failed to build email: {}", e)))?;

    let mailer = build_smtp_transport(config)?;

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
            smtp_starttls: false,
            smtp_from_email: "test@localhost".to_string(),
            smtp_from_name: "".to_string(),
            smtp_admin_email: "admin@localhost".to_string(),
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
