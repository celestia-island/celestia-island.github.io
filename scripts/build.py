#!/usr/bin/env python3
"""Build celestia-island.github.io — install deps, typecheck, bundle, generate favicons, copy assets."""

import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

try:
    from PIL import Image
except ImportError:
    print("ERROR: Pillow is required. Install with: pip install Pillow")
    sys.exit(1)

ROOT = Path(__file__).resolve().parent.parent
DIST = ROOT / "dist"
RES = ROOT / "res"
LOGOS = RES / "logos"
SOURCE_LOGO = LOGOS / "celestia.webp"


def run(cmd: list[str], *, cwd: Path | None = None) -> None:
    print(f"> {' '.join(cmd)}")
    # On Windows, pnpm/npx/vue-tsc are .cmd batch files that subprocess.run
    # cannot launch directly (CreateProcess can't find them without the
    # .cmd extension); shell=True lets the shell resolve them. On Unix the
    # bare executable names resolve fine without a shell.
    r = subprocess.run(cmd, cwd=cwd or ROOT, shell=os.name == "nt")
    if r.returncode != 0:
        sys.exit(r.returncode)


def generate_favicons() -> None:
    if not SOURCE_LOGO.exists():
        print(f"ERROR: source logo not found: {SOURCE_LOGO}")
        sys.exit(1)

    img = Image.open(SOURCE_LOGO).convert("RGBA")
    print(f"\n=== Generating favicons from {SOURCE_LOGO.name} ({img.size[0]}x{img.size[1]}) ===")

    sizes = {
        "favicon-16x16.png": 16,
        "favicon-32x32.png": 32,
        "favicon-48x48.png": 48,
        "android-chrome-192x192.png": 192,
        "android-chrome-512x512.png": 512,
        "apple-touch-icon.png": 180,
    }

    for filename, size in sizes.items():
        resized = img.resize((size, size), Image.LANCZOS)
        dst = DIST / filename
        resized.save(dst, "PNG")
        print(f"  {filename}")

    ico_sizes = [(16, 16), (32, 32), (48, 48)]
    ico_images = [img.resize(s, Image.LANCZOS) for s in ico_sizes]
    ico_images[0].save(
        DIST / "favicon.ico",
        format="ICO",
        sizes=ico_sizes,
        append_images=ico_images[1:],
    )
    print("  favicon.ico")

    manifest = {
        "name": "Celestia Island",
        "short_name": "Celestia",
        "icons": [
            {"src": "android-chrome-192x192.png", "sizes": "192x192", "type": "image/png"},
            {"src": "android-chrome-512x512.png", "sizes": "512x512", "type": "image/png"},
        ],
        "theme_color": "#8b5cf6",
        "background_color": "#000000",
        "display": "standalone",
    }
    (DIST / "site.webmanifest").write_text(json.dumps(manifest, indent=2) + "\n")
    print("  site.webmanifest")


def copy_logos() -> None:
    print("\n=== Copying logos ===")
    logos_dst = DIST / "logos"
    if logos_dst.exists():
        shutil.rmtree(logos_dst)
    shutil.copytree(LOGOS, logos_dst)
    for f in sorted(logos_dst.iterdir()):
        print(f"  logos/{f.name}")


def main() -> None:
    print("=== Installing dependencies ===")
    run(["pnpm", "install", "--ignore-scripts"])

    print("\n=== Checking hikari theme assets ===")
    run(["node", "scripts/check_theme_assets.mjs"])

    print("\n=== Type checking ===")
    run(["npx", "vue-tsc", "-b", "--noEmit"])

    print("\n=== Building ===")
    run(["npx", "vite", "build"])

    generate_favicons()
    copy_logos()

    print("\n=== Copying CNAME ===")
    cname = ROOT / "CNAME"
    if cname.exists():
        shutil.copy2(cname, DIST / "CNAME")
        print("  CNAME")

    print("\n=== Creating .nojekyll ===")
    (DIST / ".nojekyll").write_text("\n")

    print(f"\n✓ Build complete → {DIST}")


if __name__ == "__main__":
    main()
