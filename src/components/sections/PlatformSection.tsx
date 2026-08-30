// Platform section (converted from PlatformSection.vue → TSX; styles are global).
import { ref } from 'vue'
import { defineComponent } from 'vue'
import { useI18n } from 'vue-i18n'
import { useReveal } from '@/composables/useReveal'
import { projects } from '@/types/project'
import CardScroller from '@/components/CardScroller'

const items = projects.filter((p) => ['entelecheia', 'shittim-chest', 'evernight'].includes(p.id))

export default defineComponent({
  name: 'PlatformSection',
  setup(_, { expose }) {
    const { t } = useI18n()
    const { visible, triggerReveal } = useReveal()
    const el = ref<HTMLElement>()

    expose({ el, triggerReveal })

    return () => (
      <section id="projects" class="snap-section" ref={el}>
        <div class="section-inner">
          <div class="max-w-6xl mx-auto w-full">
            <div class={`text-center mb-6 reveal ${visible.value ? 'is-visible' : ''}`}>
              <h2 class="text-4xl sm:text-5xl font-bold tracking-tight">
                <span class="bg-gradient-to-r from-violet-400 to-cyan-400 bg-clip-text text-transparent">
                  {t('group.amphoreus')}
                </span>
              </h2>
              <p class="text-xs sm:text-sm font-medium tracking-widest uppercase mt-2 text-tertiary">{t('group.platform')}</p>
              <p class="text-sm mt-3 max-w-lg mx-auto text-secondary">{t('group.platformDesc')}</p>
            </div>
            <CardScroller items={items} visible={visible.value} />
          </div>
        </div>
      </section>
    )
  },
})
