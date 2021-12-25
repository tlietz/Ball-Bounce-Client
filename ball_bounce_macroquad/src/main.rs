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

const SCREEN_W: f32 = 20.0;
const SCREEN_H: f32 = 20.0;

const BORDER_W: f32 = 0.5;
const BORDER_COLOR: color::Color = GRAY;

const BALL_COLOR: color::Color = RED;
const BALL_START_SPEED: f32 = 15.;

const PLATFORM_COLOR: color::Color = WHITE;
const PLATFORM_FLOAT_H: f32 = 1.;
const PLATFORM_START_W: f32 = 2.0;

#[macroquad::main("Pongkanoid")]
async fn main() {
    // initialize platform center screen.
    let mut platform_x = SCREEN_W / 2.;
    let platform_speed = 5.;
    let platform_width = PLATFORM_START_W;
    let platform_height = BORDER_W * 0.75;

    let ball_radius = platform_height * 0.4;
    let mut ball_speed = BALL_START_SPEED;
    let mut ball_vel = vec2(0.0, 0.0);
    // Ball initialized sitting on the top of the platform,
    // randomly deviated from the center
    let mut ball_offset: f32 = new_ball_offset();
    let mut ball_x = platform_x + ball_offset;
    let mut ball_y = SCREEN_H - (ball_radius + platform_height + PLATFORM_FLOAT_H);

    let mut game_state = GameState::Ready;

    // builds camera with following coordinate system:
    // (0, 0)        ... (SCREEN_W, 0.)
    // (0, SCREEN_H) ... (SCREEN_W, SCREEN_H)
    set_camera(&Camera2D {
        zoom: vec2(1. / SCREEN_W * 2., -1. / SCREEN_H * 2.),
        target: vec2(SCREEN_W / 2., SCREEN_H / 2.),
        ..Default::default()
    });

    loop {
        let delta = get_frame_time();

        if is_key_down(KeyCode::Right) && platform_x + platform_width / 2. < SCREEN_W - BORDER_W {
            platform_x += platform_speed * delta;
        }

        if is_key_down(KeyCode::Left) && platform_x - platform_width / 2. > BORDER_W {
            platform_x -= platform_speed * delta;
        }

        if let GameState::Ready = game_state {
            ball_y = SCREEN_H - (ball_radius + platform_height + PLATFORM_FLOAT_H);
            ball_x = platform_x + ball_offset;
            if is_key_down(KeyCode::Space) {
                game_state = GameState::Playing;
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
            if (ball_x - ball_radius <= BORDER_W) || (ball_x + ball_radius >= SCREEN_W - BORDER_W) {
                bounce_ball(Axis::X, &mut ball_vel);
            }
            if ball_y - ball_radius <= BORDER_W {
                bounce_ball(Axis::Y, &mut ball_vel);
            }

            // ball goes out of bounds
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

        next_frame().await
    }
}

fn new_ball_offset() -> f32 {
    rand::thread_rng().gen_range(((-PLATFORM_START_W / 2.) * 0.5)..=((PLATFORM_START_W / 2.) * 0.5))
}

// Launches the ball from the platform by changing its velocity correponding to where the ball hits the platform.
// This is used when the ball hits the platform, or when launching the ball with spacebar.
fn launch_ball(
    platform_x: f32,
    platform_width: f32,
    ball_x: f32,
    ball_speed: f32,
    ball_vel: &mut Vec2,
) {
    let percent_offset = (ball_x - platform_x) / (platform_width / 2.);
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
