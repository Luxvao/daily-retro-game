#! /bin/bash

# Set up a RAWG API key
if [ ! -e ./RAWG_API_KEY ]; then
  key=$(fzf --prompt="RAWG Key: " --print-query --height=1 --layout=reverse --border --min-height=1 < /dev/null)
  echo "$key" > ./RAWG_API_KEY
fi

API_KEY=$(cat ./RAWG_API_KEY)

# Get the platforms available
curl -s "https://api.rawg.io/api/platforms?key=${API_KEY}&ordering=name" | jq -r '.results[] | "\(.name)|\(.id)"' > platforms.resp

# Initialise the enabled platforms file
touch platforms.enabled

ENABLED_PLATFORMS=$(cat platforms.enabled)

while true; do
  fzf_menu=$(cat platforms.resp | while IFS= read -r line; do
    IFS="|" read -r platform id <<< "$line"

    if [[ "$ENABLED_PLATFORMS" == *",$id,"* ]]; then
      echo "\e[32m\e[0m","$platform","$id"
    else
      echo "\e[31m\e[0m","$platform","$id"
    fi
  done)

  selection=$(printf "${fzf_menu}\n_,Next\n" | fzf --delimiter , --with-nth '   {1}    | {2}' --reverse --style=full --height=~100% --header="Enabled | Platform" --ansi)

  if [[ "$selection" == "_,Next" ]]; then
    echo "$ENABLED_PLATFORMS" > platforms.enabled
    break
  fi

  platform_id=$(echo "$selection" | sed 's/^.*,//')

  if [[ "$ENABLED_PLATFORMS" == *",$platform_id,"* ]]; then
    ENABLED_PLATFORMS="${ENABLED_PLATFORMS/,$platform_id,/}"
  else
    ENABLED_PLATFORMS="$ENABLED_PLATFORMS,$platform_id,"
  fi
done


# Get the genres available
curl -s "https://api.rawg.io/api/genres?key=$(cat ./RAWG_API_KEY)&ordering=name" | jq -r '.results[] | "\(.name)|\(.id)"' > genres.resp

# Initialise the enabled platforms file
touch genres.enabled

ENABLED_GENRES=$(cat genres.enabled)

while true; do
  fzf_menu=$(cat genres.resp | while IFS= read -r line; do
    IFS="|" read -r genre id <<< "$line"

    if [[ "$ENABLED_GENRES" == *",$id,"* ]]; then
      echo "\e[32m\e[0m","$genre","$id"
    else
      echo "\e[31m\e[0m","$genre","$id"
    fi
  done)

  selection=$(printf "${fzf_menu}\n_,Next\n" | fzf --delimiter , --with-nth '   {1}    | {2}' --reverse --style=full --height=~100% --header="Enabled | Genre" --ansi)

  if [[ "$selection" == "_,Next" ]]; then
    echo "$ENABLED_GENRES" > genres.enabled
    break
  fi

  genre_id=$(echo "$selection" | sed 's/^.*,//')

  if [[ "$ENABLED_GENRES" == *",$genre_id,"* ]]; then
    ENABLED_GENRES="${ENABLED_GENRES/,$genre_id,/}"
  else
    ENABLED_GENRES="$ENABLED_GENRES,$genre_id,"
  fi
done

if [ ! -e ./msmtp.conf ]; then
  setup_gmail=$(fzf --prompt="Use gmail? [y/n] " --print-query --height=1 --layout=reverse --border --min-height=1 < /dev/null)

  if [ ! "$setup_gmail" == "y" ]; then
    exit 0
  fi

  gmail_account=$(fzf --prompt="Enter gmail account: " --print-query --height=1 --layout=reverse --border --min-height=1 < /dev/null)
  application_password=$(fzf --prompt="Enter your application password (NOT your gmail password): " --print-query --height=1 --layout=reverse --border --min-height=1 < /dev/null)

  cat <<EOF > ./msmtp.conf
# Default settings
defaults
auth           on
tls            on
tls_trust_file /etc/ssl/certs/ca-certificates.crt
logfile        /.msmtp.log

# Your account
account gmail
host smtp.gmail.com
port 587
from $gmail_account
user $gmail_account
password $application_password

# Set this as the default account
account default : gmail
EOF
  chmod 600 ./msmtp.conf
fi

if [ ! -e ./recipient.conf ]; then
  fzf --prompt="Enter the recipient email address: " --print-query --height=1 --layout=reverse --border --min-height=1 < /dev/null > ./recipient.conf
fi
