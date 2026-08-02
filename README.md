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
| `FOOTER_EXTRA`    | Extra footer content rendered below the copyright line. Raw HTML allowed (e.g. `<a href="https://beian.miit.gov.cn/" target="_blank" rel="noopener">苏ICP备2025155733号-2</a>`). Injected into `index.html` at startup via `docker/entrypoint.sh`; omitted/empty on GitHub Pages renders nothing. |

The image is also published to GHCR (`ghcr.io/celestia-island/celestia-island.github.io`) on
every `main` push, so nodes can pull instead of build:

```bash
docker pull ghcr.io/celestia-island/celestia-island.github.io:latest
```

## License

Licensed under the [Synthetic Source License (SySL), Version 1.0](./LICENSE).
