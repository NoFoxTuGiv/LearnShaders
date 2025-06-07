use macroquad::prelude::*;

const VERT_SHADER: &str = include_str!("vert.glsl");
const FRAG_SHADER: &str = include_str!("frag.glsl");

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
