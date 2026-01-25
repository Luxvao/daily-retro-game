use std::{str::FromStr, thread};

use chrono::Local;
use color_eyre::eyre::Result;
use cron::Schedule;
use log::info;

use crate::{config::Config, game::Game, smtp::email_game};

pub fn init(config: &Config) -> Result<()> {
    info!("Daemon started");

    let schedule = Schedule::from_str(&config.cron)?;

    for event in schedule.upcoming(Local) {
        info!("Next retro game at {}", event);

        let now = Local::now();
        let wait = event.signed_duration_since(now).to_std()?;
        info!("Waiting for {:?}", wait);

        thread::sleep(wait);

        let game = Game::random(config)?;
        info!("Sending...");

        email_game(config, &game)?;
        info!("{} sent.", game.name);
    }

    Ok(())
}
