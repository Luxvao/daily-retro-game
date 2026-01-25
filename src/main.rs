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
    cli::init()
}
