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

- The Docker image is built on GitHub Actions, pushed to the company Aliyun ACR
  registry (the copy this node pulls) and then mirrored to GHCR.
- The origin only accepts connections from Cloudflare edge IP ranges
  (`/etc/nginx/cloudflare/celestia-allow-cf.conf`), so direct scanning of the
  origin IP returns 403.
- The ICP filing line is injected at container start: `FOOTER_EXTRA` is read
  from `.env` (`env_file:`) and substituted into `index.html` by
  `docker/entrypoint.sh` (runs via nginx's `/docker-entrypoint.d/`). Editing
  `.env` and restarting the container is enough — no rebuild.

## Image pipeline

`.github/workflows/docker.yml`:

- **`build` job** — on `push` to `main` it builds `linux/amd64` + `linux/arm64`
  once and pushes to the company ACR registry: a `main` push publishes `latest`
  and `sha-<short-sha>`, while the `v1.2.3` release tag publishes `1.2.3` and
  moves `latest`. This is the copy the website nodes pull. On PRs it builds only,
  without pushing.
- **`mirror-ghcr` job** — `needs: build`, so it only runs once the ACR push
  succeeded; it copies that same manifest list to
  `ghcr.io/celestia-island/celestia-island.github.io` with
  `docker buildx imagetools create`. No second build, so both copies keep the
  same index digest. It is a separate job on purpose — a GHCR-side failure must
  not be able to stop the ACR publish that production depends on.
- Credentials: repo secrets `ACR_USERNAME` / `ACR_PASSWORD` for ACR; the mirror
  job uses the workflow's own `GITHUB_TOKEN` (`permissions: packages: write`).
- `provenance` stays off: ACR rejects buildx attestation manifests, and GHCR
  receives the same manifest list by copy.

## Deploying on the company node

Everything lives in `/root/celestia-world/`:

| File | Purpose |
| --- | --- |
| `docker-compose.yml` | `web` service, `127.0.0.1:8080:80`, `env_file: .env`, healthcheck, resource limits |
| `.env` | `FOOTER_EXTRA` JSON (e.g. `[{"title":"苏ICP备2025155733号-2","url":"https://beian.miit.gov.cn/"}]`) |
| `deploy.sh` | pull image → compose down → up → status (the update routine) |
| `setup-tls.sh` | issue/renew the Let's Encrypt cert via certbot DNS-01 (Cloudflare API) |
| `nginx/celestia.443.conf` | HTTPS vhost (activate after cert is issued) |

> **A pushed image is not a deployment.** Nothing on this node polls the registry,
> so a merged change stays invisible on `celestia.world` until `deploy.sh` runs
> here. If a UI change looks like it had no effect, this node's container age is
> part of the evidence.

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
