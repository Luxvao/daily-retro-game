FROM docker.io/alpine

# Install tools
RUN apk add msmtp curl busybox-cron jq ca-certificates

COPY ./feed.sh /feed.sh

RUN echo "30 7 * * * /feed.sh >> /var/log/feed.log 2>&1" > /etc/crontabs/root

RUN touch /var/log/feed.log

CMD ["crond", "-f", "-d", "8"]
