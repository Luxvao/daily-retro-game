FROM docker.io/alpine

# Install tools
RUN apk add msmtp curl

COPY ./feed.sh /feed.sh
