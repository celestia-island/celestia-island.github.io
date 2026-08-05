<template>
  <div class="scroll-outer">
    <button
      v-if="!compact"
      class="scroll-arrow scroll-arrow-left"
      :class="{ 'is-hidden': !canScrollLeft }"
      @click="scrollByCards(-1)"
      aria-label="Scroll left"
    >
      <div class="i-lucide-chevron-left w-5 h-5" />
    </button>
    <div class="scroll-container" ref="scrollRef" @wheel="onWheel">
      <div class="scroll-track" :class="{ 'is-compact': compact }">
        <div
          v-for="(p, i) in items"
          :key="p.id"
          class="card-wrapper reveal"
          :class="{ 'is-visible': visible, 'is-compact': compact }"
          :style="compact
            ? { transitionDelay: `${0.08 + i * 0.06}s` }
            : { width: cardWidth, minWidth: cardWidth, transitionDelay: `${0.08 + i * 0.06}s` }"
        >
          <ProjectCard :project="p" />
        </div>
      </div>
    </div>
    <button
      v-if="!compact"
      class="scroll-arrow scroll-arrow-right"
      :class="{ 'is-hidden': !canScrollRight }"
      @click="scrollByCards(1)"
      aria-label="Scroll right"
    >
      <div class="i-lucide-chevron-right w-5 h-5" />
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount, nextTick } from 'vue'
import type { Project } from '@/types/project'
import ProjectCard from '@/components/ProjectCard.vue'

const props = defineProps<{
  items: Project[]
  visible: boolean
}>()

// Minimum readable width a card may shrink to. Below this, a compact
// (no-scrollbar) layout would crush the cards, so we fall back to the
// horizontal scrolling mode instead.
const MIN_CARD_WIDTH = 210

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
</script>

<style scoped>
.scroll-outer {
  position: relative;
  display: flex;
  align-items: center;
  gap: 8px;
}

.scroll-arrow {
  flex-shrink: 0;
  width: 36px;
  height: 36px;
  border-radius: 50%;
  border: 1px solid var(--border-subtle);
  background: var(--bg-glass);
  backdrop-filter: blur(12px);
  color: var(--text-secondary);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s ease;
  z-index: 2;
}
.scroll-arrow:hover {
  color: var(--text-primary);
  border-color: var(--text-secondary);
}
.scroll-arrow.is-hidden {
  opacity: 0;
  pointer-events: none;
}

.scroll-container {
  overflow-x: auto;
  overflow-y: hidden;
  -webkit-overflow-scrolling: touch;
  touch-action: pan-x;
  scrollbar-width: none;
  flex: 1;
  min-width: 0;
  overscroll-behavior-x: contain;
}

.scroll-container::-webkit-scrollbar {
  display: none;
}

.scroll-track {
  display: flex;
  gap: 16px;
  width: max-content;
}

.scroll-track.is-compact {
  width: 100%;
}

.card-wrapper {
  flex-shrink: 0;
  height: 280px;
}

.card-wrapper.is-compact {
  flex: 1 1 0;
  min-width: 0;
}

.card-wrapper :deep(> *) {
  height: 100%;
}

@media (max-width: 639px) {
  .scroll-arrow {
    display: none;
  }

  .scroll-track {
    gap: 12px;
  }

  .card-wrapper {
    height: 220px;
  }
}

@media (max-width: 639px) and (max-height: 500px) {
  .card-wrapper {
    height: min(180px, 50dvh);
  }
}
</style>
