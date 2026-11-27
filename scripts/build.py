#!/usr/bin/env python3
"""Build celestia-island.github.io — install deps, typecheck, bundle, generate favicons, copy assets, verify logo wiring."""

import json
import os
import re
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
PROJECTS_TS = ROOT / "src" / "types" / "project.ts"
PROJECT_CARD_TSX = ROOT / "src" / "components" / "ProjectCard.tsx"


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


def verify_logo_wiring() -> None:
    """Fail the build when a project logo is tracked but not wired into the card.

    The card component resolves logos through the explicit ``logoMap``; the
    icon fallback path cannot save an unwired card because presetIcons never
    generates data-driven icon classes (verified 2026-11-27: ``vue-tsc`` and
    ``vite build`` both stay green while the emitted logo asset silently
    disappears). This check is the standing, fail-closed gate for that class:

    1. every ``project.ts`` id with a tracked ``res/logos/<id>.webp`` must
       appear in ``logoMap`` (logo home extras without a project entry are
       fine — ``res/logos/`` is the org-wide logo store);
    2. every ``logoMap`` key must be a ``project.ts`` id AND have its logo
       file tracked (dangling entries = typos);
    3. every wired id must have its hashed asset emitted under
       ``dist/assets/<id>-*.webp`` (proves vite actually bundled the import);
    4. every ``project.ts`` id must have a tracked logo file at all — the
       icon fallback is never generated, so an id without a logo renders a
       blank slot; this closes the deletion hole (removing the file together
       with the routine cleanup of its dangling import + map entry used to
       pass silently because rules 1–3 were conditional on the file).
    """
    print("\n=== Verifying logo wiring ===")
    problems: list[str] = []

    projects_src = PROJECTS_TS.read_text(encoding="utf-8")
    project_ids = re.findall(r"\bid\s*:\s*['\"]([^'\"]+)['\"]", projects_src)
    duplicate_ids = sorted({i for i in project_ids if project_ids.count(i) > 1})
    if duplicate_ids:
        problems.append(f"duplicate project.ts ids: {duplicate_ids}")
    id_set = set(project_ids)

    card_src = PROJECT_CARD_TSX.read_text(encoding="utf-8")
    map_keys = _logo_map_keys(card_src)
    duplicate_keys = sorted({k for k in map_keys if map_keys.count(k) > 1})
    if duplicate_keys:
        problems.append(f"duplicate logoMap keys: {duplicate_keys}")
    wired = set(map_keys)

    dangling = sorted(wired - id_set)
    if dangling:
        problems.append(f"logoMap keys that are not project ids: {dangling}")

    tracked = {p.stem for p in LOGOS.glob("*.webp")}
    unwired = sorted({i for i in id_set if i in tracked} - wired)
    if unwired:
        problems.append(
            f"project ids with a tracked res/logos/<id>.webp but no logoMap entry: {unwired}"
        )
    missing_files = sorted(i for i in wired & id_set if i not in tracked)
    if missing_files:
        problems.append(f"logoMap entries without a tracked res/logos/<id>.webp: {missing_files}")
    logo_less = sorted(id_set - tracked)
    if logo_less:
        problems.append(
            "project ids without a tracked res/logos/<id>.webp (the icon fallback is not "
            f"generated by presetIcons, so these render a blank slot): {logo_less}"
        )

    unwired_emitted: list[str] = []
    for project_id in sorted(wired & id_set):
        # Assumes vite's hashed `[name]-[hash]` emission; an unhashed future
        # pattern (`<id>.webp`) fails closed here (loud RED, one-line fix).
        if not list(DIST.glob(f"assets/{project_id}-*.webp")):
            unwired_emitted.append(project_id)
    if unwired_emitted:
        problems.append(f"wired ids whose logo asset was not emitted under dist/assets: {unwired_emitted}")

    if problems:
        for problem in problems:
            print(f"  LOGO WIRING ERROR: {problem}")
        print(
            "  Fix: import the logo in ProjectCard.tsx and add it to logoMap "
            "(or remove the stale logoMap entry / logo file, whichever is wrong)."
        )
        sys.exit(1)

    print(f"  ok: {len(id_set)} project ids, {len(wired)} wired logos, all assets emitted")


def _logo_map_keys(card_src: str) -> list[str]:
    """Extract the logoMap keys from the card component source.

    Anchored on the ``const logoMap`` block (not on the value shape): any
    ``key:`` entry line inside the block counts as wired, whatever the local
    variable is named (renames, casts, dropped trailing commas, any indent
    and values continuing on the next line are all accepted). Line comments
    are skipped; ``/* */`` blocks are stripped first.
    """
    block = re.search(r"const logoMap[^{]*\{(.+?)^\}", card_src, flags=re.S | re.M)
    if block is None:
        return []
    body = re.sub(r"/\*.*?\*/", "", block.group(1), flags=re.S)
    keys: list[str] = []
    for line in body.splitlines():
        entry = re.match(r"\s*['\"]?([A-Za-z0-9-]+)['\"]?:", line)
        if entry:
            keys.append(entry.group(1))
    return keys


def main() -> None:
    print("=== Installing dependencies ===")
    run(["pnpm", "install", "--ignore-scripts"])

    print("\n=== Type checking ===")
    run(["npx", "vue-tsc", "-b", "--noEmit"])

    print("\n=== Building ===")
    run(["npx", "vite", "build"])

    generate_favicons()
    copy_logos()
    verify_logo_wiring()

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
