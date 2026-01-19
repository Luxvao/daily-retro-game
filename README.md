# Daily Retro Game
A small project in the form of a podman/docker container that retrieves a random retro game every day, formats it and sends it to an email address. (I use it with Kill the Newsletter!)

## Requirements
- `podman`
- `jq`
- `curl`
- `fzf`
- `sed`
- `RAWG account and API key`
- `An SMTP provider. Gmail is fine`
- `Recommended: Nerd Font` - A nerd font is required for the configuration script

## Usage
1. First generate configuration files with `./config_gen.sh`. It's an interactive wizard.
2. You may also change when the game is delivered. The default is at 7:30 AM and you can change it in `./Containerfile`
3. Then simply run `./create_container.sh` and it will build and start a container
