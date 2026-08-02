set shell := ["bash", "-c"]
set windows-shell := ["bash.exe", "-c"]
set unstable
set lists
# The shared celestia-devtools recipes define their own `dev` (devtools
# daemon); this file's recipes must win on name collisions, and just resolves
# duplicates in favour of the importing file.
set allow-duplicate-recipes

# Shared celestia-devtools recipes — NOT in git. Stage with: just fetch.
# `import?` silently skips when absent, so this justfile parses pre-fetch.
import? "./.just/git-bash-interop.just"
import? "./.just/celestia-devtools.just"

# Stage shared celestia-devtools recipes into .just/ (gitignored).
# Source order: explicit URL arg → local pip bundle (offline) → GitHub raw.
# curl honors HTTP_PROXY/HTTPS_PROXY/ALL_PROXY env vars automatically.
[script('bash')]
fetch URL='':
    #!/usr/bin/env bash
    set -euo pipefail
    out=.just/celestia-devtools.just
    mkdir -p .just
    if [ -n "{{URL}}" ]; then
      echo "[fetch] {{URL}} -> $out"
      curl -fsSL "{{URL}}" -o "$out"
    elif command -v celestia-devtools >/dev/null 2>&1; then
      src=$(celestia-devtools include-path)
      echo "[fetch] local bundle ($src) -> $out"
      cp "$src" "$out"
    else
      echo "[fetch] github raw -> $out"
      curl -fsSL "https://raw.githubusercontent.com/celestia-island/celestia-devtools/master/src/celestia_devtools/common.just" -o "$out"
    fi
    echo "[fetch] wrote $out"

default:
    @just --list

# Bootstrap celestia-devtools, install JS deps (pnpm) and Python build deps (Pillow).
install:
    just ensure
    just prefetch
    pip3 install Pillow

# Type-check the whole project with vue-tsc.
lint:
    npx vue-tsc -b --noEmit

# Build the static site into dist/ (delegates to scripts/build.py).
build:
    pnpm build

# Start the Vite dev server.
dev:
    pnpm dev

# Format Markdown documentation in place (celestia-devtools).
fmt:
    just fmt-markdown .

# Check Markdown formatting without writing (used by CI).
fmt-check:
    just fmt-markdown . --check
