# Daily Retro Games
This project provides a daemon to schedule (uses cron-like expressions; see [Cron](#cron)) deliveries of random retro games to an email inbox (I use it with [Kill the Newsletter!](https://kill-the-newsletter.com/); can recommended, they're awesome). It uses cron-like syntax to schedule deliveries and RAWG to pick out a random game.

## Requirements
- An SMTP provider and credentials (Gmail is fine; see [Gmail](#gmail))
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

### Important
Use the app password for the "Source Email Authentication" option in the settings *NOT YOUR EMAIL PASSWORD*

## Cron
Due to how the [cron](https://docs.rs/cron/latest/cron/) crate works it doesn't use standard cron syntax. Example from documentation:
```
//               sec  min   hour   day of month   month   day of week   year
let expression = "0   30   9,12,15     1,15       May-Aug  Mon,Wed,Fri  2018/2";
```

This is what I use so it sends daily at 7:30 AM:
```
0 30 7 * * *
```
That means at 7:30:00 on *any* day, month, and year.

You could also do every 24 hours from when you launch it as such:
```
0 0 */24 * * *
```
