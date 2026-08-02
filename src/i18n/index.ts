import { createI18n } from 'vue-i18n'
import en from '../../res/i18n/en.json'
import zhHans from '../../res/i18n/zh-Hans.json'
import zhHant from '../../res/i18n/zh-Hant.json'
import ja from '../../res/i18n/ja.json'
import ko from '../../res/i18n/ko.json'
import es from '../../res/i18n/es.json'
import fr from '../../res/i18n/fr.json'
import ru from '../../res/i18n/ru.json'

let savedLocale: string | null = null
try { savedLocale = localStorage.getItem('celestia-locale') } catch { /* localStorage 不可用时优雅降级 */ }
const browserLocale = navigator.language

// Pre-BCP-47 locale codes persisted in localStorage by older versions.
const LEGACY_LOCALE_MAP: Record<string, string> = {
  zhs: 'zh-Hans',
  zht: 'zh-Hant',
  'zh-CN': 'zh-Hans',
  'zh-TW': 'zh-Hant',
  zh: 'zh-Hans',
}

function detectLocale(): string {
  if (savedLocale) {
    const normalized = LEGACY_LOCALE_MAP[savedLocale] ?? savedLocale
    if (['en', 'zh-Hans', 'zh-Hant', 'ja', 'ko', 'es', 'fr', 'ru'].includes(normalized)) {
      return normalized
    }
  }
  if (browserLocale.startsWith('zh')) {
    return browserLocale.includes('TW') || browserLocale.includes('HK') ? 'zh-Hant' : 'zh-Hans'
  }
  if (browserLocale.startsWith('ja')) return 'ja'
  if (browserLocale.startsWith('ko')) return 'ko'
  if (browserLocale.startsWith('es')) return 'es'
  if (browserLocale.startsWith('fr')) return 'fr'
  if (browserLocale.startsWith('ru')) return 'ru'
  return 'en'
}

const i18n = createI18n({
  legacy: false,
  locale: detectLocale(),
  fallbackLocale: 'en',
  messages: {
    en,
    'zh-Hans': zhHans,
    'zh-Hant': zhHant,
    ja,
    ko,
    es,
    fr,
    ru,
  },
})

export default i18n
