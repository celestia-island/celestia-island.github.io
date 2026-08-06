<template>
  <div
    class="glass-card-static group cursor-default flex flex-col pb-3"
    :style="{ '--card-color': project.color }"
  >
    <div class="flex items-center gap-4 mb-4">
      <div
        class="w-14 h-14 rounded-xl flex items-center justify-center flex-shrink-0 overflow-hidden"
        :style="{ background: `${project.color}10` }"
      >
        <img
          v-if="logoUrl"
          :src="logoUrl"
          :alt="project.name"
          class="w-11 h-11 object-contain"
          draggable="false"
        />
        <div v-else :class="`${project.icon} w-6 h-6`" :style="{ color: project.color }" />
      </div>
      <div class="flex-1 min-w-0">
        <h3 class="text-lg font-semibold truncate text-primary">{{ t(project.shortLabelKey, project.shortLabel) }}</h3>
        <p class="text-xs font-medium mt-0.5 text-tertiary">{{ t(project.nameKey, project.name) }}</p>
        <p class="text-sm font-medium mt-1 truncate" :style="{ color: project.color }">
          {{ t(project.taglineKey, project.tagline) }}
        </p>
      </div>
    </div>

    <p class="text-sm leading-relaxed mb-4 line-clamp-3 text-secondary">
      {{ t(project.descriptionKey, project.description) }}
    </p>

    <div class="flex flex-wrap gap-1.5 content-start min-h-8">
      <span
        v-for="tech in project.techStack.slice(0, 3)"
        :key="tech"
        class="px-2 py-0.5 rounded-md text-xs font-medium"
        :style="{
          background: `${project.color}10`,
          color: project.color,
          border: `1px solid ${project.color}20`,
        }"
      >
        {{ tech }}
      </span>
    </div>

    <div class="flex items-center justify-between pt-2 mt-auto border-t border-subtle">
      <span
        class="text-xs px-2 py-0.5 rounded-full"
        :style="{
          background: statusBg,
          color: statusColor,
        }"
      >
        {{ t(`site.status.${project.status}`) }}
      </span>
      <a
        v-if="project.repo"
        :href="`https://github.com/${project.repo}`"
        target="_blank"
        class="github-link transition-colors duration-300 text-sm flex items-center gap-1 no-underline text-secondary"
      >
        <div class="i-lucide-external-link w-3.5 h-3.5" />
        GitHub
      </a>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import type { Project } from '@/types/project'

import entelecheiaLogo from '@res/logos/entelecheia.webp'
import shittimLogo from '@res/logos/shittim-chest.webp'
import evernightLogo from '@res/logos/evernight.webp'
import arisLogo from '@res/logos/aris.webp'
import keiLogo from '@res/logos/kei.webp'
import tairitsuLogo from '@res/logos/tairitsu.webp'
import hikariLogo from '@res/logos/hikari.webp'
import lagrangeLogo from '@res/logos/lagrange.webp'
import aobaLogo from '@res/logos/aoba.webp'
import kirinoLogo from '@res/logos/kirino.webp'
import ratatuiMarkdownLogo from '@res/logos/ratatui-markdown.webp'
import yuukaLogo from '@res/logos/yuuka.webp'
import ichikaLogo from '@res/logos/ichika.webp'
import hifumiLogo from '@res/logos/hifumi.webp'
import noaLogo from '@res/logos/noa.webp'
import fuukaLogo from '@res/logos/fuuka.webp'
import kouLogo from '@res/logos/kou.webp'
import malkuthLogo from '@res/logos/malkuth.webp'
import seiaLogo from '@res/logos/seia.webp'
import shirabeLogo from '@res/logos/shirabe.webp'

const props = defineProps<{
  project: Project
}>()

const { t } = useI18n()

const logoMap: Record<string, string> = {
  entelecheia: entelecheiaLogo,
  'shittim-chest': shittimLogo,
  evernight: evernightLogo,
  aris: arisLogo,
  kei: keiLogo,
  tairitsu: tairitsuLogo,
  hikari: hikariLogo,
  lagrange: lagrangeLogo,
  aoba: aobaLogo,
  kirino: kirinoLogo,
  'ratatui-markdown': ratatuiMarkdownLogo,
  yuuka: yuukaLogo,
  ichika: ichikaLogo,
  hifumi: hifumiLogo,
  noa: noaLogo,
  fuuka: fuukaLogo,
  kou: kouLogo,
  malkuth: malkuthLogo,
  seia: seiaLogo,
  shirabe: shirabeLogo,
}

const logoUrl = computed(() => logoMap[props.project.id])

const statusMap: Record<string, { bg: string; color: string }> = {
  active: { bg: 'rgba(34, 197, 94, 0.1)', color: '#22c55e' },
  stable: { bg: 'rgba(59, 130, 246, 0.1)', color: '#3b82f6' },
  research: { bg: 'rgba(245, 158, 11, 0.1)', color: '#f59e0b' },
  legacy: { bg: 'rgba(156, 163, 175, 0.1)', color: '#9ca3af' },
  preview: { bg: 'rgba(168, 85, 247, 0.1)', color: '#a855f7' },
}

const statusStyle = computed(() => statusMap[props.project.status] || statusMap.stable)
const statusBg = computed(() => statusStyle.value.bg)
const statusColor = computed(() => statusStyle.value.color)
</script>

<style scoped>
.github-link:hover {
  color: var(--text-primary);
}
</style>
