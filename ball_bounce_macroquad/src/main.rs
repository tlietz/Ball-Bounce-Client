extern crate rand;

use ball_bounce_macroquad::{systems::render_system, *};
use macroquad::prelude::*;

#[macroquad::main("Ball Bounce")]
async fn main() {
    // initialize platform center screen.
    // let mut platform_x = SCREEN_W / 2.;
    // let platform_speed = SCREEN_W / 2.;
    // let platform_width = PLATFORM_START_W;

    let mut game_state = initial_game_state().await;
    loop {
        let delta = get_frame_time();

        // if is_key_down(KeyCode::Right) && platform_x + platform_width / 2. < SCREEN_W - BORDER_W {
        //     platform_x += platform_speed * delta;
        // }

        // if is_key_down(KeyCode::Left) && platform_x - platform_width / 2. > BORDER_W {
        //     platform_x -= platform_speed * delta;
        // }

        // if let GameState::Ready = game_state {
        //     // ball tracks the platform
        //     ball_y = SCREEN_H - (ball_radius + PLATFORM_HEIGHT + PLATFORM_FLOAT_H);
        //     ball_x = platform_x + ball_offset;
        //     draw_text_ex(
        //         "Press Spacebar to start",
        //         BORDER_W * 4.,
        //         BORDER_W * 20.,
        //         TextParams {
        //             font_size: 40,
        //             font,
        //             color: BLACK,
        //             ..Default::default()
        //         },
        //     );
        //     if is_key_down(KeyCode::Space) {
        //         game_state = GameState::Playing;
        //         score = 0;
        //         launch_ball(
        //             platform_x,
        //             PLATFORM_START_W,
        //             ball_x,
        //             BALL_START_SPEED,
        //             &mut ball_vel,
        //         );
        //     }
        // }

        // if let GameState::Playing = game_state {
        //     ball_x += ball_vel.x * delta;
        //     ball_y += ball_vel.y * delta;

        //     // ball hit left or right boundary
        //     if (ball_vel.x < 0. && ball_x - ball_radius < BORDER_W)
        //         || (ball_vel.x > 0. && ball_x + ball_radius > SCREEN_W - BORDER_W)
        //     {
        //         ball_vel.x *= -1.;
        //     }
        //     //ball hit top boundary
        //     if ball_vel.y < 0. && ball_y - ball_radius < BORDER_W {
        //         ball_vel.y *= -1.;
        //     }

        //     // ball hits platform.
        //     // speed up the ball and increment score
        //     if ball_vel.y > 0.
        //         && ball_x + ball_radius > platform_x - platform_width / 2.
        //         && ball_x - ball_radius < platform_x + platform_width / 2.
        //         && ball_y + ball_radius > SCREEN_H - (PLATFORM_FLOAT_H + PLATFORM_HEIGHT)
        //         && ball_y < SCREEN_H - (PLATFORM_FLOAT_H)
        //     {
        //         score += 1;
        //         if ball_speed < BALL_START_SPEED * 2.5 {
        //             ball_speed += SCREEN_H / 8.;
        //         }
        //         launch_ball(
        //             platform_x,
        //             PLATFORM_START_W,
        //             ball_x,
        //             ball_speed,
        //             &mut ball_vel,
        //         );
        //     }

        //     // ball goes out of bounds
        //     // reinitiliaze variables for new game.
        //     if ball_y + ball_radius >= SCREEN_H {
        //         game_state = GameState::Ready;
        //         ball_offset = new_ball_offset();
        //         ball_speed = BALL_START_SPEED;
        //     }
        // }

        render_system::render(&game_state);

        next_frame().await
    }
}
