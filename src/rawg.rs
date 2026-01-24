use color_eyre::{Result, eyre::ContextCompat};

use crate::{config::Config, game::Game};

pub fn get_page(config: &Config, page_n: u32) -> Result<Vec<Game>> {
    let resp = reqwest::blocking::get(format!(
        "https://api.rawg.io/api/games?key={}&genres={}&platforms={}&page={}",
        config.rawg_key,
        config
            .genres
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(","),
        config
            .platforms
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(","),
        page_n
    ))?;

    let json_parsed = json::parse(resp.text()?.as_str())?;

    let mut games = Vec::new();

    for game in json_parsed.members() {
        let platforms = game["platforms"]
            .members()
            .map(|platform| {
                Ok(platform["platform"]["name"]
                    .as_str()
                    .context("Platform name is not a string")?
                    .to_string())
            })
            .collect::<Result<Vec<String>>>()?;

        let game_struct = Game {
            name: game["name"]
                .as_str()
                .context("Game name is not a string")?
                .to_string(),
            release: game["released"]
                .as_str()
                .context("Date of release is not a string")?
                .to_string(),
            image: game["background_image"]
                .as_str()
                .context("Background image URL is not a string")?
                .to_string(),
            platforms: platforms,
            description: None,
        };

        games.push(game_struct);
    }

    Ok(games)
}
