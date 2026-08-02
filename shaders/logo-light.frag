precision highp float;

uniform float u_time;
uniform vec2 u_resolution;
uniform float u_scale;
uniform sampler2D u_texture;

varying vec2 vUv;

// See logo.frag: bottom-left anchor + zoom, matching the legacy high-DPR look.
// LOGO_ANCHOR is in screen-fraction units; uv-space is x-normalized, so the
// y component must be scaled by res.y / res.x.
const vec2 LOGO_ANCHOR = vec2(-1.0 / 3.0, -1.0 / 3.0);
const float LOGO_ZOOM = 1.5;

void main() {
    vec2 res = u_resolution;
    vec2 uv = (gl_FragCoord.xy - 0.5 * res) / (res.x * 0.5);
    uv = vec2(uv.x - LOGO_ANCHOR.x, uv.y - LOGO_ANCHOR.y * res.y / res.x) * LOGO_ZOOM;

    float breath = 1.0 + sin(u_time * 3.14159265 / 15.0) * 0.02;
    uv = uv / breath;

    float anim = u_time;

    vec2 texUv = uv / u_scale;
    texUv.x = texUv.x * 0.55 + 0.5;
    texUv.y = texUv.y * 0.55 + 0.5;

    if (texUv.x < 0.0 || texUv.x > 1.0 || texUv.y < 0.0 || texUv.y > 1.0) {
        gl_FragColor = vec4(0.0);
        return;
    }

    vec4 texColor = texture2D(u_texture, texUv);

    float alpha = 0.10 + 0.06 * (0.5 + 0.5 * sin(anim * 3.14159265 / 15.0));

    gl_FragColor = vec4(texColor.rgb, texColor.a * alpha);
}
