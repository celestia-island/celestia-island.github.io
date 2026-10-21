import { createApp, watch } from 'vue'
import { createRouter, createWebHashHistory } from 'vue-router'
import App from './App'
import { initGoogleAnalytics } from './analytics'
import i18n from './i18n'
import { provideSnapContainer } from './composables/useSnapContainer'
import { applyViewportPolicy } from './mobileViewport'
import 'virtual:uno.css'
import '@unocss/reset/tailwind.css'
import './assets/styles/global.scss'

// Google Analytics 4 (dynamic enablement): index.html carries this site's
// own measurement id in #celestia-ga-measurement-id; analytics.ts injects
// the gtag.js loader only for a well-formed id and skips localhost, so
// stripped deployments and dev machines ship zero tracking.
initGoogleAnalytics()

// Mobile UX contract (hikari #325 sibling): normalize the viewport meta
// before first paint so phones never refuse pinch zoom (strips
// user-scalable=no / maximum-scale caps, writes user-scalable=yes);
// allowZoomOut pins minimum-scale=0.25 so the tall page can zoom out to
// fit. The tap-highlight reset ships via global.scss.
applyViewportPolicy({ allowZoomOut: true })

// Keep <html lang> in sync with the active locale (BCP 47 tags).
watch(
  () => i18n.global.locale.value,
  (locale) => { document.documentElement.lang = locale },
  { immediate: true }
)

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      component: () => import('./views/Home'),
    },
  ],
})

const app = createApp(App)
app.use(router)
app.use(i18n)
provideSnapContainer(app)
app.mount('#app')
