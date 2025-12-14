precision highp float;

uniform float uTime;
uniform float uIntensity;
varying vec2 vUv;

/* Simple grid */
float grid(vec2 uv, float scale) {
  vec2 gv = fract(uv * scale);
  float lineX = step(gv.x, 0.02) + step(1.0 - gv.x, 0.02);
  float lineY = step(gv.y, 0.02) + step(1.0 - gv.y, 0.02);
  return clamp(lineX + lineY, 0.0, 1.0);
}

void main() {
  float g = grid(vUv, 18.0);

  // Scan pulse
  float pulse = sin((vUv.y + uTime * 0.4) * 20.0) * 0.5 + 0.5;
  pulse *= uIntensity;

  // EDGE FALLOFF (this is the key)
  float edgeX = smoothstep(0.0, 0.15, vUv.x) *
                smoothstep(0.0, 0.15, 1.0 - vUv.x);
  float edgeY = smoothstep(0.0, 0.15, vUv.y) *
                smoothstep(0.0, 0.15, 1.0 - vUv.y);

  float edgeMask = edgeX * edgeY;

  vec3 color = vec3(0.9, 0.2, 0.25);

  float alpha = g * (0.18 + pulse * 0.35) * edgeMask;

  gl_FragColor = vec4(color, alpha);
}