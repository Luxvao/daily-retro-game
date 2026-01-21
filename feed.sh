#! /bin/bash

PLATFORMS=$(cat /platforms.enabled | sed 's/^,//; s/,$//; s/,,/,/g')
GENRES=$(cat /genres.enabled | sed 's/^,//; s/,$//; s/,,/,/g')
RECIPIENT=$(cat /recipient.conf)
API_KEY=$(cat /RAWG_API_KEY)

# Getting a count of the total games
TOTAL_GAMES=$(curl "https://api.rawg.io/api/games?key=$API_KEY&platforms=$PLATFORMS&genres=$GENRES" | jq -r '.count')

PAGES=$(echo "($TOTAL_GAMES + 39) / 40" | bc)
RANDOM_PAGE=$(( (RANDOM % PAGES) + 1 ))
RANDOM_ENTRY=$(( (RANDOM % 40) + 1 ))

echo "Pulling random game..."
echo "RANDOM_PAGE=$RANDOM_PAGE"
echo "RANDOM_ENTRY=$RANDOM_ENTRY"

GAMES=$(curl "https://api.rawg.io/api/games?key=$API_KEY&platforms=$PLATFORMS&genres=$GENRES&page_size=40&page=$RANDOM_PAGE" | jq -r '.results[] | "\(.id)|\(.name)|\(.released)|\(.background_image)"')

RANDOM_GAME=$(echo "$GAMES" | sed -n "${RANDOM_ENTRY}p")

IFS="|" read -r game_id game_name game_released background_image <<< "$RANDOM_GAME"

echo "Pulling description..."

game=$(curl "https://api.rawg.io/api/games/${game_id}?key=$API_KEY")
game_description=$(echo "$game" | jq -r '.description')
game_platforms=$(echo "$game" | jq -r '.platforms[] | .platform | .name')

EMAIL=$(cat <<EOF
Subject: $game_name - $game_released
MIME-Version: 1.0
Content-Type: text/html; charset=UTF-8

<html>
  <body style="font-family: Arial, sans-serif; line-height:1.5;">
    <h2>$game_name ($game_released)</h2>
    <img src="$background_image" alt="$game_name" style="max-width:600px; width:100%; height:auto; display:block; margin-bottom:15px;">
    <h3>Platforms: <span style="font-weight: normal; font-size: 0.8em;">$game_platforms</span></h3>
    <h3>Description</h3>
    <p>$game_description</p>
    <p style="font-size:0.8em; color:#555;">Data provided by <a href="https://rawg.io" target="_blank" style="color:#555; text-decoration:underline;">RAWG.io</a></p>
  </body>
</html>
EOF
)

echo "Sending..."
echo -e "$EMAIL" | msmtp -C /msmtp.conf "$RECIPIENT"
# echo -e "$EMAIL"
