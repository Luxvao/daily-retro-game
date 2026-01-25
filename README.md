# Daily Retro Games
This project provides a daemon to schedule deliveries of random retro games to an email inbox (I use it with [Kill the Newsletter!](https://kill-the-newsletter.com/); can recommended, they're awesome). It uses cron-like syntax to schedule deliveries and RAWG to pick out a random game.

## Requirements
- An SMTP provider and credentials (Gmail is fine; see [Gmail])
- A RAWG API key
- A destination address (eg. Kill the Newsletter! address)

## Usage
1. Configure it
1. a) Manually - Not recommended, but possible.
1. b) Using `./drg configure [path to config.toml]` - It will show you a UI to set everything up. It also supports editing existing configurations, if one already exists at the OPTIONAL path argument (default is `./config.toml`)
2. Run `./drg daemon (path to config.toml)` in the background - Path argument is NOT OPTIONAL
3. And you're done!

### Note:
There is also the `./drg send (path to config.toml)` command that sends a one-off random game according to the `config.toml`. Useful for testing purposes.

## Gmail
To use Gmail as your SMTP provider you need:
- A Gmail account WITH 2fa enabled
- An "App Password" for the Gmail account

### IMPORTANT
Use the app password for the "Source Email Authentication" option in the settings *NOT YOUR EMAIL PASSWORD*
