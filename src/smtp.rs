use color_eyre::eyre::Result;
use lettre::{
    Message, SmtpTransport, Transport, message::header::ContentType,
    transport::smtp::authentication::Credentials,
};

use crate::{config::Config, game::Game};

pub fn email_game(config: &Config, game: &Game) -> Result<()> {
    let email = Message::builder()
        .from(config.src_email.parse()?)
        .to(config.dest_email.parse()?)
        .header(ContentType::TEXT_HTML)
        .subject(format!(
            "{} - {}",
            game.name,
            game.release.as_deref().unwrap_or("N/A")
        ))
        .body(format!("{}", game))?;

    let creds = Credentials::new(config.src_email.clone(), config.src_email_auth.clone());

    let mailer = SmtpTransport::relay(&config.smtp_server)?
        .credentials(creds)
        .build();

    mailer.send(&email)?;

    Ok(())
}
