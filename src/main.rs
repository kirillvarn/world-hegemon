use macroquad::{prelude::*, ui::{hash, root_ui, widgets::{self, Group, Window}}};

mod country;
use country::Country;

use crate::country::CountryData;

const SCALE_FACTOR: f32 = 0.5;

#[derive(Default)]
struct State {
    info_panel: InfoPanel
}

#[derive(Default)]
struct InfoPanel {
    opened: bool,
    name: String
}

pub struct Asset {
    texture: Texture2D,
    image: Image,
    color: Color,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        window_resizable: false,
        window_width: 1024,
        window_height: 576,
        platform: miniquad::conf::Platform {
            swap_interval: Some(1),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn draw_fps() {
    draw_text(
        format!("{:.0} fps", 1.0 / (get_frame_time() as f32)).as_str(),
        0.0,
        13.0,
        20.0,
        RED,
    );
}

fn scale_image_bytes(src: &[u8], width: usize, height: usize) -> Vec<u8> {
    let new_w = (width as f32 * SCALE_FACTOR) as usize;
    let new_h = (height as f32 * SCALE_FACTOR) as usize;

    let mut dst = vec![0u8; new_w * new_h * 4];

    for y in 0..new_h {
        for x in 0..new_w {
            let src_i = ((y * 2) * width + (x * 2)) * 4;
            let dst_i = (y * new_w + x) * 4;
            dst[dst_i..dst_i + 4].copy_from_slice(&src[src_i..src_i + 4]);
        }
    }

    dst
}

fn is_colliding(image: &Image, mouse_pos: Vec2, tex_pos: Vec2) -> bool {
    let local_x = mouse_pos.x - tex_pos.x;
    let local_y = mouse_pos.y - tex_pos.y;

    if (local_x < 0. || local_y < 0.)
        || (local_x >= image.width as f32 || local_y >= image.height as f32)
    {
        return false;
    }

    let x = local_x as usize;
    let y = local_y as usize;
    let idx = (y * image.width as usize + x) * 4;

    let px = &image.bytes[idx..idx + 4];

    return px[3] > 0;
}

fn emit_event(country_el: &Country, state: &mut State) {
    if is_mouse_button_pressed(MouseButton::Left) {
        state.info_panel.name = country_el.name.clone();
        state.info_panel.opened = true;
    }
}

async fn load_scaled_texture(path: &str) -> Texture2D {
    let texture = load_texture(path).await.unwrap();
    let tex_data = texture.get_texture_data();

    let scaled_bytes = scale_image_bytes(&tex_data.bytes, tex_data.width(), tex_data.height());

    let image = Image {
        bytes: scaled_bytes,
        width: (tex_data.width() / 2) as u16,
        height: (tex_data.height() / 2) as u16,
    };

    Texture2D::from_image(&image)
}

async fn create_country(name: &str, position: Vec2, path: &str) -> Country {
    let tex = load_scaled_texture(path).await;
    let image = tex.get_texture_data();

    let result = Country {
        asset: Asset {
            texture: tex,
            image: image,
            color: WHITE,
        },
        name: String::from(name),
        data: CountryData::default(),
        position: position,
    };

    return result;
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state: State = State{ info_panel: InfoPanel::default() };

    let europe    = create_country("Europe", Vec2 { x: 344., y: 48.  }, "assets/EU.png").await;
    let africa    = create_country("Africa", Vec2 { x: 415., y: 185. }, "assets/AF.png").await;
    let asia      = create_country("Asia", Vec2 { x: 520., y: 50.  }, "assets/AS.png").await;
    let n_america = create_country("North America", Vec2 { x: 80.,  y: 50.  }, "assets/NA.png").await;
    let s_america = create_country("South America", Vec2 { x: 203., y: 277. }, "assets/SA.png").await;
    let australia = create_country("Australia", Vec2 { x: 850., y: 320. }, "assets/AU.png").await;

    let countries: [Country; 6] = [europe, africa, asia, n_america, s_america, australia];

    loop {
        clear_background(LIGHTGRAY);
        draw_fps();

        let (mouse_x, mouse_y) = mouse_position();

        let mouse_pos = Vec2 {
            x: mouse_x,
            y: mouse_y,
        };

        if state.info_panel.opened {
            widgets::Window::new(hash!(), vec2(400., 200.), vec2(320., 400.))
                .label("Info")
                .ui(&mut *root_ui(), |ui| {
                    ui.label(Vec2::new(200., 58.), &state.info_panel.name);
            });
        }

        for country_el in &countries {
            let asset = &country_el.asset;

            let collision = is_colliding(
                &asset.image,
                mouse_pos,
                country_el.position,
            );

            let mut col = asset.color;

            if collision {
                emit_event(&country_el, &mut state);
                col = BLUE;
            }

            draw_texture_ex(
                &asset.texture,
                country_el.position.x,
                country_el.position.y,
                col,
                DrawTextureParams {
                    dest_size: Some(vec2(asset.texture.width(), asset.texture.height())),
                    ..Default::default()
                },
            );
        }

        next_frame().await
    }
}

