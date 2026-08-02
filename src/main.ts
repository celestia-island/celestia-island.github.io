import { createApp, watch } from 'vue'
import { createRouter, createWebHashHistory } from 'vue-router'
import App from './App.vue'
import i18n from './i18n'
import { provideSnapContainer } from './composables/useSnapContainer'
import 'virtual:uno.css'
import '@unocss/reset/tailwind.css'
import './assets/styles/global.scss'

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
      component: () => import('./views/Home.vue'),
    },
  ],
})

const app = createApp(App)
app.use(router)
app.use(i18n)
provideSnapContainer(app)
app.mount('#app')
