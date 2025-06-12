#version 100
precision lowp float;

varying lowp vec2 uv;

uniform sampler2D Texture;
uniform vec2 uResolution;
uniform float uTime;

vec3 palette(float t) {
  vec3 a = vec3(0.5, 0.5, 0.5);
  vec3 b = vec3(0.5, 0.5, 0.5);
  vec3 c = vec3(2.0, 1.0, 0.0);
  vec3 d = vec3(0.5, 0.2, 0.25);

  return a + b*cos( 6.283185*(c*t+d) );
}

void main() {
  // Normalize to -1 to 1
  vec2 cUv = (uv - 0.5) * 2.;
  vec2 uv0 = cUv;

  vec3 finalColor = vec3(0.);

  // flip y so 0,0 is in the bottom-left and account for aspect ratio
  cUv.y *= -1.;
  cUv.x *= uResolution.x / uResolution.y;

  for (float i = 0.; i < 3.; i++) {
    cUv = fract(cUv * 1.5) - 0.5;

    float d = length(cUv) * exp(-length(uv0));

    vec3 col = palette(length(uv0) + i * 0.4 + uTime * 0.4);

    d = sin(d * 8. + uTime) / 8.;
    d = abs(d);

    d = pow(0.01 / d, 1.2);

    finalColor += col * d;
  }

  gl_FragColor = vec4(finalColor, 1.);
}
