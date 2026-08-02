<template>
  <div ref="snapContainer" class="snap-container">
    <HeroSection ref="heroRef" />
    <PlatformSection ref="platformRef" />
    <FrameworkSection ref="frameworkRef" />
    <ToolsSection ref="toolsRef" />
    <AboutSection ref="aboutRef" />
  </div>

  <nav class="fixed right-4 sm:right-6 top-1/2 -translate-y-1/2 z-40 flex flex-col gap-3" role="navigation" :aria-label="t('site.nav.home')">
    <button
      v-for="i in 5"
      :key="i"
      class="w-1 rounded-full transition-all duration-300 cursor-pointer border-none p-0"
      :style="{
        height: currentPage === i - 1 ? '2rem' : '1.5rem',
        background: currentPage === i - 1 ? '#a78bfa' : 'var(--text-muted)',
      }"
      :title="pageLabels[i - 1]"
      :aria-label="pageLabels[i - 1]"
      @click="scrollToPage(i - 1)"
    />
  </nav>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount } from 'vue'
import { useI18n } from 'vue-i18n'
import { setSnapContainer } from '@/composables/useSnapContainer'
import HeroSection from '@/components/sections/HeroSection.vue'
import PlatformSection from '@/components/sections/PlatformSection.vue'
import FrameworkSection from '@/components/sections/FrameworkSection.vue'
import ToolsSection from '@/components/sections/ToolsSection.vue'
import AboutSection from '@/components/sections/AboutSection.vue'

import '@/assets/styles/snap.scss'

const { t } = useI18n()

const snapContainer = ref<HTMLDivElement>()
const heroRef = ref<InstanceType<typeof HeroSection>>()
const platformRef = ref<InstanceType<typeof PlatformSection>>()
const frameworkRef = ref<InstanceType<typeof FrameworkSection>>()
const toolsRef = ref<InstanceType<typeof ToolsSection>>()
const aboutRef = ref<InstanceType<typeof AboutSection>>()

const currentPage = ref(0)

const pageLabels = computed(() => [
  t('site.nav.home'),
  t('group.amphoreus'),
  t('group.arcaea'),
  t('group.decagrammaton'),
  t('site.nav.about'),
])

const sectionRefs = computed(() => [
  heroRef.value?.$el as HTMLElement | undefined,
  platformRef.value?.$el as HTMLElement | undefined,
  frameworkRef.value?.$el as HTMLElement | undefined,
  toolsRef.value?.$el as HTMLElement | undefined,
  aboutRef.value?.$el as HTMLElement | undefined,
])

const sectionTriggers = computed(() => [
  null,
  platformRef.value?.triggerReveal,
  frameworkRef.value?.triggerReveal,
  toolsRef.value?.triggerReveal,
  aboutRef.value?.triggerReveal,
])

function scrollToPage(index: number) {
  const el = sectionRefs.value[index]
  if (el) snapContainer.value?.scrollTo({ top: (el as HTMLElement).offsetTop, behavior: 'smooth' })
}

// Gesture handling: a fast flick (wheel or touch) would otherwise skip
// several pages via native scroll-snap. Each gesture is capped at one page,
// with a lock window so a single swipe can never cascade into multiple pages.
const PAGE_LOCK_MS = 650
const WHEEL_THRESHOLD = 48
const TOUCH_THRESHOLD = 24

let lastPageChange = 0
let wheelAccum = 0
let touchTracking = false
let touchStartY = 0
let touchAccum = 0
let snapContainerEl: HTMLDivElement | null = null

function pageStep(direction: 1 | -1) {
  const now = performance.now()
  if (now - lastPageChange < PAGE_LOCK_MS) return
  const last = sectionRefs.value.length - 1
  const next = Math.min(last, Math.max(0, currentPage.value + direction))
  if (next === currentPage.value) return
  lastPageChange = now
  scrollToPage(next)
}

function onWheel(e: WheelEvent) {
  if (Math.abs(e.deltaX) > Math.abs(e.deltaY)) return
  e.preventDefault()
  wheelAccum += e.deltaY
  if (Math.abs(wheelAccum) < WHEEL_THRESHOLD) return
  const direction: 1 | -1 = wheelAccum >= 0 ? 1 : -1
  wheelAccum = 0
  pageStep(direction)
}

function onTouchStart(e: TouchEvent) {
  if (e.touches.length !== 1) {
    touchTracking = false
    return
  }
  touchTracking = true
  touchStartY = e.touches[0].clientY
  touchAccum = 0
}

function onTouchMove(e: TouchEvent) {
  if (!touchTracking || e.touches.length !== 1) return
  e.preventDefault()
  touchAccum = e.touches[0].clientY - touchStartY
}

function onTouchEnd() {
  if (!touchTracking) return
  touchTracking = false
  if (Math.abs(touchAccum) < TOUCH_THRESHOLD) return
  pageStep(touchAccum > 0 ? -1 : 1)
}

let observer: IntersectionObserver

onMounted(() => {
  setSnapContainer(snapContainer.value ?? null)

  snapContainerEl = snapContainer.value ?? null
  snapContainerEl?.addEventListener('wheel', onWheel, { passive: false })
  snapContainerEl?.addEventListener('touchstart', onTouchStart, { passive: true })
  snapContainerEl?.addEventListener('touchmove', onTouchMove, { passive: false })
  snapContainerEl?.addEventListener('touchend', onTouchEnd, { passive: true })

  observer = new IntersectionObserver((entries) => {
    for (const entry of entries) {
      if (entry.isIntersecting) {
        const idx = sectionRefs.value.indexOf(entry.target as HTMLElement)
        if (idx !== -1) {
          currentPage.value = idx
          const trigger = sectionTriggers.value[idx]
          if (trigger) trigger()
        }
      }
    }
  }, { root: snapContainer.value, threshold: 0.25 })

  for (const el of sectionRefs.value) {
    if (el) observer.observe(el as HTMLElement)
  }
})

onBeforeUnmount(() => {
  snapContainerEl?.removeEventListener('wheel', onWheel)
  snapContainerEl?.removeEventListener('touchstart', onTouchStart)
  snapContainerEl?.removeEventListener('touchmove', onTouchMove)
  snapContainerEl?.removeEventListener('touchend', onTouchEnd)
  setSnapContainer(null)
  observer?.disconnect()
})
</script>
