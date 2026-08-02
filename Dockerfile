# Multi-stage: build the SPA, then serve it with nginx.
# Env vars (FOOTER_EXTRA, ...) are injected into index.html at container
# start by docker/entrypoint.sh, so changing .env never requires a rebuild.
FROM node:22-slim AS build

WORKDIR /app

RUN corepack enable && corepack prepare pnpm@10 --activate
# scripts/build.py generates the favicons with Pillow.
RUN apt-get update \
    && apt-get install -y --no-install-recommends python3 python3-pip \
    && pip3 install --break-system-packages Pillow \
    && rm -rf /var/lib/apt/lists/*

COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile --ignore-scripts

COPY . .
RUN pnpm build

FROM nginx:alpine

LABEL org.opencontainers.image.source="https://github.com/celestia-island/celestia-island.github.io"
LABEL org.opencontainers.image.title="Celestia Island Homepage"
LABEL org.opencontainers.image.description="Official website of Celestia Island"

# gettext provides envsubst for the startup-time .env injection.
RUN apk add --no-cache gettext

COPY --from=build /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY docker/entrypoint.sh /docker-entrypoint.d/40-inject-env.sh
RUN chmod +x /docker-entrypoint.d/40-inject-env.sh

EXPOSE 80

HEALTHCHECK --interval=30s --timeout=3s --retries=3 \
  CMD wget -qO- http://localhost/ || exit 1
