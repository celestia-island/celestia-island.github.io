<template>
  <div ref="containerRef" class="fixed inset-0 z-0 pointer-events-none" aria-hidden="true" />
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onBeforeUnmount } from 'vue'
import { useTheme } from '@/composables/useTheme'
import { WebGLRenderer } from 'three/src/renderers/WebGLRenderer.js'
import { Scene } from 'three/src/scenes/Scene.js'
import { OrthographicCamera } from 'three/src/cameras/OrthographicCamera.js'
import { Mesh } from 'three/src/objects/Mesh.js'
import { ShaderMaterial } from 'three/src/materials/ShaderMaterial.js'
import { MeshBasicMaterial } from 'three/src/materials/MeshBasicMaterial.js'
import { PointsMaterial } from 'three/src/materials/PointsMaterial.js'
import { Points } from 'three/src/objects/Points.js'
import { PlaneGeometry } from 'three/src/geometries/PlaneGeometry.js'
import { BufferGeometry } from 'three/src/core/BufferGeometry.js'
import { BufferAttribute } from 'three/src/core/BufferAttribute.js'
import { WebGLRenderTarget } from 'three/src/renderers/WebGLRenderTarget.js'
import { CanvasTexture } from 'three/src/textures/CanvasTexture.js'
import { TextureLoader } from 'three/src/loaders/TextureLoader.js'
import { Vector2 } from 'three/src/math/Vector2.js'
import { AdditiveBlending, NormalBlending, LinearFilter } from 'three/src/constants.js'
import logoVert from '../../shaders/logo.vert?raw'
import logoFrag from '../../shaders/logo.frag?raw'
import logoLightFrag from '../../shaders/logo-light.frag?raw'
import entelecheiaLogo from '@res/logos/entelecheia.webp'

const containerRef = ref<HTMLDivElement>()
const { theme } = useTheme()

let renderer: WebGLRenderer
let scene: Scene
let camera: OrthographicCamera
let logoMesh: Mesh
let logoMaterial: ShaderMaterial
let lightMaterial: ShaderMaterial
let starMaterial: PointsMaterial
let twinkleMaterial: PointsMaterial
let starPoints: Points
let starTwinkle: Points
let halfTarget: WebGLRenderTarget
let blitScene: Scene
let blitMesh: Mesh
let animationId: number
let startTime = 0
let visible = true
let observer: IntersectionObserver

watch(theme, (t) => {
  if (!logoMesh) return
  if (t === 'dark') {
    logoMesh.material = logoMaterial
    starPoints.visible = true
    starTwinkle.visible = true
  } else {
    logoMesh.material = lightMaterial
    starPoints.visible = false
    starTwinkle.visible = false
  }
})

function createStarfield() {
  const count = 1200
  const positions = new Float32Array(count * 3)

  for (let i = 0; i < count; i++) {
    const theta = Math.random() * Math.PI * 2
    const phi = Math.acos(2 * Math.random() - 1)
    const r = 3 + Math.random() * 2
    positions[i * 3] = r * Math.sin(phi) * Math.cos(theta)
    positions[i * 3 + 1] = r * Math.sin(phi) * Math.sin(theta)
    positions[i * 3 + 2] = -r * Math.cos(phi)
  }

  const geometry = new BufferGeometry()
  geometry.setAttribute('position', new BufferAttribute(positions, 3))

  const canvas = document.createElement('canvas')
  canvas.width = 64
  canvas.height = 64
  const ctx = canvas.getContext('2d')!
  const gradient = ctx.createRadialGradient(32, 32, 0, 32, 32, 32)
  gradient.addColorStop(0, 'rgba(255, 255, 255, 1)')
  gradient.addColorStop(0.15, 'rgba(200, 200, 255, 0.6)')
  gradient.addColorStop(0.5, 'rgba(140, 140, 220, 0.15)')
  gradient.addColorStop(1, 'rgba(0, 0, 0, 0)')
  ctx.fillStyle = gradient
  ctx.fillRect(0, 0, 64, 64)

  const texture = new CanvasTexture(canvas)
  texture.generateMipmaps = false
  texture.minFilter = LinearFilter
  texture.magFilter = LinearFilter

  starMaterial = new PointsMaterial({
    size: 0.035,
    map: texture,
    blending: AdditiveBlending,
    depthWrite: false,
    depthTest: false,
    transparent: true,
    opacity: 0.45,
  })

  starPoints = new Points(geometry, starMaterial)
  scene.add(starPoints)

  const count2 = 600
  const positions2 = new Float32Array(count2 * 3)
  for (let i = 0; i < count2; i++) {
    const theta = Math.random() * Math.PI * 2
    const phi = Math.acos(2 * Math.random() - 1)
    const r = 5 + Math.random() * 3
    positions2[i * 3] = r * Math.sin(phi) * Math.cos(theta)
    positions2[i * 3 + 1] = r * Math.sin(phi) * Math.sin(theta)
    positions2[i * 3 + 2] = -r * Math.cos(phi)
  }
  const geometry2 = new BufferGeometry()
  geometry2.setAttribute('position', new BufferAttribute(positions2, 3))

  const canvas2 = document.createElement('canvas')
  canvas2.width = 32
  canvas2.height = 32
  const ctx2 = canvas2.getContext('2d')!
  const gradient2 = ctx2.createRadialGradient(16, 16, 0, 16, 16, 16)
  gradient2.addColorStop(0, 'rgba(160, 160, 255, 0.7)')
  gradient2.addColorStop(0.5, 'rgba(100, 100, 200, 0.25)')
  gradient2.addColorStop(1, 'rgba(0, 0, 0, 0)')
  ctx2.fillStyle = gradient2
  ctx2.fillRect(0, 0, 32, 32)

  twinkleMaterial = new PointsMaterial({
    size: 0.022,
    map: new CanvasTexture(canvas2),
    blending: AdditiveBlending,
    depthWrite: false,
    depthTest: false,
    transparent: true,
    opacity: 0.28,
  })

  starTwinkle = new Points(geometry2, twinkleMaterial)
  scene.add(starTwinkle)
}

function createLogoPlane() {
  const geometry = new PlaneGeometry(2.4, 2.4)

  logoMaterial = new ShaderMaterial({
    uniforms: {
      u_time: { value: 0 },
      u_resolution: { value: new Vector2(window.innerWidth, window.innerHeight) },
      u_scale: { value: 1.0 },
    },
    vertexShader: logoVert,
    fragmentShader: logoFrag,
    transparent: true,
    depthTest: false,
    depthWrite: false,
    blending: NormalBlending,
  })

  const lightTex = new TextureLoader().load(entelecheiaLogo)
  lightTex.generateMipmaps = false
  lightTex.minFilter = LinearFilter
  lightTex.magFilter = LinearFilter

  lightMaterial = new ShaderMaterial({
    uniforms: {
      u_time: { value: 0 },
      u_resolution: { value: new Vector2(window.innerWidth, window.innerHeight) },
      u_scale: { value: 1.0 },
      u_texture: { value: lightTex },
    },
    vertexShader: logoVert,
    fragmentShader: logoLightFrag,
    transparent: true,
    depthTest: false,
    depthWrite: false,
    blending: NormalBlending,
  })

  logoMesh = new Mesh(geometry, theme.value === 'dark' ? logoMaterial : lightMaterial)
  logoMesh.position.z = -1
  scene.add(logoMesh)
}

function init() {
  if (!containerRef.value) return

  startTime = performance.now()

  renderer = new WebGLRenderer({ alpha: true, antialias: false, powerPreference: 'low-power' })
  // Cap the render resolution: touch devices get a lower DPR ceiling than
  // desktops (the infinity logo fragment shader is fill-rate bound, and
  // phones often report DPR 2-3).
  const isCoarsePointer = window.matchMedia('(pointer: coarse)').matches
  renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, isCoarsePointer ? 1.25 : 1.5))
  renderer.setSize(window.innerWidth, window.innerHeight)
  renderer.setClearColor(0x000000, 0)
  containerRef.value.appendChild(renderer.domElement)

  scene = new Scene()
  camera = new OrthographicCamera(-1, 1, 1, -1, 0.1, 10)
  camera.position.z = 1

  createStarfield()
  createLogoPlane()

  // The infinity-logo fragment shader is fill-rate bound, so the whole scene
  // is rendered at half resolution and upscaled with a cheap fullscreen blit.
  // The soft glowing logo and star sprites lose no perceptible detail.
  const halfW = Math.max(1, Math.round(window.innerWidth / 2))
  const halfH = Math.max(1, Math.round(window.innerHeight / 2))
  halfTarget = new WebGLRenderTarget(halfW, halfH, {
    minFilter: LinearFilter,
    magFilter: LinearFilter,
    depthBuffer: false,
    stencilBuffer: false,
  })
  blitScene = new Scene()
  blitMesh = new Mesh(
    new PlaneGeometry(2, 2),
    new MeshBasicMaterial({ map: halfTarget.texture })
  )
  blitScene.add(blitMesh)

  if (theme.value !== 'dark') {
    starPoints.visible = false
    starTwinkle.visible = false
  }

  logoMaterial.uniforms.u_resolution.value.set(window.innerWidth, window.innerHeight)

  updateScale()

  animate()
}

function animate() {
  animationId = requestAnimationFrame(animate)

  if (!visible) return

  if (theme.value === 'dark') {
    const t = (performance.now() - startTime) / 1000

    logoMaterial.uniforms.u_time.value = t

    starPoints.rotation.y += 0.00018
    starPoints.rotation.x += 0.00006
    starMaterial.opacity = 0.38 + 0.12 * Math.sin(t * 0.7)

    starTwinkle.rotation.y -= 0.00012
    starTwinkle.rotation.z += 0.00004
    twinkleMaterial.opacity = 0.22 + 0.12 * Math.sin(t * 0.5 + 1)
  } else {
    const t = (performance.now() - startTime) / 1000
    lightMaterial.uniforms.u_time.value = t
  }

  // The logo shader derives p-space from gl_FragCoord vs u_resolution, so it
  // must match the render-target size (not the full-res canvas).
  const rtW = halfTarget.width
  const rtH = halfTarget.height
  logoMaterial.uniforms.u_resolution.value.set(rtW, rtH)
  lightMaterial.uniforms.u_resolution.value.set(rtW, rtH)

  renderer.setRenderTarget(halfTarget)
  renderer.render(scene, camera)
  renderer.setRenderTarget(null)
  renderer.render(blitScene, camera)
}

function getScale(w: number, h: number) {
  const aspect = w / h
  if (aspect < 1.0) return Math.min(5.0, 1.6 / aspect)
  return 1.0
}

function updateScale() {
  if (!renderer || !logoMaterial || !lightMaterial) return
  const s = getScale(window.innerWidth, window.innerHeight)
  logoMaterial.uniforms.u_scale.value = s
  lightMaterial.uniforms.u_scale.value = s
}

function onResize() {
  if (!renderer) return
  renderer.setSize(window.innerWidth, window.innerHeight)
  if (halfTarget) {
    halfTarget.setSize(Math.max(1, Math.round(window.innerWidth / 2)), Math.max(1, Math.round(window.innerHeight / 2)))
  }
  updateScale()
}

onMounted(() => {
  init()
  window.addEventListener('resize', onResize)

  observer = new IntersectionObserver(
    ([entry]) => { visible = entry.isIntersecting },
    { threshold: 0 }
  )
  if (containerRef.value) observer.observe(containerRef.value)
})

onBeforeUnmount(() => {
  cancelAnimationFrame(animationId)
  window.removeEventListener('resize', onResize)
  observer?.disconnect()
  if (renderer) {
    renderer.dispose()
    containerRef.value?.removeChild(renderer.domElement)
  }
  halfTarget?.dispose()
  blitMesh?.geometry.dispose()
  ;(blitMesh?.material as MeshBasicMaterial | undefined)?.dispose()
  starMaterial?.map?.dispose()
  starPoints?.geometry.dispose()
  starMaterial?.dispose()
  twinkleMaterial?.map?.dispose()
  starTwinkle?.geometry.dispose()
  twinkleMaterial?.dispose()
  logoMaterial?.dispose()
  lightMaterial?.uniforms?.u_texture?.value?.dispose()
  lightMaterial?.dispose()
  logoMesh?.geometry.dispose()
})
</script>
