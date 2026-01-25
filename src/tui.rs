use color_eyre::eyre::Result;
use dialoguer::{Input, MultiSelect, Select, theme::ColorfulTheme};

use crate::{
    config::Config,
    rawg::{get_genres, get_platforms},
};

pub fn configure_new() -> Result<Config> {
    configure_existent(Config::default())
}

pub fn configure_existent(mut config: Config) -> Result<Config> {
    let options = vec![
        "Edit Genres",
        "Edit Platforms",
        "Edit RAWG API Key",
        "Edit SMTP Settings",
        "Edit Cron Expression",
    ];

    loop {
        let Some(selection) = Select::with_theme(&ColorfulTheme::default())
            .items(&options)
            .default(0)
            .interact_opt()?
        else {
            break;
        };

        match options[selection] {
            "Edit Genres" => genre_options(&mut config)?,
            "Edit Platforms" => platform_options(&mut config)?,
            "Edit RAWG API Key" => {
                config.rawg_key = input_wrapper("RAWG Key", config.rawg_key.clone())?
            }
            "Edit SMTP Settings" => smtp_settings(&mut config)?,
            "Edit Cron Expression" => {
                config.cron = input_wrapper("Cron Expression", config.cron.clone())?
            }
            _ => unreachable!(),
        }
    }

    Ok(config)
}

fn genre_options(config: &mut Config) -> Result<()> {
    // Get all available genres
    let available_genres = get_genres(config)?;

    // Extract the IDs
    let ids = available_genres
        .iter()
        .map(|genre| genre.id)
        .collect::<Vec<u8>>();

    // Extract the names
    let names = available_genres
        .iter()
        .map(|genre| genre.name.as_str())
        .collect::<Vec<&str>>();

    // Figure out which are enabled already and turn it into a Vec<bool> (for defaults in the selection)
    let already_enabled = ids
        .iter()
        .map(|id| config.genres.contains(id))
        .collect::<Vec<bool>>();

    // Show the selection menu
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .items(&names)
        .defaults(&already_enabled)
        .interact()?;

    // Construct the ID vector from the selection indices
    config.genres = selections
        .iter()
        .map(|idx| available_genres[*idx].id)
        .collect::<Vec<u8>>();

    Ok(())
}

fn platform_options(config: &mut Config) -> Result<()> {
    // Get all available platforms
    let available_platforms = get_platforms(config)?;

    // Extract the IDs
    let ids = available_platforms
        .iter()
        .map(|platform| platform.id)
        .collect::<Vec<u8>>();

    // Extract the names
    let names = available_platforms
        .iter()
        .map(|genre| genre.name.as_str())
        .collect::<Vec<&str>>();

    // Figure out which are enabled already and turn it into a Vec<bool> (for defaults in the selection)
    let already_enabled = ids
        .iter()
        .map(|id| config.platforms.contains(id))
        .collect::<Vec<bool>>();

    // Show the selection menu
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .items(&names)
        .defaults(&already_enabled)
        .interact()?;

    // Construct the ID vector from the selection indices
    config.platforms = selections
        .iter()
        .map(|idx| available_platforms[*idx].id)
        .collect::<Vec<u8>>();

    Ok(())
}

fn input_wrapper(prompt: &str, default: String) -> Result<String> {
    Ok(Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .with_initial_text(default)
        .interact_text()?)
}

fn smtp_settings(config: &mut Config) -> Result<()> {
    let options = vec![
        "Edit Source Email",
        "Edit Source Email Authentication",
        "Edit Destination Email",
        "Edit SMTP Server",
    ];

    loop {
        let Some(selection) = Select::with_theme(&ColorfulTheme::default())
            .items(&options)
            .default(0)
            .interact_opt()?
        else {
            return Ok(());
        };

        match options[selection] {
            "Edit Source Email" => {
                config.src_email = input_wrapper("Source Email", config.src_email.clone())?
            }
            "Edit Source Email Authentication" => {
                config.src_email_auth =
                    input_wrapper("Source Email Authentication", config.src_email_auth.clone())?
            }
            "Edit Destination Email" => {
                config.dest_email = input_wrapper("Destination Email", config.dest_email.clone())?
            }
            "Edit SMTP Server" => {
                config.smtp_server = input_wrapper("SMTP Server", config.smtp_server.clone())?
            }
            _ => unreachable!(),
        }
    }
}
