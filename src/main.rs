use macroquad::audio::*;
use macroquad::prelude::*;
//use wasm_bindgen::prelude::*;

mod icon_data;
use icon_data::ICON;

const MUSIC_BYTES: &[u8] = include_bytes!("../assets/bgm.ogg");
const VOICE_PONG1_BYTES: &[u8] = include_bytes!("../assets/pong1.ogg");
const VOICE_PONG2_BYTES: &[u8] = include_bytes!("../assets/pong2.ogg");
const VOICE_PONG3_BYTES: &[u8] = include_bytes!("../assets/pong3.ogg");
const FONT_BYTES: &[u8] = include_bytes!("../assets/nova-round.ttf");

const LOGICAL_WIDTH: f32 = 400.0;
const LOGICAL_HEIGHT: f32 = 300.0;

const PADDLE_WIDTH: f32 = 60.0;
const PADDLE_HEIGHT: f32 = 10.0;
const PADDLE_Y: f32 = 260.0;
const PADDLE_VEC: f32 = 260.0;

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

#[derive(PartialEq)]
enum GameState {
    Playing,
    Gameover,
}

// 将鼠标的屏幕坐标转换为逻辑坐标
fn get_zoom(zoom_mode: i32) -> (Vec2, f32, f32) {
    let (mx, my) = mouse_position();
    let mut wx = 0.0;
    let mut wy = 0.0;
    let mut zoom = Vec2::new(0.0, 0.0);
    let tx = screen_width();
    let ty = screen_height();
    let lx = LOGICAL_WIDTH;
    let ly = LOGICAL_HEIGHT;
    if zoom_mode == 1 {
        zoom = vec2(2.0 / LOGICAL_WIDTH, 2.0 / LOGICAL_HEIGHT);
        wx = (mx * LOGICAL_WIDTH) / screen_width();
        wy = (my * LOGICAL_HEIGHT) / screen_height();
    } else if zoom_mode == 2 {
        let psx = LOGICAL_WIDTH / screen_width();
        let psy = LOGICAL_HEIGHT / screen_height();
        let mut sx = 1.0;
        let mut sy = 1.0;
        if psx > psy {
            sy = psy / psx;
        }
        if psy > psx {
            sx = psx / psy;
        }
        zoom = vec2(2.0 * sx / (LOGICAL_WIDTH), 2.0 * sy / (LOGICAL_HEIGHT));
        wx = (mx - (tx * (1.0 - sx) / 2.0)) * lx / (tx * sx);
        wy = (my - (ty * (1.0 - sy) / 2.0)) * ly / (ty * sy);
    }
    (zoom, wx, wy)
}
//窗口信息
fn window_conf() -> Conf {
    Conf {
        window_title: "Pong-rust&wasm".to_string(),
        window_width: LOGICAL_WIDTH as i32,
        window_height: LOGICAL_HEIGHT as i32,
        high_dpi: true,
        icon: ICON,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    //request_new_screen_size(LOGICAL_WIDTH, LOGICAL_HEIGHT);

    let my_font = load_ttf_font_from_bytes(FONT_BYTES).expect("none font:无法加载字体");
    let bgm = load_sound_from_bytes(MUSIC_BYTES).await.unwrap();
    let voice_pong1 = load_sound_from_bytes(VOICE_PONG1_BYTES).await.unwrap();
    let voice_pong2 = load_sound_from_bytes(VOICE_PONG2_BYTES).await.unwrap();
    let voice_pong3 = load_sound_from_bytes(VOICE_PONG3_BYTES).await.unwrap();
    //随机播放碰撞音效
    let voice_pong = || {
        let s = rand::gen_range(1, 4);
        if s == 1 {
            play_sound_once(&voice_pong1);
        } else if s == 2 {
            play_sound_once(&voice_pong2);
        } else if s == 3 {
            play_sound_once(&voice_pong3);
        }
    };
    play_sound(
        &bgm,
        PlaySoundParams {
            looped: true,
            volume: 1.0,
        },
    );

    let mut zoom_mode = 2;
    let mut fps_smooth = 59.0;
    let mut debug_never_die = false;
    let mut debug_cancle_ball_vel_max = false;
    let mut debug_freeze = false;
    let mut is_fullscreen = false;

    let mut paddle_x = LOGICAL_WIDTH / 2.0 - PADDLE_WIDTH / 2.0;
    let mut game_state = GameState::Playing;
    let mut point = 0;
    let mut ball_pos = vec2(
        rand::gen_range(BALL_SIZE, LOGICAL_WIDTH - BALL_SIZE),
        BALL_SIZE * 2.0,
    );
    let mut ball_vel = vec2(
        rand::gen_range(0.45, 0.65) * BALL_VEL_INIT * 1.1,
        rand::gen_range(0.5, 0.75) * BALL_VEL_INIT,
    );

    loop {
        //帧率
        let mut dt = get_frame_time();
        if 1.0 / dt < 30.0 {
            dt = 1.0 / 30.0
        }
        //println!("{}",debug_never_die);
        //设置相机
        let (zoom, wx, wy) = get_zoom(zoom_mode);
        let camera = Camera2D {
            target: vec2(LOGICAL_WIDTH / 2.0, LOGICAL_HEIGHT / 2.0),
            zoom,
            ..Default::default()
        };
        set_camera(&camera);

        if !debug_freeze {
            if let GameState::Playing = game_state {
                //挡板移动
                if is_mouse_button_down(MouseButton::Left) {
                    if wx - (paddle_x + PADDLE_WIDTH / 2.0) >= PADDLE_VEC * dt {
                        paddle_x += PADDLE_VEC * dt;
                    } else if (paddle_x + PADDLE_WIDTH / 2.0) - wx >= PADDLE_VEC * dt {
                        paddle_x -= PADDLE_VEC * dt;
                    } else {
                        paddle_x = wx - PADDLE_WIDTH / 2.0;
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
                //小球移动
                ball_pos += ball_vel * dt;

                if ball_pos.x > paddle_x - BALL_SIZE
                    && ball_pos.x < paddle_x + PADDLE_WIDTH
                    && ball_pos.y > PADDLE_Y - BALL_SIZE / 2.0
                {
                    ball_vel.y = -(ball_vel.y.abs());
                    ball_pos.y = PADDLE_Y - BALL_SIZE / 2.0;
                    point += 1;
                    voice_pong();

                    if !debug_cancle_ball_vel_max {
                        if ball_vel.x.abs() < BALL_VEL_MAX {
                            ball_vel.x *= rand::gen_range(1.05, 1.3);
                        }
                        if ball_vel.y.abs() < BALL_VEL_MAX {
                            ball_vel.y *= rand::gen_range(1.05, 1.15);
                        }
                        if ball_vel.x.abs() > BALL_VEL_MAX {
                            ball_vel.x =
                                BALL_VEL_MAX * rand::gen_range(0.97, 1.03) * ball_vel.x.signum();
                        }
                        if ball_vel.y.abs() > BALL_VEL_MAX {
                            ball_vel.y =
                                BALL_VEL_MAX * rand::gen_range(0.97, 1.03) * ball_vel.y.signum();
                        }
                    } else {
                        if ball_vel.x <= BALL_VEL_MAX {
                            ball_vel.x *= rand::gen_range(1.05, 1.3);
                        }
                        if ball_vel.y <= BALL_VEL_MAX {
                            ball_vel.y *= rand::gen_range(1.05, 1.1);
                        }
                    }
                }

                if ball_pos.x < 0.0 {
                    ball_vel.x = -ball_vel.x;
                    ball_pos.x = 0.0;
                    voice_pong();
                }
                if ball_pos.x > LOGICAL_WIDTH - BALL_SIZE {
                    ball_vel.x = -ball_vel.x;
                    ball_pos.x = LOGICAL_WIDTH - BALL_SIZE;
                    voice_pong();
                }

                if ball_pos.y < 0.0 {
                    ball_vel.y = -ball_vel.y;
                    ball_pos.y = 0.0;
                    voice_pong();
                }
                if ball_pos.y > LOGICAL_HEIGHT + BALL_SIZE * 2.0 {
                    if debug_never_die {
                        ball_vel.y = -ball_vel.y;
                        ball_pos.y = LOGICAL_HEIGHT + BALL_SIZE;
                    } else {
                        game_state = GameState::Gameover;
                    }
                }
            }
            //restart
            if is_key_down(KeyCode::R)
                || wx >= BUTTON_RESTART_POS.x
                    && wx <= BUTTON_RESTART_POS.x + BUTTON_RESTART_WIDTH
                    && wy >= BUTTON_RESTART_POS.y
                    && wy <= BUTTON_RESTART_POS.y + BUTTON_RESTART_HEIGHT
                    && is_mouse_button_pressed(MouseButton::Left)
                    && GameState::Gameover == game_state
            {
                paddle_x = LOGICAL_WIDTH / 2.0 - PADDLE_WIDTH / 2.0;
                game_state = GameState::Playing;
                point = 0;
                if !debug_cancle_ball_vel_max {
                    ball_pos = vec2(
                        rand::gen_range(BALL_SIZE, LOGICAL_WIDTH - BALL_SIZE),
                        BALL_SIZE * 2.0,
                    );
                    ball_vel = vec2(
                        rand::gen_range(0.45, 0.65) * BALL_VEL_INIT * 1.1,
                        rand::gen_range(0.5, 0.75) * BALL_VEL_INIT,
                    );
                } else {
                    ball_pos = vec2(
                        rand::gen_range(BALL_SIZE, LOGICAL_WIDTH - BALL_SIZE),
                        BALL_SIZE * 2.0,
                    );
                    ball_vel = vec2(
                        rand::gen_range(-BALL_VEL_INIT, BALL_VEL_INIT) * 2.0,
                        rand::gen_range(0.0, BALL_VEL_INIT),
                    );
                }
            }
        }
        //debug
        if is_key_pressed(KeyCode::Key1) {
            zoom_mode = 1;
        }
        if is_key_pressed(KeyCode::Key2) {
            zoom_mode = 2;
        }
        if is_key_pressed(KeyCode::U) {
            debug_never_die = !debug_never_die;
        }
        if is_key_pressed(KeyCode::I) {
            debug_cancle_ball_vel_max = !debug_cancle_ball_vel_max;
        }
        if is_key_pressed(KeyCode::P) {
            debug_freeze = !debug_freeze;
        }
        if is_key_pressed(KeyCode::M) {
            is_fullscreen = !is_fullscreen;
            set_fullscreen(is_fullscreen);
        }
		//draw
        clear_background(BLACK);
        draw_circle(0.0, 0.0, 20.0, RED);
        draw_rectangle(paddle_x, PADDLE_Y, PADDLE_WIDTH, PADDLE_HEIGHT, WHITE);
        draw_circle(
            ball_pos.x + BALL_SIZE / 2.0,
            ball_pos.y + BALL_SIZE / 2.0,
            BALL_SIZE / 2.0,
            WHITE,
        );
        draw_text_ex(
            "press left, right, or mouse_left to move",
            2.0,
            35.0,
            TextParams {
                font_size: 15,
                font: Some(&my_font),
                color: GRAY,
                ..Default::default()
            },
        );
        draw_text_ex(
            "more key: 1,2,R,M,U,I,P",
            2.0,
            35.0 + 18.0,
            TextParams {
                font_size: 15,
                font: Some(&my_font),
                color: GRAY,
                ..Default::default()
            },
        );
        if debug_never_die {
            draw_text_ex(
                "never die",
                LOGICAL_WIDTH - 80.0,
                40.0,
                TextParams {
                    font_size: 17,
                    font: Some(&my_font),
                    color: BLUE,
                    ..Default::default()
                },
            );
        }
        if debug_cancle_ball_vel_max {
            draw_text_ex(
                "cancle_ball_vel_max",
                LOGICAL_WIDTH - 170.0,
                40.0 + 18.0,
                TextParams {
                    font_size: 17,
                    font: Some(&my_font),
                    color: BLUE,
                    ..Default::default()
                },
            );
        }
        if debug_freeze {
            draw_text_ex(
                "freeze",
                LOGICAL_WIDTH - 60.0,
                40.0 + 18.0 * 2.0,
                TextParams {
                    font_size: 17,
                    font: Some(&my_font),
                    color: BLUE,
                    ..Default::default()
                },
            );
        }
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
        let current_fps = get_fps() as f32;
        fps_smooth = fps_smooth * 0.98 + current_fps * 0.02;
        draw_text_ex(
            &format!("FPS: {}", fps_smooth as i32),
            LOGICAL_WIDTH - 80.0,
            20.0,
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
                BUTTON_RESTART_POS.y + BUTTON_RESTART_HEIGHT / 2.0 + 6.0,
                TextParams {
                    font_size: 22,
                    font: Some(&my_font),
                    color: BLACK,
                    ..Default::default()
                },
            );
            draw_text_ex(
                "Can you reach 20 points?",
                BUTTON_RESTART_POS.x - 60.0,
                BUTTON_RESTART_POS.y + BUTTON_RESTART_HEIGHT + 25.0,
                TextParams {
                    font_size: 18,
                    font: Some(&my_font),
                    color: WHITE,
                    ..Default::default()
                },
            );
        };
        if is_mouse_button_down(MouseButton::Left) {
            draw_circle(wx, wy, 5.0, GREEN.with_alpha(0.5));
        } else {
            draw_circle(wx, wy, 5.0, YELLOW.with_alpha(0.5));
        }

        next_frame().await;
    }
}
