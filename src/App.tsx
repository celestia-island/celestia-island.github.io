// Root shell (converted from App.vue → TSX, 2026-08-30 house rule: no SFCs).
import { defineComponent } from 'vue'
import ThreeBackground from './components/ThreeBackground'
import NavBar from './components/NavBar'

export default defineComponent({
  name: 'App',
  setup() {
    return () => (
      <div class="relative min-h-screen">
        <ThreeBackground />
        <NavBar />
        <main class="relative z-10">
          <router-view />
        </main>
      </div>
    )
  },
})
