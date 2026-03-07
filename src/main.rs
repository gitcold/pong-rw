use macroquad::prelude::*;

const WINDOW_WIDTH: f32 = 400.0;
const WINDOW_HEIGHT: f32 = 300.0;

const PADDLE_WIDTH: f32 = 60.0;
const PADDLE_HEIGHT: f32 = 10.0;
const PADDLE_Y: f32 = 260.0;
const PADDLE_VEC: f32 = 240.0;

const BALL_SIZE: f32 = 10.0;
const BALL_VEL_INIT: f32 = 100.0;
const BALL_VEL_MAX: f32 = 300.0;

const POINT_POS: Vec2 = Vec2::new(WINDOW_WIDTH / 2.0 - 40.0, 14.0);
const TEXT_LOSE_POS: Vec2 = Vec2::new(WINDOW_WIDTH / 2.0 - 60.0, WINDOW_HEIGHT / 2.0);
const TEXT_LOSE_CONTEXT: &str = "You lose!";

enum GameState {
    Playing,
    Gameover,
}

#[macroquad::main("pong-rw")]
async fn main() {
    request_new_screen_size(WINDOW_WIDTH, WINDOW_HEIGHT);

    let mut paddle_x = WINDOW_WIDTH / 2.0 - PADDLE_WIDTH / 2.0;
    let mut game_state = GameState::Playing;
    let mut point = 0;
    let mut ball_pos = vec2(
        rand::gen_range(BALL_SIZE, WINDOW_WIDTH - BALL_SIZE),
        BALL_SIZE * 2.0,
    );
    let mut ball_vel = vec2(
        rand::gen_range(-BALL_VEL_INIT, BALL_VEL_INIT) * 2.0,
        rand::gen_range(0.0, BALL_VEL_INIT),
    );

    loop {
        let dt = get_frame_time();

        if let GameState::Playing = game_state {
            if is_mouse_button_down(MouseButton::Left) {
                if mouse_position().0 - (paddle_x + PADDLE_WIDTH / 2.0) >= PADDLE_VEC * dt {
                    paddle_x += PADDLE_VEC * dt;
                } else if (paddle_x + PADDLE_WIDTH / 2.0) - mouse_position().0 >= PADDLE_VEC * dt {
                    paddle_x -= PADDLE_VEC * dt;
                } else {
                    paddle_x = mouse_position().0 - PADDLE_WIDTH / 2.0;
                }
            }

            ball_pos += ball_vel * dt;

            if ball_pos.x < 0.0 {
                ball_vel.x = -ball_vel.x;
                ball_pos.x = 0.0;
            }
            if ball_pos.x > WINDOW_WIDTH {
                ball_vel.x = -ball_vel.x;
                ball_pos.x = WINDOW_WIDTH;
            }

            if ball_pos.y < 0.0 {
                ball_vel.y = -ball_vel.y;
                ball_pos.y = 0.0;
            }
            if ball_pos.y > WINDOW_HEIGHT {
                game_state = GameState::Gameover;
            }
            if ball_pos.x >= paddle_x
                && ball_pos.x <= paddle_x + PADDLE_WIDTH
                && ball_pos.y >= PADDLE_Y - BALL_SIZE / 2.0
            {
                ball_vel.y = -ball_vel.y;
                ball_pos.y = PADDLE_Y - BALL_SIZE / 2.0;
                point += 1;
                if ball_vel.x <= BALL_VEL_MAX {
                    ball_vel.x *= rand::gen_range(1.05, 1.3);
                }
                if ball_vel.y <= BALL_VEL_MAX {
                    ball_vel.y *= rand::gen_range(1.05, 1.1);
                }
            }
        }

        clear_background(BLACK);
        draw_rectangle(paddle_x, PADDLE_Y, PADDLE_WIDTH, PADDLE_HEIGHT, WHITE);
        draw_circle(
            ball_pos.x + BALL_SIZE / 2.0,
            ball_pos.y + BALL_SIZE / 2.0,
            BALL_SIZE / 2.0,
            WHITE,
        );
        draw_text(
            &format!("Point: {}", point),
            POINT_POS.x,
            POINT_POS.y,
            20.0,
            WHITE,
        );
        draw_rectangle_lines(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT, 3.0, WHITE);
        if let GameState::Gameover = game_state {
            draw_text(
                TEXT_LOSE_CONTEXT,
                TEXT_LOSE_POS.x,
                TEXT_LOSE_POS.y,
                30.0,
                WHITE,
            );
        };

        next_frame().await;
    }
}
