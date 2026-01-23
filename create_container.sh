#! /bin/bash

podman build -t drgfeed .

podman run --name drg -d -v "./genres.enabled:/genres.enabled" -v "./platforms.enabled:/platforms.enabled" -v "./RAWG_API_KEY:/RAWG_API_KEY" -v "./recipient.conf:/recipient.conf" -v "./msmtp.conf:/msmtp.conf" -v "/etc/localtime:/etc/localtime:ro" drgfeed
