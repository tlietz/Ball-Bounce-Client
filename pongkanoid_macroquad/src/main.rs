use macroquad::{color, prelude::*};

enum GameState {
    Ready,
    Playing,
}

const SCREEN_W: f32 = 20.0;
const SCREEN_H: f32 = 20.0;

const BORDER_W: f32 = 0.5;
const BORDER_COLOR: color::Color = GRAY;

const BALL_COLOR: color::Color = RED;
const PLATFORM_COLOR: color::Color = WHITE;

#[macroquad::main("Pongkanoid")]
async fn main() {
    // initialize platform center screen.
    let mut platform_x = SCREEN_W / 2.;
    let platform_width = 2.;
    let platform_height = BORDER_W;

    // initialize the ball to be sitting on the top of the platform
    let mut ball_radius = platform_height * 0.5;
    let mut ball_x = platform_x;
    let mut ball_y = SCREEN_H - (ball_radius + platform_height);
    let mut ball_vel = vec2(0.0, 0.0);

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
            platform_x += 5. * delta;
        }

        if is_key_down(KeyCode::Left) && platform_x - platform_width / 2. > BORDER_W {
            platform_x -= 5. * delta;
        }

        if let GameState::Ready = game_state {
            ball_x = platform_x;
            if is_key_down(KeyCode::Space) {
                game_state = GameState::Playing;
            }
        }

        if let GameState::Playing = game_state {
            ball_y = ball_y - 9.0 * delta;
        }

        draw_circle(ball_x, ball_y, ball_radius, BALL_COLOR);
        draw_rectangle(
            platform_x - platform_width / 2.,
            SCREEN_H - platform_height,
            platform_width,
            platform_height,
            PLATFORM_COLOR,
        );

        draw_border();

        next_frame().await
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
