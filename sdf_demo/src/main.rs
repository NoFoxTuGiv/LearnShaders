use macroquad::prelude::*;

const VERT_SHADER: &str = include_str!("vert.glsl");
const FRAG_SHADER: &str = include_str!("frag.glsl");

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
    )
    .unwrap();

    loop {
        clear_background(DARKGRAY);

        gl_use_material(&mat_shader);
        draw_rectangle(0., 0., screen_width(), screen_height(), WHITE);
        gl_use_default_material();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Signed Distance Functions".to_owned(),
        ..Default::default()
    }
}
