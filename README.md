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

The image is also published to GHCR (`ghcr.io/celestia-island/celestia-island.github.io`) on
every `main` push, so nodes can pull instead of build:

```bash
docker pull ghcr.io/celestia-island/celestia-island.github.io:latest
```

## License

Licensed under the [Synthetic Source License (SySL), Version 1.0](./LICENSE).
