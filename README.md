<div align="center">

<img src="res/logos/celestia.webp" alt="Celestia Island" width="200"/>

# Celestia Island

**[celestia.world](https://celestia.world)**

The official website for [Celestia Island](https://github.com/celestia-island), built with Vue 3, Three.js, and UnoCSS.

</div>

## Development

```bash
pnpm install
pnpm dev
```

**Production build:**

```bash
python3 scripts/build.py
```

## Docker deployment

The site ships as a self-contained nginx image for the company website nodes.
Footer content is read from environment variables at **container start**, so
`.env` changes never require a rebuild:

```bash
cp .env.example .env   # set FOOTER_EXTRA (e.g. the ICP filing line)
docker compose up -d --build
```

| Variable          | Meaning                                                                                    |
| ----------------- | ------------------------------------------------------------------------------------------ |
| `FOOTER_EXTRA`    | Extra footer content rendered below the copyright line — one line per entry. Accepts a JSON array of `{ "title", "url" }` objects (each renders as a link, e.g. `[{"title":"ICP filing number","url":"https://beian.miit.gov.cn/"}]`), a JSON array of raw-HTML strings, or a single raw-HTML string. Injected into `index.html` at startup via `docker/entrypoint.sh`; omitted/empty on GitHub Pages renders nothing. |

The image is published to two registries on every `main` push, so nodes can pull
instead of build:

| Registry | Image |
| --- | --- |
| Aliyun ACR (the copy the website nodes pull) | `crpi-88d7shkt0yo9qvvt.cn-shanghai.personal.cr.aliyuncs.com/langyo_personal/celestia-island.github.io` |
| GHCR (GitHub-side copy) | `ghcr.io/celestia-island/celestia-island.github.io` |

GHCR receives the same manifest list as ACR, with the same tags. A `main` push
publishes `latest` and `sha-<short-sha>`; the `v1.2.3` release tag publishes
`1.2.3` and moves `latest`.

```bash
docker pull ghcr.io/celestia-island/celestia-island.github.io:latest
```

That pull needs the package to be public first: GHCR packages do not inherit the
repository's visibility, and this one is private. Switch it under the package
settings, or run `docker login ghcr.io` before pulling.

## License

Licensed under the [Synthetic Source License (SySL), Version 1.0](./LICENSE).
