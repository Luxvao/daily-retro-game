use std::{str::FromStr, thread};

use chrono::Local;
use color_eyre::eyre::Result;
use cron::Schedule;

use crate::{config::Config, game::Game, smtp::email_game};

pub fn init(config: &Config) -> Result<()> {
    let schedule = Schedule::from_str(&config.cron)?;

    for event in schedule.upcoming(Local) {
        let now = Local::now();
        let wait = event.signed_duration_since(now).to_std()?;
        thread::sleep(wait);

        let game = Game::random(config)?;

        email_game(config, &game)?;
    }

    Ok(())
}
