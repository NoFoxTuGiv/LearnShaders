use macroquad::prelude::*;

const VERT_SHADER: &str = include_str!("neon.frag");
const FRAG_SHADER: &str = include_str!("neon.vert");

#[macroquad::main(window_conf())]
async fn main() {
    let mat_shader = load_material(
        ShaderSource::Glsl {
            vertex: VERT_SHADER,
            fragment: FRAG_SHADER,
        },
        MaterialParams {
            ..Default::default()
        },
    ).unwrap();

    loop {
        clear_background(DARKGRAY);

        // === Begin custom GLSL Shader ===
        gl_use_material(&mat_shader);

        draw_rectangle(
            screen_width() / 2.,
            screen_height() / 2.,
            100.,
            100.,
            WHITE,
        );

        gl_use_default_material();
        // === End custom GLSL Shader ===
        
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Neon Shader Test".to_owned(),
        ..Default::default()
    }
}
