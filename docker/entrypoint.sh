#!/bin/sh
# Startup-time .env injection for the static site.
#
# Runs from /docker-entrypoint.d/ (nginx:alpine executes every *.sh there in
# order before launching nginx). Reads FOOTER_EXTRA from the container
# environment — docker compose loads it from .env via env_file: — and
# substitutes the $FOOTER_EXTRA placeholder inside the built index.html.
#
# The envsubst whitelist keeps every other "$..." in the page untouched, and
# an unset variable leaves the placeholder in place (the app hides it).
set -e

index=/usr/share/nginx/html/index.html

if [ -n "${FOOTER_EXTRA:-}" ]; then
  envsubst '$FOOTER_EXTRA' <"$index" >"$index.tmp"
  mv "$index.tmp" "$index"
fi
