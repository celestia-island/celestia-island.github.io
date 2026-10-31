
/// <reference types="vite/client" />

declare module '*.vue' {
  import type { DefineComponent } from 'vue'
  /** SFC default export. The wildcard cannot know a component's props / emits
   *  / data, so the honest expression is "an object with no declared members";
   *  `any` there made every imported SFC accept arbitrary props unchecked. */
  type SfcComponent = DefineComponent<
    Record<string, never>,
    Record<string, never>,
    Record<string, never>
  >
  const component: SfcComponent
  export default component
}

declare module '*.vert?raw' {
  const value: string
  export default value
}

declare module '*.frag?raw' {
  const value: string
  export default value
}

declare module '*.png' {
  const value: string
  export default value
}

declare module '*.webp' {
  const value: string
  export default value
}
