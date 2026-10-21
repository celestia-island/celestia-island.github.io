/**
 * Google Analytics 4 (gtag.js) bootstrap — dynamic enablement from the TS
 * entry, never a static snippet in the bundle.
 *
 * Contract (same shape as the FOOTER_EXTRA runtime knob of this site):
 * - The measurement ID rides the SERVED DOCUMENT in
 *   `<script type="application/json" id="celestia-ga-measurement-id">`.
 *   This repo is the source of celestia.world itself, so the built
 *   index.html carries the site's own id — GitHub Pages serves the dist
 *   as-is, with no container-start envsubst step. Docker
 *   deployments could substitute it the FOOTER_EXTRA way without any
 *   rebuild.
 * - Only a well-formed `G-…` id enables the loader; a raw `$…` / `__…`
 *   placeholder counts as absent, so stripped deployments ship zero
 *   tracking code.
 * - Localhost hosts are skipped so dev machines never pollute analytics.
 * - When enabled, the loader is injected at boot exactly like the official
 *   snippet: an async gtag.js script + dataLayer + `config` — non-blocking.
 */

const GA_TAG_ID = 'celestia-ga-measurement-id'

declare global {
  interface Window {
    dataLayer?: unknown[]
  }
}

/** Strict GA4 measurement-id shape (`G-` + 1..=32 uppercase alphanumerics). */
export function isValidMeasurementId(raw: string | null | undefined): raw is string {
  if (raw === null || raw === undefined || raw.length < 3) return false
  return raw.startsWith('G-') && /^[A-Z0-9]{1,32}$/.test(raw.slice(2))
}

/** Read the measurement id from the served document. Returns null when the
 * tag is absent or its content is a placeholder / not a well-formed id —
 * every failure mode means "tracking disabled". */
export function resolveMeasurementId(doc: Document = document): string | null {
  let raw: string | null
  try {
    raw = doc.getElementById(GA_TAG_ID)?.textContent?.trim() ?? null
  } catch {
    return null // doc-less context (prerender / workers)
  }
  if (raw === null || raw.length === 0 || raw.startsWith('$') || raw.startsWith('__')) {
    return null
  }
  return isValidMeasurementId(raw) ? raw : null
}

/** Hosts that must never report: local dev servers only. */
export function isLocalHost(hostname: string): boolean {
  return (
    hostname === 'localhost' ||
    hostname === '127.0.0.1' ||
    hostname === '[::1]' ||
    hostname.endsWith('.localhost')
  )
}

/** Inject the gtag.js loader + config into `doc`. Split from
 * `initGoogleAnalytics` so tests can drive a bare document/window. */
export function loadGoogleAnalytics(doc: Document, win: Window, id: string): void {
  const script = doc.createElement('script')
  script.async = true
  script.src = `https://www.googletagmanager.com/gtag/js?id=${encodeURIComponent(id)}`
  doc.head.appendChild(script)
  // gtag.js consumes dataLayer entries positionally; the rest-arg form is
  // Google's documented TypeScript variant of the official inline snippet.
  const dataLayer = (win.dataLayer = win.dataLayer ?? [])
  const gtag = (...args: unknown[]): void => {
    dataLayer.push(args)
  }
  gtag('js', new Date())
  gtag('config', id)
}
/** Test seam: reset the module's once-guard between cases (exported for
 * vitest only; production calls initGoogleAnalytics exactly once). */
export function __resetAnalyticsForTests(): void {
  initialized = false
}

/** Production entrypoint, called once at boot. Every gate is fail-closed:
 * no tag, malformed id, or a localhost host → no network call at all. */
let initialized = false

export function initGoogleAnalytics(): void {
  if (initialized) return // one loader per document, ever
  if (typeof document === 'undefined' || typeof window === 'undefined') return
  if (isLocalHost(window.location.hostname)) return
  const id = resolveMeasurementId(document)
  if (!id) return
  initialized = true
  try {
    loadGoogleAnalytics(document, window, id)
  } catch {
    // A hostile or broken predeclared dataLayer must never break the page.
  }
}
