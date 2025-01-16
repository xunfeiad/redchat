use anyhow::Context;
use error::Result;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::sync::LazyLock;

static SMTP_USERNAME: LazyLock<String> =
    LazyLock::new(|| std::env::var("smtp_username").expect("Get env `stmp_username` failed."));

static SMTP_PASSWORD: LazyLock<String> =
    LazyLock::new(|| std::env::var("smtp_password").expect("Get env `smtp_password` failed."));

static EMAIL: LazyLock<String> =
    LazyLock::new(|| std::env::var("email").expect("Get env `email` failed."));

static SMTP_SERVER: LazyLock<String> =
    LazyLock::new(|| std::env::var("smtp_server").expect("Get env `smtp_server` failed."));

pub trait SendEmail {
    fn send_email(&self, subject: &str, to: String) -> Result<()>;
}

impl SendEmail for String {
    fn send_email(&self, subject: &str, to: String) -> Result<()> {
        let email = Message::builder()
            .from(EMAIL.to_string().parse().context("parse email failed")?)
            .to(to.parse().context("parse email failed")?)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(self.to_string())
            .context("build email failed")?;
        let creds = Credentials::new(SMTP_USERNAME.to_string(), SMTP_PASSWORD.to_string());

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay(&SMTP_SERVER)
            .context("relay smtp server failed")?
            .credentials(creds)
            .build();
        mailer.send(&email).context("send email failed")?;
        Ok(())
    }
}

#[test]
fn test_send_email() {
    dotenv::dotenv().ok();
    let email = "xfhd520@126.com".to_string();
    email.send_email("test", "test".to_string()).unwrap();
}
