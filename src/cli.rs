use std::{
    fs::File,
    io::{Read, Seek, Write},
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
use color_eyre::Result;

use crate::{
    config::Config,
    tui::{configure_existent, configure_new},
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Sends a random retro game in accordance with the configuration file")]
    Send { config: PathBuf },
    #[command(about = "Starts the daemon")]
    Daemon { config: PathBuf },
    #[command(about = "Configure the program")]
    Configure { config: Option<PathBuf> },
}

pub fn init() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Send { config } => send(&config),
        Commands::Daemon { config } => daemon(&config),
        Commands::Configure { config } => configure(config),
    }
}

fn send(config: &Path) -> Result<()> {
    Ok(())
}

fn daemon(config: &Path) -> Result<()> {
    Ok(())
}

fn configure(config: Option<PathBuf>) -> Result<()> {
    let config_path = config.unwrap_or(PathBuf::from("config.toml"));

    let (mut config_file, config): (File, Config) = if !config_path.try_exists()? {
        (File::create(config_path)?, configure_new()?)
    } else {
        let mut file = File::open(config_path)?;

        let mut contents = Vec::new();

        file.read_to_end(&mut contents)?;

        (file, configure_existent(toml::from_slice(&contents)?)?)
    };

    let config_serialised = toml::to_string_pretty(&config)?;

    config_file.set_len(0)?;
    config_file.seek(std::io::SeekFrom::Start(0))?;

    config_file.write_all(config_serialised.as_bytes())?;

    Ok(())
}
