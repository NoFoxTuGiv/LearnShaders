use macroquad::prelude::*;

/// Entry point of the macroquad application.
/// This function sets up the rendering context, loads a GLSL shader, and enters the main game loop.
#[macroquad::main("mq minimal shader template")]
async fn main() {
    // Load a custom shader from GLSL source strings.
    // This creates a material that will use our provided vertex and fragment shaders.
    let mat_shader = load_material(
        ShaderSource::Glsl {
            vertex: VERT_SHADER,
            fragment: FRAG_SHADER,
        },
        // Here we use default material parameters — no custom textures, blending, or uniforms.
        MaterialParams {
            ..Default::default()
        },
    )
    .unwrap(); // Panics on shader compile error.

    loop {
        // If functional, this should not be visible.
        clear_background(DARKGRAY);
        draw_rectangle_ex(
            screen_width() / 2.,
            screen_height() / 2.,
            100.,
            100.,
            DrawRectangleParams {
                offset: vec2(0.5, 0.5),
                rotation: (0.),
                color: (DARKPURPLE),
            },
        );

        // === Begin custom GLSL shader ===
        gl_use_material(&mat_shader); // Activate our custom shader material.

        // Draw a full-screen white rectangle.
        // This geometry becomes the input for our fragment shader, effectively letting us "paint" the whole screen.
        draw_rectangle(0., 0., screen_width(), screen_height(), WHITE);

        // Revert to the default shader so subsequent draw calls are unaffected.
        gl_use_default_material();
        // === End custom GLSL shader ===

        // Exit the application when the Escape key is pressed.
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}

/// A simple fragment shader written in GLSL ES 1.00 (used in WebGL and OpenGL ES 2.0).
///
/// This shader runs for every pixel in the rendered geometry and outputs a solid white color.
/// `precision lowp float;` defines float precision (mandatory in WebGL).
const FRAG_SHADER: &str = r#"#version 100
precision lowp float;

void main() {
  gl_FragColor = vec4(1.);
}
"#;

/// A basic vertex shader in GLSL ES 1.00 compatible syntax.
///
/// It transforms input geometry from object space into clip space using the `Model` and `Projection` matrices.
/// Macroquad automatically provides these uniforms and vertex attributes.
const VERT_SHADER: &str = "#version 100
attribute vec3 position;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1); 
}
";
