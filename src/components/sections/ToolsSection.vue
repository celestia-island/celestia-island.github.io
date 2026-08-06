<template>
  <section id="tools" class="snap-section" ref="el">
    <div class="section-inner">
      <div class="max-w-6xl mx-auto w-full">
        <div class="text-center mb-6 reveal" :class="{ 'is-visible': visible }">
          <h2 class="text-4xl sm:text-5xl font-bold tracking-tight">
            <span class="bg-gradient-to-r from-emerald-400 to-cyan-400 bg-clip-text text-transparent">
              {{ t('group.decagrammaton') }}
            </span>
          </h2>
          <p class="text-xs sm:text-sm font-medium tracking-widest uppercase mt-2 text-tertiary">{{ t('group.tools') }}</p>
          <p class="text-sm mt-3 max-w-lg mx-auto text-secondary">{{ t('group.toolsDesc') }}</p>
        </div>
        <CardScroller :items="items" :visible="visible" />
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useReveal } from '@/composables/useReveal'
import { projects } from '@/types/project'
import CardScroller from '@/components/CardScroller.vue'

const { t } = useI18n()
const { visible, triggerReveal } = useReveal()
const el = ref<HTMLElement>()

const items = projects.filter(p => ['aoba', 'kirino', 'ratatui-markdown', 'yuuka', 'ichika', 'hifumi', 'noa', 'fuuka', 'malkuth', 'seia', 'aris', 'kei'].includes(p.id))

defineExpose({ el, triggerReveal })
</script>
