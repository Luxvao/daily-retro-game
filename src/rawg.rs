use color_eyre::{Result, eyre::ContextCompat};

use crate::{config::Config, game::Game};

pub struct Genre {
    pub id: u8,
    pub name: String,
}

pub struct Platform {
    pub id: u8,
    pub name: String,
}

pub fn get_page(config: &Config, page_n: u32) -> Result<(u32, Vec<Game>)> {
    let resp = reqwest::blocking::get(format!(
        "https://api.rawg.io/api/games?key={}&genres={}&platforms={}&page={}&page_size=40",
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

    for game in json_parsed["results"].members() {
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
            id: game["id"].as_u32().context("Game ID is not an integer")?,
            release: game["released"]
                .as_str()
                .map(|released| released.to_string()),
            image: game["background_image"]
                .as_str()
                .map(|image| image.to_string()),
            platforms: platforms,
            description: None,
        };

        games.push(game_struct);
    }

    Ok((
        json_parsed["count"]
            .as_u32()
            .context("Count of games is not an integer")?,
        games,
    ))
}

pub fn get_description(config: &Config, id: u32) -> Result<Option<String>> {
    let resp = reqwest::blocking::get(format!(
        "https://api.rawg.io/api/games/{}?key={}",
        id, config.rawg_key
    ))?;

    let json_parsed = json::parse(resp.text()?.as_str())?;

    Ok(json_parsed["description"]
        .as_str()
        .map(|desc| desc.to_string()))
}

pub fn get_genres(config: &Config) -> Result<Vec<Genre>> {
    let resp = reqwest::blocking::get(format!(
        "https://api.rawg.io/api/genres?key={}",
        config.rawg_key
    ))?;

    let json_parsed = json::parse(resp.text()?.as_str())?;

    let genres = json_parsed["results"]
        .members()
        .map(|genre| {
            Ok(Genre {
                id: genre["id"].as_u8().context("ID is not an integer")?,
                name: genre["name"]
                    .as_str()
                    .context("Name is not a string")
                    .map(|name| name.to_string())?,
            })
        })
        .collect::<Result<Vec<Genre>>>()?;

    Ok(genres)
}

pub fn get_platforms(config: &Config) -> Result<Vec<Platform>> {
    let resp = reqwest::blocking::get(format!(
        "https://api.rawg.io/api/platforms?key={}",
        config.rawg_key
    ))?;

    let json_parsed = json::parse(resp.text()?.as_str())?;

    let platforms = json_parsed["results"]
        .members()
        .map(|platform| {
            Ok(Platform {
                id: platform["id"].as_u8().context("ID is not an integer")?,
                name: platform["name"]
                    .as_str()
                    .context("Name is not a string")
                    .map(|name| name.to_string())?,
            })
        })
        .collect::<Result<Vec<Platform>>>()?;

    Ok(platforms)
}
