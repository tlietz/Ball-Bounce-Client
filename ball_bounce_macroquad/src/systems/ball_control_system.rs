use crate::*;
use macroquad::prelude::*;

extern crate rand;

pub fn execute(game_state: &mut GameState, delta: f32) {
    // Hard coded for 1 ball. There might be multiple balls added later.
    let ball = &mut game_state.balls[0];
    match &ball.state {
        BallState::Ready {
            player_entity_index,
            offset,
        } => {
            // follows the paddle of the player
            let player_x = game_state.players[player_entity_index.clone()].position.x;

            ball.position.x = player_x + offset.clone();

            if is_key_down(KeyCode::Space) {
                ball.state = BallState::Playing;
            }
        }
        BallState::Playing => {}
    }
}

// Launches the ball from the platform by changing its velocity corresponding to where the ball hits the platform.
// This is used when the ball hits the platform, or when launching the ball with spacebar.
// The ball is launched at a maximum angle with respect to the normal.
fn launch_ball(
    platform_x: f32,
    platform_width: f32,
    ball_x: f32,
    ball_speed: f32,
    ball_vel: &mut Vec2,
) {
    // maximum offset calculated to determine angle of launch
    let max_offset = 0.7;
    let mut percent_offset = (ball_x - platform_x) / (platform_width / 2.);
    if percent_offset < -max_offset {
        percent_offset = -max_offset;
    } else if percent_offset > max_offset {
        percent_offset = max_offset;
    } else if percent_offset == 0.00 {
        // if the ball hits the center of platform, launch at a random angle
        percent_offset = rand::thread_rng().gen_range(-0.10..=0.10)
    }

    ball_vel.x = percent_offset * ball_speed;
    ball_vel.y = -(1. - percent_offset.abs()) * ball_speed;
}
