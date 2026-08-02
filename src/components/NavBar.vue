<template>
  <nav class="fixed top-0 left-0 right-0 z-50 backdrop-blur-md border-b border-[var(--border-subtle)] bg-nav" role="navigation" aria-label="Main navigation">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <div class="flex items-center justify-between h-16">
        <a href="#" class="flex items-center gap-3 no-underline group text-primary" @click.prevent="scrollToTop">
          <img :src="celestiaLogo" alt="Celestia Island" class="w-8 h-8 object-contain rounded-lg animate-glow" draggable="false" />
          <span class="text-lg font-semibold tracking-wide bg-gradient-to-r from-violet-400 via-cyan-400 to-pink-400 bg-clip-text text-transparent">
            {{ t('site.title') }}
          </span>
        </a>

        <div class="flex items-center gap-1.5">
          <a href="https://github.com/celestia-island" target="_blank" class="nav-icon-btn no-underline group" title="GitHub">
            <div class="i-lucide-github w-5 h-5 group-hover:text-[var(--text-primary)] transition-colors" />
          </a>

          <button @click="toggleTheme" class="nav-icon-btn" :title="theme === 'dark' ? 'Light mode' : 'Dark mode'">
            <div v-if="theme === 'dark'" class="i-lucide-sun w-5 h-5 transition-transform hover:rotate-45" />
            <div v-else class="i-lucide-moon w-5 h-5 transition-transform hover:-rotate-12" />
          </button>

          <div ref="langRef" class="relative">
            <button @click.stop="showLangMenu = !showLangMenu" class="nav-lang-btn" :aria-expanded="showLangMenu" aria-haspopup="listbox">
              <div class="i-lucide-globe w-5 h-5" />
              <span class="hidden sm:inline ml-0.5">{{ currentLangLabel }}</span>
              <div class="i-lucide-chevron-down w-3.5 h-3.5 ml-0.5 opacity-60 transition-transform" :class="showLangMenu ? 'rotate-180' : ''" />
            </button>
            <transition name="fade">
              <div
                v-if="showLangMenu"
                class="absolute right-0 top-full mt-2 bg-[var(--bg-primary)] border border-[var(--border-subtle)] p-2 rounded-xl min-w-[140px] shadow-2xl z-50 flex flex-col gap-1"
                role="listbox"
                :aria-label="t('site.nav.languageList')"
              >
                <button
                  v-for="lang in langs"
                  :key="lang.code"
                  @click="switchLang(lang.code)"
                  class="w-full text-left px-3 py-2 rounded-lg text-sm transition-all duration-300 hover:bg-[var(--bg-secondary)] flex items-center justify-between group"
                  :class="{ 'text-[var(--text-primary)] font-medium bg-[var(--bg-secondary)]': locale === lang.code, 'text-secondary': locale !== lang.code }"
                  role="option"
                  :aria-selected="locale === lang.code"
                >
                  <span class="group-hover:text-[var(--text-primary)] transition-colors">{{ lang.label }}</span>
                  <div v-if="locale === lang.code" class="i-lucide-check w-3.5 h-3.5 text-violet-400" />
                </button>
              </div>
            </transition>
          </div>
        </div>
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount, inject } from 'vue'
import { useI18n } from 'vue-i18n'
import { useTheme } from '@/composables/useTheme'
import { SNAP_CONTAINER_KEY } from '@/composables/useSnapContainer'
import celestiaLogo from '@res/logos/celestia.webp'

const { t, locale } = useI18n()
const { theme, toggleTheme } = useTheme()
const showLangMenu = ref(false)
const langRef = ref<HTMLDivElement>()
const snapContainerRef = inject(SNAP_CONTAINER_KEY, null)
const snapContainer = computed(() => snapContainerRef?.value ?? null)

const langs = [
  { code: 'en', label: 'English' },
  { code: 'zh-Hans', label: '简体中文' },
  { code: 'zh-Hant', label: '繁體中文' },
  { code: 'ja', label: '日本語' },
  { code: 'ko', label: '한국어' },
  { code: 'es', label: 'Español' },
  { code: 'fr', label: 'Français' },
  { code: 'ru', label: 'Русский' },
]

const currentLangLabel = computed(() => {
  const l = langs.find(l => l.code === locale.value)
  return l?.label ?? 'English'
})

function switchLang(code: string) {
  locale.value = code
  try { localStorage.setItem('celestia-locale', code) } catch { /* guard */ }
  showLangMenu.value = false
}

function scrollToTop() {
  snapContainer.value?.scrollTo({ top: 0, behavior: 'smooth' })
}

function onClickOutside(e: MouseEvent) {
  if (langRef.value && !langRef.value.contains(e.target as Node)) {
    showLangMenu.value = false
  }
}

onMounted(() => window.addEventListener('click', onClickOutside))
onBeforeUnmount(() => window.removeEventListener('click', onClickOutside))
</script>

<style scoped>
.fade-enter-active, .fade-leave-active {
  transition: opacity 0.3s ease, transform 0.3s ease;
}
.fade-enter-from, .fade-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}
</style>
