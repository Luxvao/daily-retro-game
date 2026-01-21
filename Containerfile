FROM docker.io/alpine

# Install tools
RUN apk add --no-cache msmtp curl cronie jq ca-certificates bash

COPY ./feed.sh /feed.sh

RUN chmod +x /feed.sh

RUN echo "30 7 * * * /feed.sh >> /var/log/feed.log 2>&1" > /etc/crontabs/root

RUN touch /var/log/feed.log

CMD ["crond", "-f"]
