#!/bin/sh
set -o errexit

case "$1" in
    sh|bash)
        set -- "$@"
    ;;
    *)
        set -- /usr/bin/git-find-uncommited /mnt
    ;;
esac

exec "$@"
