// Fullscreen blit of the offscreen scene target onto the canvas.
//
// The target is rendered with the regular alpha blending into a transparent
// clear, so it already holds *premultiplied* color, and the logo shaders emit
// display-encoded color by themselves (logo.frag applies pow(1/2.2) at the end;
// logo-light.frag samples a texture whose texels are display-encoded already).
// This pass therefore has to be a plain pass-through:
//
//   * Sampling the target through a built-in material re-encodes the color with
//     the output color space (<colorspace_fragment>) — a second gamma encode on
//     top of the shaders' own one.
//   * Blending that material with SRC_ALPHA multiplies by alpha a second time,
//     because the source color is premultiplied (dst.rgb ends up as color*a^2).
//
// Together they washed the themed logo out. In the light theme, where the logo
// alpha is only ~0.1, only about a third of the mark's hue survived (a ~10/255
// channel spread over the ~230/255 white veil of the page background), so the
// colorful infinity showed up as a neutral gray silhouette; the dark theme's
// ribbon, already display-encoded by its own shader, was brightened past what
// that shader intends. The premultiplied source blend (ONE / ONE_MINUS_SRC_ALPHA)
// is configured on the material in ThreeBackground.tsx.
//
// This pass-through is correct as long as every material rendered into the
// target premultiplies its color — every one of them blends with alpha into the
// transparent clear, which does.

uniform sampler2D u_map;

varying vec2 vUv;

void main() {
  gl_FragColor = texture2D(u_map, vUv);
}
