use macroquad::prelude::*;

#[macroquad::main("Pongkanoid")]
async fn main() {
    const SCREEN_W: f32 = 20.0;
    const SCREEN_H: f32 = 20.0;

    let mut platform_x = SCREEN_W / 2.;
    let platform_width = 2.;
    let platform_height = 0.5;

    // initialize the ball to be sitting on the top of the platform
    let mut ball_radius = platform_height * 0.5;
    let mut ball_x = platform_x;
    let mut ball_y = SCREEN_H - (ball_radius + platform_height);

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

        if is_key_down(KeyCode::Right) && platform_x + platform_width / 2. < SCREEN_W {
            platform_x += 5. * delta;
        }

        if is_key_down(KeyCode::Left) && platform_x - platform_width / 2. > 0. {
            platform_x -= 5. * delta;
        }

        draw_circle(ball_x, ball_y, ball_radius, RED);
        draw_rectangle(
            platform_x - platform_width / 2.,
            SCREEN_H - platform_height,
            platform_width,
            platform_height,
            WHITE,
        );

        next_frame().await
    }
}
