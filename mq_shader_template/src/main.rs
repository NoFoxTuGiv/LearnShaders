use macroquad::{miniquad::window::screen_size, prelude::*};

const VERT_SHADER: &str = include_str!("vert.glsl");
const FRAG_SHADER: &str = include_str!("frag.glsl");
const WIDTH: i32 = 640; // Default: 640
const HEIGHT: i32 = 360; // Default: 360

/// Entry point of the macroquad application.
/// This function sets up the rendering context, loads a GLSL shader, and enters the main game loop.
#[macroquad::main(window_conf())]
async fn main() {
    let (width, height) = screen_size();
    // Set up a render target to paint to off-screen
    let render_target = render_target(WIDTH as u32, HEIGHT as u32);
    render_target.texture.set_filter(FilterMode::Nearest);

    // Load a custom shader from GLSL source strings.
    // This creates a material that will use our provided vertex and fragment shaders.
    let mat_shader = load_material(
        ShaderSource::Glsl {
            vertex: VERT_SHADER,
            fragment: FRAG_SHADER,
        },
        MaterialParams {
            uniforms: vec!(
                UniformDesc::new("uResolution", UniformType::Float2),
                UniformDesc::new( "uTime", UniformType::Float1),
            ),
            ..Default::default()
        }
    )
    .unwrap(); // Panics on shader compile error.

    // Shader Uniforms
    mat_shader.set_uniform("uResolution", (width, height));

    loop {
        // == Drawing to the screen
        clear_background(DARKGRAY);

        // == Computed Uniforms ==
        mat_shader.set_uniform("uTime", get_time() as f32);

        // === Begin custom GLSL shader ===
        gl_use_material(&mat_shader); // Activate our custom shader material.

        // Draw render target as texture to screen with our shader applied
        draw_texture_ex(
            &render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(WIDTH as f32, HEIGHT as f32)),
                ..Default::default()
            },
        );

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

fn window_conf() -> Conf {
    Conf {
        window_title: "Simple Shader Template".to_owned(),
        window_width: WIDTH,
        window_height: HEIGHT,
        ..Default::default()
    }
}
