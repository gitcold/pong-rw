use macroquad::audio::*;
use macroquad::prelude::*;
use wasm_bindgen::prelude::*;

const MUSIC_BYTES: &[u8] = include_bytes!("../assets/bgm.ogg");
const FONT_BYTES: &[u8] = include_bytes!("../assets/nova-round.ttf");

const LOGICAL_WIDTH: f32 = 400.0;
const LOGICAL_HEIGHT: f32 = 300.0;

const PADDLE_WIDTH: f32 = 60.0;
const PADDLE_HEIGHT: f32 = 10.0;
const PADDLE_Y: f32 = 260.0;
const PADDLE_VEC: f32 = 240.0;

const BALL_SIZE: f32 = 10.0;
const BALL_VEL_INIT: f32 = 150.0;
const BALL_VEL_MAX: f32 = 400.0;

const POINT_POS: Vec2 = Vec2::new(LOGICAL_WIDTH / 2.0 - 40.0, 20.0);
const TEXT_LOSE_POS: Vec2 = Vec2::new(LOGICAL_WIDTH / 2.0 - 60.0, LOGICAL_HEIGHT / 2.0);
const TEXT_LOSE_CONTEXT: &str = "You lose!";

const BUTTON_RESTART_POS: Vec2 = Vec2::new(
    LOGICAL_WIDTH / 2.0 - BUTTON_RESTART_WIDTH / 2.0,
    LOGICAL_HEIGHT / 2.0 + 20.0,
);
const BUTTON_RESTART_WIDTH: f32 = 80.0;
const BUTTON_RESTART_HEIGHT: f32 = 30.0;

enum GameState {
    Playing,
    Gameover,
}

// 计算缩放比例和黑边偏移
fn get_scale_and_offset() -> (f32, Vec2) {
    let win_w = screen_width();
    let win_h = screen_height();
    let scale_x = win_w / LOGICAL_WIDTH;
    let scale_y = win_h / LOGICAL_HEIGHT;
    let scale = scale_x.min(scale_y);
    let offset_x = (win_w - LOGICAL_WIDTH * scale) / 2.0;
    let offset_y = (win_h - LOGICAL_HEIGHT * scale) / 2.0;
    (scale, vec2(offset_x, offset_y))
}

// 将屏幕坐标转换为逻辑坐标
fn screen_to_world(screen_x: f32, screen_y: f32, scale: f32, offset: Vec2) -> (f32, f32) {
    let world_x = (screen_x - offset.x) / scale;
    let world_y = (screen_y - offset.y) / scale;
    (
        world_x.clamp(0.0, LOGICAL_WIDTH),
        world_y.clamp(0.0, LOGICAL_HEIGHT),
    )
}

#[macroquad::main("pongrw")]
async fn main() {
    request_new_screen_size(LOGICAL_WIDTH, LOGICAL_HEIGHT);

    let my_font = load_ttf_font_from_bytes(FONT_BYTES).expect("none font:无法加载字体");
    let bgm = load_sound_from_bytes(MUSIC_BYTES).await.unwrap();
    play_sound(
        &bgm,
        PlaySoundParams {
            looped: true,
            volume: 1.0,
        },
    );

    let mut paddle_x = LOGICAL_WIDTH / 2.0 - PADDLE_WIDTH / 2.0;
    let mut game_state = GameState::Playing;
    let mut point = 0;
    let mut ball_pos = vec2(
        rand::gen_range(BALL_SIZE, LOGICAL_WIDTH - BALL_SIZE),
        BALL_SIZE * 2.0,
    );
    let mut ball_vel = vec2(
        rand::gen_range(-BALL_VEL_INIT, BALL_VEL_INIT) * 2.0,
        rand::gen_range(BALL_VEL_INIT * 0.5, BALL_VEL_INIT),
    );
    /*
        let mut camera = Camera2D::from_display_rect(
            Rect::new(0.0, 0.0, LOGICAL_WIDTH, LOGICAL_HEIGHT),
            Rect::new(0.0, 0.0, screen_width(), screen_height()),
        );

        set_window_resize_callback(|w, h| {
            camera = Camera2D::from_display_rect(
                Rect::new(0.0, 0.0, LOGICAL_WIDTH, LOGICAL_HEIGHT),
                Rect::new(0.0, 0.0, w as f32, h as f32),
            );
        });
    */
    loop {
        let dt = get_frame_time();
        //let (scale, offset) = get_scale_and_offset();
        println!("{}", ball_vel);
        /*let camera = Camera2D {
            zoom: vec2(scale, scale),
            target: vec2(LOGICAL_WIDTH / 2.0, LOGICAL_HEIGHT / 2.0),
            offset: vec2(offset.x - screen_width() / 2.0, offset.y - screen_height() / 2.0),
            ..Default::default()
        };
        set_camera(&camera);
        */
        /*let camera = Camera2D {
            zoom: vec2(1., 1.),
            target: vec2(0.0, 0.5),
            //offset: vec2(offset.x - screen_width() / 2.0, offset.y - screen_height() / 2.0),
            ..Default::default()
        };
        set_camera(&camera);*/

        // 使用逻辑中心点作为target，保持物理屏幕居中
        //let target = vec2(LOGICAL_WIDTH / 2.0, LOGICAL_HEIGHT / 2.0);
        //let target = vec2(0.0, 0.0);
        /*let offset = vec2(
            (screen_width() - LOGICAL_WIDTH * scale) / 2.0,
            (screen_height() - LOGICAL_HEIGHT * scale) / 2.0,
        );*/

        // 修正后的相机设置
        /*let camera = Camera2D {
            target,
            zoom: vec2(scale, scale),
            offset,
            ..Default::default()
        };
        set_camera(&camera);*/
        let camera = Camera2D {
            target: vec2(LOGICAL_WIDTH / 2.0, LOGICAL_HEIGHT / 2.0),
            zoom: vec2(1.0 / 200.0, 1.0 / 150.0),
            //offset: vec2(0.0, 0.0),
            ..Default::default()
        };
        set_camera(&camera);
        if let GameState::Playing = game_state {
            if is_mouse_button_down(MouseButton::Left) {
                if mouse_position().0 - (paddle_x + PADDLE_WIDTH / 2.0) >= PADDLE_VEC * dt {
                    paddle_x += PADDLE_VEC * dt;
                } else if (paddle_x + PADDLE_WIDTH / 2.0) - mouse_position().0 >= PADDLE_VEC * dt {
                    paddle_x -= PADDLE_VEC * dt;
                } else {
                    paddle_x = mouse_position().0 - PADDLE_WIDTH / 2.0;
                }
            } else {
                if is_key_down(KeyCode::Left) {
                    paddle_x -= PADDLE_VEC * dt;
                    if paddle_x < 0.0 - PADDLE_WIDTH / 2.0 {
                        paddle_x = 0.0 - PADDLE_WIDTH / 2.0;
                    }
                }
                if is_key_down(KeyCode::Right) {
                    paddle_x += PADDLE_VEC * dt;
                    if paddle_x > LOGICAL_WIDTH - PADDLE_WIDTH / 2.0 {
                        paddle_x = LOGICAL_WIDTH - PADDLE_WIDTH / 2.0;
                    }
                }
            }

            ball_pos += ball_vel * dt;

            if ball_pos.x < 0.0 {
                ball_vel.x = -ball_vel.x;
                ball_pos.x = 0.0;
            }
            if ball_pos.x > LOGICAL_WIDTH {
                ball_vel.x = -ball_vel.x;
                ball_pos.x = LOGICAL_WIDTH;
            }

            if ball_pos.y < 0.0 {
                ball_vel.y = -ball_vel.y;
                ball_pos.y = 0.0;
            }
            if ball_pos.y > LOGICAL_HEIGHT {
                game_state = GameState::Gameover;
            }
            if ball_pos.x > paddle_x
                && ball_pos.x < paddle_x + PADDLE_WIDTH
                && ball_pos.y > PADDLE_Y - BALL_SIZE / 2.0
            {
                ball_vel.y = - (ball_vel.y.abs());
                ball_pos.y = PADDLE_Y - BALL_SIZE / 2.0;
                point += 1;
                if ball_vel.x.abs() < BALL_VEL_MAX {
                    ball_vel.x *= rand::gen_range(1.05, 1.3);
                }
                if ball_vel.x.abs() > BALL_VEL_MAX {
                    ball_vel.x = BALL_VEL_MAX * rand::gen_range(0.97, 1.03) * ball_vel.x.signum();
                }
                if ball_vel.y.abs() < BALL_VEL_MAX {
                    ball_vel.y *= rand::gen_range(1.05, 1.15);
                }
                if ball_vel.y.abs() > BALL_VEL_MAX {
                    ball_vel.y = BALL_VEL_MAX * rand::gen_range(0.97, 1.03) * ball_vel.y.signum();
                }
            }
        }
        if let GameState::Gameover = game_state {
            if is_key_down(KeyCode::R)
                || mouse_position().0 >= BUTTON_RESTART_POS.x
                    && mouse_position().0 <= BUTTON_RESTART_POS.x + BUTTON_RESTART_WIDTH
                    && mouse_position().1 >= BUTTON_RESTART_POS.y
                    && mouse_position().1 <= BUTTON_RESTART_POS.y + BUTTON_RESTART_HEIGHT
                    && is_mouse_button_pressed(MouseButton::Left)
            {
                paddle_x = LOGICAL_WIDTH / 2.0 - PADDLE_WIDTH / 2.0;
                game_state = GameState::Playing;
                point = 0;
                ball_pos = vec2(
                    rand::gen_range(BALL_SIZE, LOGICAL_WIDTH - BALL_SIZE),
                    BALL_SIZE * 2.0,
                );
                ball_vel = vec2(
                    rand::gen_range(-BALL_VEL_INIT, BALL_VEL_INIT) * 2.0,
                    rand::gen_range(BALL_VEL_INIT * 0.5, BALL_VEL_INIT),
                );
            }
        }

        clear_background(BLACK);
        //camera.use_camera();

        draw_circle(0.0, 0.0, 20.0, RED);
        draw_rectangle(paddle_x, PADDLE_Y, PADDLE_WIDTH, PADDLE_HEIGHT, WHITE);
        draw_circle(
            ball_pos.x + BALL_SIZE / 2.0,
            ball_pos.y + BALL_SIZE / 2.0,
            BALL_SIZE / 2.0,
            WHITE,
        );
        draw_text_ex(
            &format!("Point: {}", point),
            POINT_POS.x,
            POINT_POS.y,
            TextParams {
                font_size: 20,
                font: Some(&my_font),
                color: WHITE,
                ..Default::default()
            },
        );
        draw_rectangle_lines(0.0, 0.0, LOGICAL_WIDTH, LOGICAL_HEIGHT, 3.0, WHITE);
        if let GameState::Gameover = game_state {
            draw_text_ex(
                TEXT_LOSE_CONTEXT,
                TEXT_LOSE_POS.x,
                TEXT_LOSE_POS.y,
                TextParams {
                    font_size: 30,
                    font: Some(&my_font),
                    color: WHITE,
                    ..Default::default()
                },
            );
            draw_rectangle(
                BUTTON_RESTART_POS.x,
                BUTTON_RESTART_POS.y,
                BUTTON_RESTART_WIDTH,
                BUTTON_RESTART_HEIGHT,
                WHITE,
            );
            draw_text_ex(
                "restart",
                BUTTON_RESTART_POS.x + 5.0,
                BUTTON_RESTART_POS.y + BUTTON_RESTART_HEIGHT / 2.0,
                TextParams {
                    font_size: 15,
                    font: Some(&my_font),
                    color: BLACK,
                    ..Default::default()
                },
            );
        };
        //set_default_camera();

        next_frame().await;
    }
}
