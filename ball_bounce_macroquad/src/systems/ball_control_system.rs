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
            let player = &game_state.players[player_entity_index.clone()];
            let player_x = player.position.x;

            ball.position.x = player_x + offset.clone();

            if is_key_down(KeyCode::Space) {
                // launching the ball must come before changing the state,
                // because the `launch_ball` function uses the `player_entity_index`
                // varible from BallState::Ready enum
                launch_ball(ball, player);
                ball.state = BallState::Playing;
            }
        }
        BallState::Playing => {
            for player in &game_state.players {
                ball.position.x += ball.velocity.dx * delta;
                ball.position.y += ball.velocity.dy * delta;

                // ball hit left or right boundary
                if (ball.velocity.dx < 0. && ball.position.x - ball.radius < BORDER_W)
                    || (ball.velocity.dx > 0.
                        && ball.position.x + ball.radius > SCREEN_W - BORDER_W)
                {
                    ball.velocity.dx *= -1.;
                }
                //ball hit top boundary
                if ball.velocity.dy < 0. && ball.position.y - ball.radius < BORDER_W {
                    ball.velocity.dy *= -1.;
                }

                // ball hits platform.
                // speed up the ball and increment score
                if ball.velocity.dy > 0.
                    && ball.position.x + ball.radius > player.position.x - player.width / 2.
                    && ball.position.x - ball.radius < player.position.x + player.width / 2.
                    && ball.position.y + ball.radius
                        > SCREEN_H - (PLATFORM_FLOAT_H + PLATFORM_HEIGHT)
                    && ball.position.y < SCREEN_H - (PLATFORM_FLOAT_H)
                {
                    game_state.score += 1;
                    if ball.speed < BALL_START_SPEED * 2.5 {
                        ball.speed += SCREEN_H / 8.;
                    }
                    launch_ball(ball, player);
                }

                // ball goes out of bounds
                // reinitiliaze variables for new game.
                if ball.position.y + ball.radius >= SCREEN_H {
                    ball.state = BallState::Ready {
                        player_entity_index: random_player_index(game_state.players.len()),
                        offset: rand_ball_offset(),
                    };
                    ball.position.y = SCREEN_H - (ball.radius + PLATFORM_HEIGHT + PLATFORM_FLOAT_H);
                    ball.speed = BALL_START_SPEED;
                }
            }
        }
    }
}

// Launches the ball from the platform by changing its velocity corresponding to where the ball hits the platform.
// This is used when the ball hits the platform, or when launching the ball with spacebar.
// The ball is launched at a maximum angle with respect to the normal.
fn launch_ball(ball: &mut Ball, player: &Player) {
    // maximum offset calculated to determine angle of launch
    let max_offset = 0.7;
    let mut percent_offset = (ball.position.x - player.position.x) / (player.width / 2.);
    if percent_offset < -max_offset {
        percent_offset = -max_offset;
    } else if percent_offset > max_offset {
        percent_offset = max_offset;
    } else if percent_offset == 0.00 {
        // if the ball hits the center of platform, launch at a random angle
        percent_offset = rand::thread_rng().gen_range(-0.10..=0.10)
    }

    ball.velocity.dx = percent_offset * ball.speed;
    ball.velocity.dy = -(1. - percent_offset.abs()) * ball.speed;
}
