#define PI 3.14159265358979323846
#define TAU 6.28318530717958647692
#define COARSE_SAMPLES 16
#define REFINE_SAMPLES 8
#define STROKE_COUNT 30
#define RIBBON_W 0.075

precision highp float;

uniform float u_time;
uniform vec2 u_resolution;
uniform float u_scale;
varying vec2 vUv;

// Integer (sin-free) hashes - the original sin()-based hashes dominated the
// per-pixel ALU cost on mobile GPUs, and the noise quality is equivalent.
float hash(float n) {
    n = fract(n * 0.1031);
    n *= n + 33.33;
    n *= n + n;
    return fract(n);
}

float hash2(vec2 p) {
    vec3 p3 = fract(vec3(p.xyx) * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

float vnoise(vec2 p) {
    vec2 i = floor(p), f = fract(p);
    f = f * f * (3.0 - 2.0 * f);
    return mix(mix(hash2(i), hash2(i + vec2(1.0,0.0)), f.x),
               mix(hash2(i + vec2(0.0,1.0)), hash2(i + vec2(1.0,1.0)), f.x), f.y);
}

vec3 curve3D(float t) {
    float px = sin(t);
    float py = -sin(2.0 * t) * 0.18;
    
    float topLeftWeight = smoothstep(0.1, -0.9, px) * smoothstep(-0.05, 0.25, py);
    px -= 0.04 * topLeftWeight;
    py += 0.02 * topLeftWeight;
    
    float pz = cos(t);
    
    float twist_angle = px * 0.15;
    float cy = cos(twist_angle);
    float sy = sin(twist_angle);
    
    float new_py = py * cy - pz * sy;
    float new_pz = py * sy + pz * cy;
    py = new_py;
    pz = new_pz;
    
    return vec3(px * 0.75, py * 0.75, pz);
}

vec2 curvePt(float t) { return curve3D(t).xy; }

vec2 curveTan(float t) {
    float e = 0.001;
    return normalize(curvePt(t + e) - curvePt(t - e));
}

vec2 curveNor(float t) {
    vec2 tanDir = curveTan(t);
    return vec2(-tanDir.y, tanDir.x);
}

// Two-level search for the curve parameter nearest to p within [t0, t1]:
// a coarse pass finds the best sample, then a fine pass around it keeps the
// same precision as a dense full scan at a fraction of the cost (48 curve
// evaluations per pixel instead of 128).
float findNearestT(vec2 p, float t0, float t1) {
    float step = (t1 - t0) / float(COARSE_SAMPLES - 1);
    float best = t0;
    float bestD = dot(p - curvePt(t0), p - curvePt(t0));
    for (int i = 1; i < COARSE_SAMPLES; i++) {
        float t = t0 + step * float(i);
        vec2 cp = curvePt(t);
        float d = dot(p - cp, p - cp);
        if (d < bestD) { bestD = d; best = t; }
    }
    float lo = max(t0, best - step);
    float hi = min(t1, best + step);
    float fineStep = (hi - lo) / float(REFINE_SAMPLES - 1);
    for (int i = 0; i < REFINE_SAMPLES; i++) {
        float t = lo + fineStep * float(i);
        vec2 cp = curvePt(t);
        float d = dot(p - cp, p - cp);
        if (d < bestD) { bestD = d; best = t; }
    }
    return best;
}

vec3 getColor(float t, float pos, float seed) {
    float nt = fract(t * 2.0 / TAU);
    
    vec3 yellow = vec3(1.0, 0.9, 0.4);
    vec3 purple = vec3(0.6, 0.4, 0.9);
    vec3 cyan   = vec3(0.3, 0.6, 1.0);
    vec3 pink   = vec3(0.95, 0.5, 0.8);
    
    vec3 c;
    if (nt < 0.25) {
        c = mix(yellow, purple, smoothstep(0.05, 0.20, nt));
    } else if (nt < 0.5) {
        c = mix(purple, cyan, smoothstep(0.30, 0.45, nt));
    } else if (nt < 0.75) {
        c = mix(cyan, pink, smoothstep(0.55, 0.70, nt));
    } else {
        c = mix(pink, yellow, smoothstep(0.80, 0.95, nt));
    }
    
    float hueShift = (hash(seed * 11.2) - 0.5) * 0.08;
    c += hueShift;
    
    float brightness = 0.5 + 0.7 * hash(seed * 13.4);
    c *= brightness;
    
    float special = hash(seed * 15.3);
    if (special > 0.92) {
        c = vec3(1.0);
    } else if (special > 0.78) {
        c = vec3(0.15, 0.15, 0.2);
    }
    
    return clamp(c, 0.0, 1.0);
}

vec4 renderLayer(vec2 p, float base_t, float color_flow, float stroke_flow, float z_depth) {
    vec2 cp = curvePt(base_t);
    float distToCurve = length(p - cp);
    
    float cx = cos(base_t);
    float looseness = 1.0 - 0.20 * cx;
    float ribbon_width = RIBBON_W * looseness * (0.35 + 0.65 * pow(abs(cx), 2.0));
    
    if (distToCurve > ribbon_width * 1.5) return vec4(0.0);
    
    vec2 tanDir = curveTan(base_t);
    vec2 norDir = curveNor(base_t);
    
    vec3 layerRgb = vec3(0.0);
    float layerAlpha = 0.0;
    
    for (int i = 0; i < STROKE_COUNT; i++) {
        float seed = float(i) * 17.3 + 1.0;
        
        float rawPos = hash(seed);
        float pos = (rawPos - 0.5) * 2.0;
        pos = sign(pos) * pow(abs(pos), 1.3);
        
        float offset = pos * ribbon_width;
        
        float freq = floor(2.0 + hash(seed) * 5.0);
        float wobble = sin(base_t * freq + seed * 10.0) * 0.005;
        vec2 strokePt = cp + norDir * (offset + wobble);
        float d = length(p - strokePt);
        
        float strokeW = 0.0007 + hash(seed + 1.0) * 0.0014;
        if (d <= strokeW * 3.0) {
            float alpha = exp(-d * d / (strokeW * strokeW * 1.5));
            
            float num_segments = floor(2.0 + hash(seed * 4.4) * 3.0);
            float segment_phase = hash(seed * 5.5) * TAU;
            float envelope = sin(base_t * num_segments + segment_phase);
            float macro_dash = smoothstep(-0.3, 0.3, envelope);
            alpha *= macro_dash;
            
            vec2 seamless_uv = vec2(cos(base_t - stroke_flow), sin(base_t - stroke_flow)) * 30.0;
            float noise = vnoise(seamless_uv + vec2(pos * 50.0, seed * 11.0));
            float fine_noise = vnoise(p * 800.0);
            float breakPattern = smoothstep(0.2, 0.8, noise * 0.6 + fine_noise * 0.4 + 0.2);
            alpha *= breakPattern;
            
            float pressure = 0.4 + 0.6 * hash(seed + 3.0);
            alpha *= pressure;
            
            float depth_shade = mix(0.5, 1.0, z_depth * 0.5 + 0.5);
            vec3 col = getColor(base_t - color_flow, pos, seed) * depth_shade;
            
            float invA = 1.0 - alpha;
            layerRgb = col * alpha + layerRgb * invA;
            layerAlpha = alpha + layerAlpha * invA;
        }
    }
    
    layerAlpha = clamp(layerAlpha * 1.2, 0.0, 1.0);
    
    if (layerAlpha > 0.001) {
        layerRgb /= layerAlpha;
    }
    
    return vec4(layerRgb, layerAlpha);
}

// Anchor the infinity logo toward the bottom-left of the viewport and shrink
// it, reproducing the layout the pre-render-target build accidentally
// produced on high-DPR screens (its u_resolution was in CSS pixels while the
// drawing buffer was scaled by devicePixelRatio).
// LOGO_ANCHOR is in screen-fraction units (bottom-left origin); uv-space is
// x-normalized, so the y component must be scaled by res.y / res.x.
const vec2 LOGO_ANCHOR = vec2(-1.0 / 3.0, -1.0 / 3.0);
const float LOGO_ZOOM = 1.5;

void main() {
    vec2 res = u_resolution;
    vec2 uv = (gl_FragCoord.xy - 0.5 * res) / (res.x * 0.5);

    float anim = u_time;
    float breath = 1.0 + sin(anim * PI / 15.0) * 0.015;
    vec2 anchor = vec2(LOGO_ANCHOR.x, LOGO_ANCHOR.y * res.y / res.x);
    vec2 p = (uv - anchor) * LOGO_ZOOM / breath / u_scale;

    // Background early-out: the curve ribbon stays inside |y| < 0.45 and
    // |x| < 1.3 in p-space (the star rays and glow decay exponentially and
    // are invisible beyond that), so background pixels skip the per-pixel
    // curve search and stroke loops entirely (the dominant fragment cost).
    if (abs(p.y) > 0.45 || abs(p.x) > 1.3) {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 0.0);
        return;
    }

    float color_flow = anim * (PI / 30.0);
    float stroke_flow = anim * 0.15;
    
    float t1 = findNearestT(p, -PI / 2.0, PI / 2.0);
    float t2 = findNearestT(p, PI / 2.0, 3.0 * PI / 2.0);

    float z1 = curve3D(t1).z;
    float z2 = curve3D(t2).z;
    
    float t_front, t_back;
    if (z1 > z2) {
        t_front = t1; t_back = t2;
    } else {
        t_front = t2; t_back = t1;
    }
    
    vec4 backLayer = renderLayer(p, t_back, color_flow, stroke_flow, min(z1, z2));
    vec4 frontLayer = renderLayer(p, t_front, color_flow, stroke_flow, max(z1, z2));
    
    float shadow = smoothstep(0.0, 0.4, frontLayer.a);
    backLayer.rgb *= (1.0 - shadow * 0.4);
    
    float outAlpha = frontLayer.a + backLayer.a * (1.0 - frontLayer.a);
    vec3 outRgb = vec3(0.0);
    
    if (outAlpha > 0.0001) {
        outRgb = (frontLayer.rgb * frontLayer.a + backLayer.rgb * backLayer.a * (1.0 - frontLayer.a)) / outAlpha;
    }
    
    float dc = length(p);
    float cross_ray = exp(-abs(p.x) * 30.0 - abs(p.y) * 600.0) + exp(-abs(p.y) * 30.0 - abs(p.x) * 600.0);
    float diag_ray = exp(-abs(p.x + p.y) * 50.0 - abs(p.x - p.y) * 400.0) + exp(-abs(p.x - p.y) * 50.0 - abs(p.x + p.y) * 400.0);
    float core = exp(-dc * 60.0);
    
    float star = (cross_ray * 2.0 + diag_ray * 0.5 + core * 2.0);
    float blackHoleDist = abs(p.x) + abs(p.y);
    float blackHole = smoothstep(0.010, 0.025, blackHoleDist);
    star *= blackHole;
    
    float glow = exp(-dc * 8.0) * 0.2;
    star += glow * smoothstep(0.3, 0.0, dc) * blackHole;
    
    float twinkle = 1.0 + 0.15 * sin(anim * PI / 15.0 * 5.0);
    vec3 starColor = vec3(1.0, 0.98, 0.9) * star * twinkle;
    
    outRgb += starColor;
    outAlpha = clamp(outAlpha + star, 0.0, 1.0);
    
    if (outAlpha > 0.001) {
        outRgb = smoothstep(0.05, 0.95, outRgb);
        outRgb = pow(max(outRgb, 0.0), vec3(1.0 / 2.2));
        outRgb *= outAlpha;
    }
    
    if (outAlpha > 0.001) {
        outRgb /= outAlpha;
    }
    
    gl_FragColor = vec4(clamp(outRgb, 0.0, 1.0), outAlpha);
}
