ARG ALPINE_VERSION=3.17.1
FROM alpine:${ALPINE_VERSION}
LABEL maintainer="Björn Busse <bj.rn@baerlin.eu>"

ENV APK_ADD="git curl xz" \
    USER="git"

# Add packages
RUN apk update \
    && apk upgrade \
    && apk add --no-cache $APK_ADD \
    && addgroup -S $USER && adduser -S -G $USER $USER \
    && mkdir -p /mnt \
    && chown $USER /mnt \
    && curl -O --output-dir /tmp https://github.com/bbusse/git-find-uncommited/releases/download/linux-amd64-latest/git-find-uncommited-linux-amd64.xz \
    && xz -d /tmp/git-find-uncommited-linux-amd64.xz \
    && mv /tmp/git-find-uncommited-linux /usr/bin

# Add entrypoint
USER $USER
ENTRYPOINT ["/entrypoint.sh"]
