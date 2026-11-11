#!/usr/bin/env node
/* Validate the public hikari theme payloads and their CSS-only overlays. */

import { createRequire } from "node:module";
import { readFileSync, existsSync } from "node:fs";
import { fileURLToPath } from "node:url";
import path from "node:path";

const require = createRequire(import.meta.url);
const viteRequire = createRequire(require.resolve("vite"));
const postcss = viteRequire("postcss");
const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const THEME_DIR = path.join(ROOT, "public", "themes");
const FOUNDATION = "hikari-foundation.css";
const THEME_IDS = ["endfield-wuling", "arcaea-gothic"];
const LOCALES = ["ar", "de", "en", "es", "fr", "ja", "ko", "pt", "ru", "zh-Hans", "zh-Hant"];
const SLOTS = [
  "primary", "secondary", "accent", "text", "muted", "border", "focusedBorder",
  "background", "surface", "selectedBackground", "selectedText", "statusBarBackground",
  "success", "error", "warning", "info", "onSolidText", "onSolidIcon",
];
const REQUIRED_SELECTORS = [
  ".hk-btn-primary", ".hk-icon-button", ".hk-input-box", ".hk-textarea-field",
  ".hk-card", ".hk-modal-content", ".hk-drawer-panel", ".hk-select-trigger",
  ".hk-select-popout", ".hk-tabs-indicator", ".hk-switch-thumb", ".hk-slider-thumb",
  ".hk-progress-bar-fill-loading", ".hk-progress-ring-track", ".hk-tooltip-popup",
  ".hk-touch-hint-bar", ".hk-toast-item", ".hk-scrollbar-thumb", ".hk-empty-state--boxed",
  ".hk-skeleton", ".hk-spinner", ".hk-loading-veil-body",
];
const FORBIDDEN_PROPS = new Set([
  "z-index", "position", "inset", "top", "right", "bottom", "left", "display",
  "width", "height", "padding", "margin", "gap", "order", "contain", "box-sizing",
  "touch-action", "overscroll-behavior", "resize", "user-select", "will-change",
  "pointer-events", "transform", "translate", "scale", "rotate", "clip-path",
  "backdrop-filter", "font-size", "line-height", "content",
]);
const FORBIDDEN_PROP_PREFIXES = [
  "min-", "max-", "flex", "grid", "overflow", "scroll", "mask",
];
const FORBIDDEN_CUSTOM_PREFIXES = [
  "--z-", "--hk-z-", "--hk-overlay-z-", "--hk-scroll", "--hk-sheet", "--hk-pop-",
  "--hk-pad", "--hk-modal-shift", "--hk-modal-duration", "--hk-modal-ease",
  "--radius", "--shadow", "--hk-modal-bg", "--hk-drawer-bg", "--hk-popover-bg",
  "--hk-modal-blur", "--hk-drawer-blur", "--hk-popover-blur",
];

function fail(code, detail) {
  const error = new Error(`${code}: ${detail}`);
  error.code = code;
  throw error;
}

function readTheme(id) {
  return JSON.parse(readFileSync(path.join(THEME_DIR, `${id}.theme.json`), "utf8"));
}

function readCss(name) {
  return readFileSync(path.join(THEME_DIR, name), "utf8");
}

function isHex(value) {
  return typeof value === "string" && /^#[0-9a-fA-F]{6}$/.test(value);
}

function rgb(hex) {
  return [1, 3, 5].map((offset) => Number.parseInt(hex.slice(offset, offset + 2), 16) / 255);
}

function luminance(hex) {
  const [r, g, b] = rgb(hex).map((channel) =>
    channel <= 0.04045 ? channel / 12.92 : ((channel + 0.055) / 1.055) ** 2.4);
  return 0.2126 * r + 0.7152 * g + 0.0722 * b;
}

function contrast(a, b) {
  const [high, low] = [luminance(a), luminance(b)].sort((x, y) => y - x);
  return (high + 0.05) / (low + 0.05);
}

function validatePayload(config) {
  if (!THEME_IDS.includes(config.id)) fail("theme-id", String(config.id));
  if (config.formatVersion !== "0.1" || config.preset !== false) fail("format", config.id);
  if (!config.name || typeof config.name !== "string") fail("name", config.id);
  const locales = Object.keys(config.name_i18n ?? {}).sort();
  if (JSON.stringify(locales) !== JSON.stringify([...LOCALES].sort())) {
    fail("i18n", `${config.id} locales=${locales.join(",")}`);
  }
  for (const locale of LOCALES) {
    if (!config.name_i18n[locale]?.trim()) fail("i18n-empty", `${config.id}/${locale}`);
  }
  for (const mode of ["dark", "light"]) {
    const tokens = config.tokens?.[mode];
    if (!tokens) fail("tokens", `${config.id}/${mode}`);
    for (const slot of SLOTS) {
      if (!isHex(tokens[slot])) fail("token-hex", `${config.id}/${mode}/${slot}`);
    }
    const requiredContrast = [
      ["text", "surface", 4.5], ["text", "background", 4.5], ["muted", "surface", 4.5],
      ["primary", "surface", 3], ["focusedBorder", "surface", 3],
      ["selectedText", "selectedBackground", 4.5], ["text", "statusBarBackground", 4.5],
      ["onSolidText", "primary", 4.5], ["onSolidIcon", "primary", 4.5],
      ["onSolidText", "secondary", 4.5], ["onSolidText", "error", 4.5],
      ["onSolidText", "success", 4.5], ["onSolidText", "warning", 4.5],
      ["onSolidText", "info", 4.5],
    ];
    for (const [foreground, background, minimum] of requiredContrast) {
      const ratio = contrast(tokens[foreground], tokens[background]);
      if (ratio < minimum) {
        fail("contrast", `${config.id}/${mode} ${foreground}/${background}=${ratio.toFixed(2)} < ${minimum}`);
      }
    }
  }
  const mounts = config.css_mounts;
  if (!Array.isArray(mounts) || mounts.length !== 1) fail("mount-count", config.id);
  const mount = mounts[0];
  if (!Array.isArray(mount.templates) || mount.templates.length < 2) fail("mount-mirrors", config.id);
  for (const template of mount.templates) {
    if (!template.startsWith("https://") || template.split("{{ file }}").length !== 2) {
      fail("mount-template", template);
    }
  }
  for (const required of [FOUNDATION, `${config.id}.css`]) {
    if (!mount.files.includes(required)) fail("mount-file", `${config.id} missing ${required}`);
  }
  for (const file of mount.files) {
    if (!existsSync(path.join(THEME_DIR, file))) fail("mount-file", `${config.id}/${file}`);
  }
}

function scopedSelector(selector, ownId, shared) {
  const exact = `:root[data-theme="${ownId}"]`;
  const both = ':root:is([data-theme="endfield-wuling"], [data-theme="arcaea-gothic"])';
  return selector.startsWith(exact) || (shared && selector.startsWith(both));
}

function reducedTarget(selector) {
  return selector.trim().split(/\s+/).at(-1);
}

function validateCss(sharedCss, ownCss, config, requiredSelectors = REQUIRED_SELECTORS) {
  const documents = [];
  for (const [shared, source] of [[true, sharedCss], [false, ownCss]]) {
    let document;
    try {
      document = postcss.parse(source, { from: shared ? FOUNDATION : `${config.id}.css` });
    } catch (error) {
      fail("css-parse", error.message);
    }
    documents.push({ shared, source, document });
  }

  const definitions = new Set();
  const references = [];
  const keyframes = new Set();
  const animations = [];
  let ruleCount = 0;

  for (const { shared, document } of documents) {
    document.walkAtRules("keyframes", (atRule) => {
      if (!atRule.params.startsWith("ct-")) fail("keyframes-prefix", atRule.params);
      if (keyframes.has(atRule.params)) fail("keyframes-duplicate", atRule.params);
      keyframes.add(atRule.params);
    });
    document.walkRules((rule) => {
      const inKeyframes = rule.parent?.type === "atrule" && rule.parent.name === "keyframes";
      if (inKeyframes) return;
      ruleCount += 1;
      const selectors = postcss.list.comma(rule.selector);
      for (const selector of selectors) {
        if (!scopedSelector(selector, config.id, shared)) fail("selector-scope", selector);
        if (/enter-|leave-|popout-host|scroll-container-viewport::after|empty-sr-only|body::|#app|#hk-overlay-root|\*|nth-child|\[class/.test(selector)) {
          fail("forbidden-selector", selector);
        }
      }
      for (const required of requiredSelectors) {
        // Coverage is asserted below across the complete pair, never by a zero-hit scan.
      }
      rule.walkDecls((decl) => {
        const prop = decl.prop.toLowerCase();
        if (decl.important) fail("important", `${rule.selector} ${decl.prop}`);
        if (decl.value.includes("url(")) fail("external-asset", rule.selector);
        if (prop.startsWith("--theme-")) definitions.add(prop);
        for (const match of decl.value.matchAll(/var\(\s*(--theme-[a-z0-9-]+)/g)) {
          references.push([match[1], rule.selector]);
        }
        for (const prefix of FORBIDDEN_CUSTOM_PREFIXES) {
          if (prop.startsWith(prefix)) fail("forbidden-custom-property", `${rule.selector} ${decl.prop}`);
        }
        if (prop === "--hk-modal-radius" && !rule.selector.includes("data-radius")) {
          fail("radius-preference", rule.selector);
        }
        const statusPseudo = rule.selector.includes(".s-status-bar::before");
        const statusGeometry = new Set(["content", "position", "inset", "height", "pointer-events"]);
        if ((FORBIDDEN_PROPS.has(prop) || FORBIDDEN_PROP_PREFIXES.some((prefix) => prop.startsWith(prefix))) &&
            !(statusPseudo && statusGeometry.has(prop))) {
          fail("forbidden-property", `${rule.selector} ${decl.prop}`);
        }
        if (prop === "filter" && !rule.selector.includes(".hk-btn")) {
          fail("filter-scope", rule.selector);
        }
        if ((prop === "transition" || prop === "transition-property") && decl.value.includes("all")) {
          fail("transition-all", rule.selector);
        }
        if (prop === "animation") {
          if (!rule.selector.includes("s-theme-transitioning") &&
              !rule.selector.includes("hk-progress-bar-fill-loading")) {
            fail("animation-scope", rule.selector);
          }
          if (decl.value.includes("infinite") && !rule.selector.includes("hk-progress-bar-fill-loading")) {
            fail("decorative-loop", rule.selector);
          }
          animations.push(rule.selector);
        }
      });
    });
    document.walkDecls((decl) => {
      const inKeyframes = decl.parent?.parent?.type === "atrule" && decl.parent.parent.name === "keyframes";
      if (decl.prop === "background-position" && !inKeyframes) {
        fail("background-position", decl.parent.selector);
      }
      if (decl.prop === "filter" && inKeyframes && !decl.parent.parent.params.startsWith("ct-")) {
        fail("filter-scope", decl.parent.parent.params);
      }
    });
  }

  const combined = documents.map(({ source }) => source).join("\n");
  for (const required of requiredSelectors) {
    if (!combined.includes(required)) fail("selector-coverage", required);
  }
  for (const [name, selector] of references) {
    if (!definitions.has(name)) fail("undefined-variable", `${name} at ${selector}`);
  }

  for (const selector of new Set(animations)) {
    const target = reducedTarget(selector);
    let osStatic = false;
    let classStatic = false;
    for (const { document } of documents) {
      document.walkRules((rule) => {
        if (!postcss.list.comma(rule.selector).some((item) => reducedTarget(item) === target)) return;
        const staticAnimation = rule.nodes?.some((decl) => decl.prop === "animation" && decl.value === "none");
        if (!staticAnimation) return;
        const inOsMedia = rule.parent?.type === "atrule" && rule.parent.name === "media" &&
          rule.parent.params.includes("prefers-reduced-motion: reduce");
        if (inOsMedia) osStatic = true;
        if (rule.selector.includes(".reduce-motion")) classStatic = true;
      });
    }
    if (!osStatic || !classStatic) fail("reduced-motion", `${target} os=${osStatic} class=${classStatic}`);
  }
  return { rules: ruleCount, keyframes: keyframes.size };
}

function contrastSummary(config) {
  return ["dark", "light"].map((mode) => {
    const tokens = config.tokens[mode];
    return `${mode}:text=${contrast(tokens.text, tokens.surface).toFixed(2)} primary/on-solid=${contrast(tokens.primary, tokens.onSolidText).toFixed(2)}`;
  }).join(" · ");
}

function validateTheme(config) {
  validatePayload(config);
  const css = validateCss(readCss(FOUNDATION), readCss(`${config.id}.css`), config);
  return `${config.id} [${css.rules} rules/${css.keyframes} keyframes] ${contrastSummary(config)}`;
}

function expectFailure(code, callback) {
  try {
    callback();
  } catch (error) {
    if (error.code === code) {
      console.log(`negative-control PASS ${code}`);
      return;
    }
    fail("selftest-wrong-error", `wanted ${code}, got ${error.code ?? error.message}`);
  }
  fail("selftest-survivor", `${code} mutation was accepted`);
}

function selftest() {
  const configs = THEME_IDS.map(readTheme);
  for (const config of configs) console.log(`theme PASS ${validateTheme(config)}`);
  const config = configs[0];
  const shared = readCss(FOUNDATION);
  const own = readCss(`${config.id}.css`);
  const valid = (a = shared, b = own, c = config, required = REQUIRED_SELECTORS) => validateCss(a, b, c, required);

  expectFailure("css-parse", () => valid(shared, own.replace("--theme-line:", "--theme-line")));
  expectFailure("selector-scope", () => valid(shared, `${own}\n.hk-btn { color: red; }`));
  expectFailure("forbidden-property", () => valid(shared,
    `${own}\n:root[data-theme="endfield-wuling"] .hk-btn { transform: scale(1); }`));
  expectFailure("forbidden-selector", () => valid(shared,
    `${own}\n:root[data-theme="endfield-wuling"] .hk-modal-content-enter-active { opacity: 1; }`));
  expectFailure("forbidden-custom-property", () => valid(shared,
    `${own}\n:root[data-theme="endfield-wuling"] { --radius-md: 0; }`));
  expectFailure("undefined-variable", () => valid(shared, own.replace("--theme-primary-fill:", "--theme-renamed-fill:")));
  expectFailure("selector-coverage", () => valid(shared, own, config, [".hk-not-a-real-component"]));
  expectFailure("reduced-motion", () => valid(shared, own.replace("prefers-reduced-motion: reduce", "prefers-reduced-motion: no-preference")));
  const badContrast = structuredClone(config);
  badContrast.tokens.light.text = badContrast.tokens.light.surface;
  expectFailure("contrast", () => validatePayload(badContrast));
  const badMount = structuredClone(config);
  badMount.css_mounts[0].files[0] = "missing.css";
  expectFailure("mount-file", () => validatePayload(badMount));
  console.log("theme selftest PASS 10 negative controls");
}

for (const id of THEME_IDS) {
  if (!existsSync(path.join(THEME_DIR, `${id}.theme.json`)) || !existsSync(path.join(THEME_DIR, `${id}.css`))) {
    fail("missing-theme", id);
  }
}

if (process.argv.includes("--selftest")) {
  selftest();
} else {
  for (const id of THEME_IDS) console.log(`theme PASS ${validateTheme(readTheme(id))}`);
  console.log("theme assets PASS 2 themes");
}
