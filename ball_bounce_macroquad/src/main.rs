extern crate rand;
use macroquad::{color, prelude::*};
use rand::Rng;

enum GameState {
    Ready,
    Playing,
}

enum Axis {
    X,
    Y,
}

const SCREEN_W: f32 = 600.0;
const SCREEN_H: f32 = 600.0;

const BORDER_W: f32 = SCREEN_W * 0.025;
const BORDER_COLOR: color::Color = GRAY;

const BALL_COLOR: color::Color = RED;
const BALL_START_SPEED: f32 = SCREEN_H / 1.5;

const PLATFORM_COLOR: color::Color = WHITE;
const PLATFORM_FLOAT_H: f32 = SCREEN_H * 0.025;
const PLATFORM_START_W: f32 = SCREEN_W * 0.11;

#[macroquad::main("Ball Bounce")]
async fn main() {
    let font = load_ttf_font("../assets/MinimalPixelv2.ttf").await.unwrap();

    // initialize platform center screen.
    let mut platform_x = SCREEN_W / 2.;
    let platform_speed = SCREEN_W / 1.5;
    let platform_width = PLATFORM_START_W;
    let platform_height = BORDER_W * 0.75;

    let ball_radius = platform_height * 0.5;
    let mut ball_speed = BALL_START_SPEED;
    let mut ball_vel = vec2(0.0, 0.0);
    // Ball initialized sitting on the top of the platform,
    // randomly deviated from the center
    let mut ball_offset: f32 = new_ball_offset();
    let mut ball_x = platform_x + ball_offset;
    let mut ball_y = SCREEN_H - (ball_radius + platform_height + PLATFORM_FLOAT_H);

    let mut score: i32 = 0;

    let mut game_state = GameState::Ready;

    loop {
        let delta = get_frame_time();

        // draw background
        draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, DARKBLUE);

        if is_key_down(KeyCode::Right) && platform_x + platform_width / 2. < SCREEN_W - BORDER_W {
            platform_x += platform_speed * delta;
        }

        if is_key_down(KeyCode::Left) && platform_x - platform_width / 2. > BORDER_W {
            platform_x -= platform_speed * delta;
        }

        if let GameState::Ready = game_state {
            // ball tracks the platform
            ball_y = SCREEN_H - (ball_radius + platform_height + PLATFORM_FLOAT_H);
            ball_x = platform_x + ball_offset;
            draw_text_ex(
                "Press Spacebar to start",
                BORDER_W * 4.,
                BORDER_W * 20.,
                TextParams {
                    font_size: 40,
                    font,
                    color: BLACK,
                    ..Default::default()
                },
            );
            if is_key_down(KeyCode::Space) {
                game_state = GameState::Playing;
                score = 0;
                launch_ball(
                    platform_x,
                    PLATFORM_START_W,
                    ball_x,
                    BALL_START_SPEED,
                    &mut ball_vel,
                );
            }
        }

        if let GameState::Playing = game_state {
            ball_x += ball_vel.x * delta;
            ball_y += ball_vel.y * delta;

            // ball hit left or right boundary
            if (ball_x - ball_radius < BORDER_W) || (ball_x + ball_radius > SCREEN_W - BORDER_W) {
                bounce_ball(Axis::X, &mut ball_vel);
            }
            //ball hit top boundary
            if ball_y - ball_radius < BORDER_W {
                bounce_ball(Axis::Y, &mut ball_vel);
            }

            // ball hits platform.
            // speed up the ball and increment score
            if ball_x + ball_radius > platform_x - platform_width / 2.
                && ball_x - ball_radius < platform_x + platform_width / 2.
                && ball_y + ball_radius > SCREEN_H - (PLATFORM_FLOAT_H + platform_height)
                && ball_y < SCREEN_H - (PLATFORM_FLOAT_H)
            {
                score += 1;
                if ball_speed < BALL_START_SPEED * 2.5 {
                    ball_speed += SCREEN_H / 8.;
                }
                println!("{}", ball_speed);
                launch_ball(
                    platform_x,
                    PLATFORM_START_W,
                    ball_x,
                    ball_speed,
                    &mut ball_vel,
                );
            }

            // ball goes out of bounds
            // reinitiliaze variables for new game.
            if ball_y + ball_radius >= SCREEN_H {
                game_state = GameState::Ready;
                ball_offset = new_ball_offset();
            }
        }

        // draw ball
        draw_circle(ball_x, ball_y, ball_radius, BALL_COLOR);
        // draw platform
        draw_rectangle(
            platform_x - platform_width / 2.,
            SCREEN_H - (platform_height + PLATFORM_FLOAT_H),
            platform_width,
            platform_height,
            PLATFORM_COLOR,
        );
        draw_border();

        // draw score
        draw_text_ex(
            &score.to_string(),
            BORDER_W * 2.,
            BORDER_W * 5.,
            TextParams {
                font_size: 50,
                font,
                ..Default::default()
            },
        );

        next_frame().await
    }
}

fn new_ball_offset() -> f32 {
    rand::thread_rng().gen_range(((-PLATFORM_START_W / 2.) * 0.5)..=((PLATFORM_START_W / 2.) * 0.5))
}

// Launches the ball from the platform by changing its velocity correponding to where the ball hits the platform.
// This is used when the ball hits the platform, or when launching the ball with spacebar.
// The ball is launched at a maximum angle with respect to the normal.
fn launch_ball(
    platform_x: f32,
    platform_width: f32,
    ball_x: f32,
    ball_speed: f32,
    ball_vel: &mut Vec2,
) {
    let max_angle = 0.7;
    let mut percent_offset = (ball_x - platform_x) / (platform_width / 2.);
    if percent_offset < -max_angle {
        percent_offset = -max_angle;
    } else if percent_offset > 0.75 {
        percent_offset = max_angle;
    } else if percent_offset == 0.00 {
        // if the ball hits the center of platform, launch at a random angle
        percent_offset = rand::thread_rng().gen_range(-0.10..=0.10)
    }

    ball_vel.x = percent_offset * ball_speed;
    ball_vel.y = -(1. - percent_offset.abs()) * ball_speed;
}

// bounces the ball across the axis passed in
fn bounce_ball(axis: Axis, ball_vel: &mut Vec2) {
    match axis {
        Axis::X => ball_vel.x *= -1.,
        Axis::Y => ball_vel.y *= -1.,
    }
}

fn draw_border() {
    // left border
    draw_rectangle(0., 0., BORDER_W, SCREEN_H, BORDER_COLOR);
    //right border
    draw_rectangle(SCREEN_W - BORDER_W, 0., BORDER_W, SCREEN_H, BORDER_COLOR);
    //top border
    draw_rectangle(0., 0., SCREEN_W, BORDER_W, BORDER_COLOR);
}
