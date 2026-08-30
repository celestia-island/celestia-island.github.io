// Horizontal card scroller (converted from CardScroller.vue → TSX + CardScroller.scss).
import { computed, nextTick, onBeforeUnmount, onMounted, ref } from 'vue'
import { defineComponent } from 'vue'
import type { Project } from '@/types/project'
import ProjectCard from '@/components/ProjectCard'
import './CardScroller.scss'

// Minimum readable width a card may shrink to. Below this, a compact
// (no-scrollbar) layout would crush the cards, so we fall back to the
// horizontal scrolling mode instead.
const MIN_CARD_WIDTH = 210

export default defineComponent({
  name: 'CardScroller',
  props: {
    items: { type: Array as () => Project[], required: true },
    visible: { type: Boolean, required: true },
  },
  setup(props) {
    const scrollRef = ref<HTMLDivElement>()
    const containerWidth = ref(0)
    const canScrollLeft = ref(false)
    const canScrollRight = ref(false)
    const cardWidth = ref('280px')

    // Compact = no-scrollbar, cards share the track width evenly.
    // Only safe when there are few cards AND the container is wide enough to
    // give every card at least MIN_CARD_WIDTH; otherwise scroll instead.
    const compact = computed(() => {
      const n = props.items.length
      if (n === 0 || n > 3) return false
      const gap = window.innerWidth < 640 ? 12 : 16
      const perCard = (containerWidth.value - (n - 1) * gap) / n
      return containerWidth.value > 0 && perCard >= MIN_CARD_WIDTH
    })

    function updateCardWidth() {
      const el = scrollRef.value
      if (!el) return
      containerWidth.value = el.clientWidth
      if (compact.value) return
      const gap = 16
      const isMobile = window.innerWidth < 640
      const visibleCount = isMobile ? 1.2 : 3.2
      const w = (el.clientWidth - Math.ceil(visibleCount - 1) * gap) / visibleCount
      cardWidth.value = `${w}px`
    }

    function updateScrollState() {
      const el = scrollRef.value
      if (!el) return
      canScrollLeft.value = el.scrollLeft > 2
      canScrollRight.value = el.scrollLeft + el.clientWidth < el.scrollWidth - 2
    }

    function scrollByCards(direction: number) {
      const el = scrollRef.value
      if (!el) return
      el.scrollBy({ left: direction * parseFloat(cardWidth.value) * 1.1, behavior: 'smooth' })
    }

    let _wheelAccum = 0
    let _wheelRaf = 0

    function onWheel(e: WheelEvent) {
      const el = scrollRef.value
      if (!el || compact.value) return

      const delta = e.deltaY || e.deltaX
      if (delta === 0) return

      const atStart = el.scrollLeft <= 0 && delta < 0
      const atEnd = el.scrollLeft + el.clientWidth >= el.scrollWidth - 1 && delta > 0

      if (atStart || atEnd) {
        _wheelAccum = 0
        return
      }

      e.preventDefault()
      e.stopPropagation()

      _wheelAccum += delta

      if (!_wheelRaf) {
        _wheelRaf = requestAnimationFrame(function step() {
          if (Math.abs(_wheelAccum) < 0.5) {
            _wheelRaf = 0
            updateScrollState()
            return
          }
          const consume = _wheelAccum * 0.25
          _wheelAccum -= consume
          el.scrollLeft += consume
          _wheelRaf = requestAnimationFrame(step)
        })
      }
    }

    function onResize() {
      updateCardWidth()
      updateScrollState()
    }

    onMounted(() => {
      nextTick(() => {
        updateCardWidth()
        updateScrollState()
        scrollRef.value?.addEventListener('scroll', updateScrollState, { passive: true })
        window.addEventListener('resize', onResize)
      })
    })

    onBeforeUnmount(() => {
      scrollRef.value?.removeEventListener('scroll', updateScrollState)
      window.removeEventListener('resize', onResize)
      if (_wheelRaf) cancelAnimationFrame(_wheelRaf)
    })

    return () => (
      <div class="scroll-outer">
        {!compact.value ? (
          <button
            class={`scroll-arrow scroll-arrow-left ${!canScrollLeft.value ? 'is-hidden' : ''}`}
            onClick={() => scrollByCards(-1)}
            aria-label="Scroll left"
          >
            <div class="i-lucide-chevron-left w-5 h-5" />
          </button>
        ) : null}
        <div class="scroll-container" ref={scrollRef} onWheel={onWheel}>
          <div class={`scroll-track ${compact.value ? 'is-compact' : ''}`}>
            {props.items.map((p, i) => (
              <div
                key={p.id}
                class={`card-wrapper reveal ${props.visible ? 'is-visible' : ''} ${compact.value ? 'is-compact' : ''}`}
                style={compact.value
                  ? { transitionDelay: `${0.08 + i * 0.06}s` }
                  : { width: cardWidth.value, minWidth: cardWidth.value, transitionDelay: `${0.08 + i * 0.06}s` }}
              >
                <ProjectCard project={p} />
              </div>
            ))}
          </div>
        </div>
        {!compact.value ? (
          <button
            class={`scroll-arrow scroll-arrow-right ${!canScrollRight.value ? 'is-hidden' : ''}`}
            onClick={() => scrollByCards(1)}
            aria-label="Scroll right"
          >
            <div class="i-lucide-chevron-right w-5 h-5" />
          </button>
        ) : null}
      </div>
    )
  },
})
