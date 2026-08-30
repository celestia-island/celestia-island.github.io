// About + footer section (converted from AboutSection.vue → TSX + AboutSection.scss).
import { computed, ref } from 'vue'
import { defineComponent } from 'vue'
import { useI18n } from 'vue-i18n'
import { marked } from 'marked'
import { useReveal } from '@/composables/useReveal'
import celestiaLogo from '@res/logos/celestia.webp'
import './AboutSection.scss'

const aboutDocs = import.meta.glob('../../../docs/**/about.md', { query: '?raw', import: 'default', eager: true })

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

export default defineComponent({
  name: 'AboutSection',
  setup(_, { expose }) {
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

    expose({ el, triggerReveal })

    return () => (
      <section id="about" class="snap-start flex flex-col about-section" ref={el}>
        <div class="flex-1 flex items-center justify-center px-4 py-6 sm:py-8">
          <div class={`glass-card-static text-center p-5 sm:p-6 max-w-xl mx-auto reveal ${visible.value ? 'is-visible' : ''}`}>
            <div class="mb-2 flex justify-center">
              <img
                src={celestiaLogo}
                alt="Celestia Island Logo"
                class="w-10 h-10 sm:w-12 sm:h-12 object-contain rounded-xl animate-glow"
                draggable={false}
              />
            </div>
            <div
              class="about-text leading-normal max-w-lg mx-auto about-content prose prose-sm prose-zinc dark:prose-invert text-secondary"
              innerHTML={renderedAboutText.value as string}
            ></div>
          </div>
        </div>

        <footer class={`border-t backdrop-blur-md reveal py-6 border-subtle bg-footer delay-300 ${visible.value ? 'is-visible' : ''}`}>
          <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-full flex flex-col items-center justify-center gap-3 text-sm text-muted">
            <a href="https://github.com/celestia-island" target="_blank" class="nav-icon-btn no-underline group" title="GitHub">
              <div class="i-lucide-github w-5 h-5 group-hover:text-[var(--text-primary)] transition-colors" />
            </a>
            <span class="text-tertiary">
              「{t('site.slogan')}」
            </span>
            <span>{t('site.footer.copyright', { year: new Date().getFullYear() })}</span>
            {footerExtraItems.value.map((item, index) => (
              <span key={index} class="footer-extra">
                {typeof item === 'object'
                  ? <a href={item.url} target="_blank" rel="noopener">{item.title}</a>
                  : <span innerHTML={item}></span>}
              </span>
            ))}
          </div>
        </footer>
      </section>
    )
  },
})
