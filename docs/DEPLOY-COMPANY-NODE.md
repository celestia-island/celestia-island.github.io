# Deploying to the company website node

The production site (`celestia.world`) is served from the company website node
(Alibaba Cloud Linux, `106.14.124.153`) behind Cloudflare. This page documents
the deployment pipeline.

## Architecture

```
Browser ──► Cloudflare edge (TLS, HTTP→HTTPS redirect)
                │  (CF IP ranges only — origin refuses everything else)
                ▼
            nginx 1.30.x (systemd)  ──► 127.0.0.1:8080
                                             │
                                      docker container
                                      (celestia-island-home)
```

- The Docker image is built on GitHub Actions and pushed to the company Aliyun
  ACR registry (not GHCR — GHCR is unreliable from mainland China).
- The origin only accepts connections from Cloudflare edge IP ranges
  (`/etc/nginx/cloudflare/celestia-allow-cf.conf`), so direct scanning of the
  origin IP returns 403.
- The ICP filing line is injected at container start: `FOOTER_EXTRA` is read
  from `.env` (`env_file:`) and substituted into `index.html` by
  `docker/entrypoint.sh` (runs via nginx's `/docker-entrypoint.d/`). Editing
  `.env` and restarting the container is enough — no rebuild.

## Image pipeline

`.github/workflows/docker.yml`:

- On `push` to `main`: builds `linux/amd64` + `linux/arm64` and pushes to
  `crpi-88d7shkt0yo9qvvt.cn-shanghai.personal.cr.aliyuncs.com/langyo_personal/celestia-island.github.io`
  with tags `latest`, `sha-<hash>`, `v<semver>` (release tags).
- On PRs: builds only (no push).
- Credentials: repo secrets `ACR_USERNAME` / `ACR_PASSWORD`.

## Deploying on the company node

Everything lives in `/root/celestia-world/`:

| File | Purpose |
| --- | --- |
| `docker-compose.yml` | `web` service, `127.0.0.1:8080:80`, `env_file: .env`, healthcheck, resource limits |
| `.env` | `FOOTER_EXTRA` JSON (e.g. `[{"title":"苏ICP备2025155733号-2","url":"https://beian.miit.gov.cn/"}]`) |
| `deploy.sh` | pull image → compose down → up → status (the update routine) |
| `setup-tls.sh` | issue/renew the Let's Encrypt cert via certbot DNS-01 (Cloudflare API) |
| `nginx/celestia.443.conf` | HTTPS vhost (activate after cert is issued) |

Steps:

```bash
# update (after a new image is pushed)
cd /root/celestia-world && ./deploy.sh

# first-time TLS (requires a Cloudflare API token with Zone→DNS:Edit for celestia.world)
printf 'dns_cloudflare_api_token = %s\n' '<TOKEN>' > /root/.secrets/cloudflare.ini
chmod 600 /root/.secrets/cloudflare.ini
./setup-tls.sh
cp nginx/celestia.443.conf /etc/nginx/conf.d/celestia.world.443.conf
systemctl reload nginx
```

TLS renewal is automatic: `certbot-renew.timer` runs certbot, and
`/etc/letsencrypt/renewal-hooks/deploy/nginx-reload.sh` reloads nginx after
each successful renewal.

## DNS

`celestia.world` (and `www`) point at Cloudflare (orange-cloud proxy mode).
The CF zone handles TLS termination and HTTP→HTTPS redirects; the origin is
reached over HTTP on port 80 (CF SSL mode: Flexible is sufficient since the
origin is CF-IP-restricted, Full works too once the 443 vhost is active).

## Troubleshooting

- `docker inspect celestia-island-home --format '{{.State.Health.Status}}'` —
  the healthcheck is `wget http://127.0.0.1/` (IPv4 explicitly; busybox wget
  resolves `localhost` to `::1` which the container's nginx does not listen on).
- CF errors: `526` means CF tried HTTPS origin fetch without a cert (run
  `setup-tls.sh`); `522` means the origin refused the connection (check nginx
  and the container).
- Origin access control: `curl -H "Host: celestia.world" http://127.0.0.1/`
  returns 200; the same request to the public IP returns 403.
