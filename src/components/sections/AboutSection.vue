<template>
  <section id="about" class="snap-start flex flex-col about-section" ref="el">
    <div class="flex-1 flex items-center justify-center px-4 py-6 sm:py-8">
      <div
        class="glass-card-static text-center p-5 sm:p-6 max-w-xl mx-auto reveal"
        :class="{ 'is-visible': visible }"
      >
        <div class="mb-2 flex justify-center">
          <img
            :src="celestiaLogo"
            alt="Celestia Island Logo"
            class="w-10 h-10 sm:w-12 sm:h-12 object-contain rounded-xl animate-glow"
            draggable="false"
          />
        </div>
        <div
          class="about-text leading-normal max-w-lg mx-auto about-content prose prose-sm prose-zinc dark:prose-invert text-secondary"
          v-html="renderedAboutText"
        ></div>
      </div>
    </div>

    <footer
      class="border-t backdrop-blur-md reveal py-6 border-subtle bg-footer delay-300"
      :class="{ 'is-visible': visible }"
    >
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-full flex flex-col items-center justify-center gap-3 text-sm text-muted">
        <a href="https://github.com/celestia-island" target="_blank" class="nav-icon-btn no-underline group" title="GitHub">
          <div class="i-lucide-github w-5 h-5 group-hover:text-[var(--text-primary)] transition-colors" />
        </a>
        <span class="text-tertiary">
          「{{ t('site.slogan') }}」
        </span>
        <span>{{ t('site.footer.copyright', { year: new Date().getFullYear() }) }}</span>
        <span v-for="(item, index) in footerExtraItems" :key="index" class="footer-extra">
          <a v-if="typeof item === 'object'" :href="item.url" target="_blank" rel="noopener">{{ item.title }}</a>
          <span v-else v-html="item"></span>
        </span>
      </div>
    </footer>
  </section>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { marked } from 'marked'
import { useReveal } from '@/composables/useReveal'
import celestiaLogo from '@res/logos/celestia.webp'

const aboutDocs = import.meta.glob('../../../docs/**/about.md', { query: '?raw', import: 'default', eager: true })

const { t, locale } = useI18n()
const { visible, triggerReveal } = useReveal()
const el = ref<HTMLElement>()

const renderedAboutText = computed(() => {
  const currentLang = locale.value
  const fallbackLang = 'en'
  const docPath = `../../../docs/${currentLang}/about.md`
  const fallbackPath = `../../../docs/${fallbackLang}/about.md`

  const mdContent = (aboutDocs[docPath] as string) || (aboutDocs[fallbackPath] as string) || ''
  return marked.parse(mdContent)
})

interface FooterExtraLink {
  title: string
  url: string
}

type FooterExtraEntry = string | FooterExtraLink

function isFooterExtraLink(item: unknown): item is FooterExtraLink {
  return (
    typeof item === 'object'
    && item !== null
    && typeof (item as FooterExtraLink).title === 'string'
    && (item as FooterExtraLink).title.trim().length > 0
    && typeof (item as FooterExtraLink).url === 'string'
    && (item as FooterExtraLink).url.trim().length > 0
  )
}

function parseFooterExtra(raw: string): FooterExtraEntry[] {
  const value = raw.trim()
  if (!value) return []
  try {
    const parsed: unknown = JSON.parse(value)
    if (Array.isArray(parsed)) {
      return parsed.filter((item): item is FooterExtraEntry => isFooterExtraLink(item) || (typeof item === 'string' && item.trim().length > 0))
    }
    if (typeof parsed === 'string') return [parsed]
  } catch {
    /* not JSON — treat as a single raw HTML string */
  }
  return [value]
}

const footerExtraItems = computed(() => {
  let raw = ''
  try {
    raw = document.getElementById('celestia-footer-extra')?.textContent?.trim() ?? ''
  } catch {
    /* document unavailable in some prerender contexts — fall through */
  }
  if (!raw || raw.startsWith('$') || raw.startsWith('__')) {
    raw = (import.meta.env.VITE_FOOTER_EXTRA as string | undefined)?.trim() ?? ''
  }
  return parseFooterExtra(raw)
})

defineExpose({ el, triggerReveal })
</script>

<style scoped>
.about-content {
  font-size: 1.125rem;
  line-height: 1.75;
  text-align: center;
}
.about-content :deep(p) {
  margin-bottom: 1.2em;
  color: var(--text-secondary);
}
.about-content :deep(p:last-child) {
  margin-bottom: 0;
}
.about-content :deep(a) {
  color: var(--text-primary);
  text-decoration: underline;
  text-decoration-color: var(--border-subtle);
  text-underline-offset: 4px;
  transition: all 0.3s ease;
}
.about-content :deep(a:hover) {
  color: #a78bfa;
  text-decoration-color: #a78bfa;
}
.about-content :deep(strong) {
  color: var(--text-primary);
  font-weight: 600;
}

.about-text {
  font-size: 0.8125rem;
}

.footer-extra :deep(a) {
  color: var(--text-secondary);
  text-decoration: underline;
  text-decoration-color: var(--border-subtle);
  text-underline-offset: 4px;
  transition: color 0.3s ease;
}

.footer-extra :deep(a:hover) {
  color: var(--text-primary);
}

.delay-300 {
  transition-delay: 0.3s;
}

.about-section {
  min-height: 100vh;
  min-height: 100dvh;
}
</style>
