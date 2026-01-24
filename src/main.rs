use color_eyre::eyre::Result;

use crate::{config::Config, rawg::get_page};

mod cli;
mod config;
mod daemon;
mod game;
mod rawg;
mod tui;

fn main() -> Result<()> {
    color_eyre::install()?;
    // cli::init()

    let config = Config {
        src_email: String::new(),
        src_email_auth: String::new(),
        smtp_server: String::new(),
        smtp_port: String::new(),
        dest_email: String::new(),
        rawg_key: String::from(""),
        platforms: vec![],
        genres: vec![],
        cron: String::new(),
    };

    println!("{:?}", get_page(&config, 1));

    Ok(())
}
