#version 100
precision mediump float;

uniform sampler2D u_texture;    // your shapes RT or texture
uniform vec2      u_resolution; // (screen_width, screen_height)
uniform float     u_threshold;  // e.g. 0.8 for fairly bright only
uniform float     u_radius;     // in pixels
uniform int       u_samples;    // e.g. 8 or 12

varying vec2 v_uv;

void main() {
    // 1) original color
    vec4 orig = texture2D(u_texture, v_uv);

    // 2) bright‐pass
    vec4 bright = max(orig - u_threshold, vec4(0.0));

    // 3) radial blur
    vec4 bloom = vec4(0.0);
    float angleStep = 6.2831853 / float(u_samples);
    vec2 px = vec2(u_radius) / u_resolution;
    for(int i = 0; i < 32; i++) {            // 32 is max; we'll break early
        if(i >= u_samples) break;
        float angle = angleStep * float(i);
        vec2 dir = vec2(cos(angle), sin(angle));
        vec4 sampleCol = texture2D(u_texture, v_uv + dir * px);
        bloom += max(sampleCol - u_threshold, vec4(0.0));
    }
    bloom /= float(u_samples);

    // 4) composite
    gl_FragColor = orig + bloom;
}
