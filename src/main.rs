use macroquad::prelude::*;

const WINDOW_WIDTH: f32 = 400.0;
const WINDOW_HEIGHT: f32 = 300.0;

const PADDLE_WIDTH: f32 = 60.0;
const PADDLE_HEIGHT: f32 = 10.0;
const PADDLE_Y: f32 = 260.0;
const PADDLE_VEC: f32 = 15.0;

enum GameState {
    Playing,
    GameOver,
}

#[macroquad::main("pong-rw")]
async fn main() {
    request_new_screen_size(WINDOW_WIDTH, WINDOW_HEIGHT);

    let mut paddle_x = WINDOW_WIDTH / 2.0 - PADDLE_WIDTH / 2.0;
    let mut game_state = GameState::Playing;

    loop {
        if is_mouse_button_down(MouseButton::Left) {
            if mouse_position().0 - paddle_x >= PADDLE_VEC {
                paddle_x += PADDLE_VEC;
            } else if paddle_x - mouse_position().0 >= PADDLE_VEC {
                paddle_x -= PADDLE_VEC;
            } else {
                paddle_x = mouse_position().0;
            }
        }

        clear_background(BLACK);
        draw_rectangle(paddle_x, PADDLE_Y, PADDLE_WIDTH, PADDLE_HEIGHT, WHITE);

        next_frame().await;
    }
}
