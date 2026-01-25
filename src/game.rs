use std::fmt::Display;

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

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let game_name = &self.name;
        let release_date = self.release.as_deref().unwrap_or("N/A");
        let image = self.image.as_deref().unwrap_or("");
        let platforms = self.platforms.join(", ");
        let description = self
            .description
            .as_deref()
            .unwrap_or("No description found");

        write!(
            f,
            r#"<html>
  <body style="font-family: Arial, sans-serif; line-height:1.5;">
    <h2>{game_name} ({release_date})</h2>
    <img src="{image}" alt="{game_name}" style="max-width:600px; width:100%; height:auto; display:block; margin-bottom:15px;">
    <h3>Platforms: <span style="font-weight: normal; font-size: 0.8em;">{platforms}</span></h3>
    <h3>Description</h3>
    <p>{description}</p>
    <p style="font-size:0.8em; color:#555;">Data provided by <a href="https://rawg.io" target="_blank" style="color:#555; text-decoration:underline;">RAWG.io</a></p>
  </body>
</html>"#
        )
    }
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
