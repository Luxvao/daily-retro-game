use color_eyre::eyre::Result;
use rand::Rng;

use crate::{
    config::Config,
    rawg::{get_description, get_page},
};

#[derive(Clone, Debug)]
pub struct Game {
    pub name: String,
    pub id: u32,
    pub release: Option<String>,
    pub image: Option<String>,
    pub platforms: Vec<String>,
    pub description: Option<String>,
}

impl Game {
    pub fn random(config: &Config) -> Result<Game> {
        let mut rng = rand::rng();

        let (total_games, _) = get_page(config, 1)?;

        let total_pages = (total_games + 39) / 40 + 1;

        let random_page = rng.random_range(1..total_pages);

        let (_, games) = get_page(config, random_page)?;

        let random_index = rng.random_range(0..games.len());

        let mut game = games[random_index].clone();

        game.description = get_description(config, game.id)?;

        Ok(game)
    }
}
