use color_eyre::eyre::Result;

mod cli;
mod config;
mod daemon;
mod game;
mod rawg;
mod smtp;
mod tui;

fn main() -> Result<()> {
    color_eyre::install()?;
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    cli::init()
}
