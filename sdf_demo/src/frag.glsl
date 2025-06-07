#version 100
precision lowp float;

varying vec2 uv;

void main() {
  gl_FragColor = vec4(uv.x, 0., 1., 1.);
}
