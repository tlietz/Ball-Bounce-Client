use ball_bounce_macroquad::{systems::*, *};
use macroquad::prelude::*;

#[macroquad::main("Ball Bounce")]
async fn main() {
    let mut game_state = initial_game_state().await;
    loop {
        let delta = get_frame_time();

        player_control_system::execute(&mut game_state, delta);
        ball_control_system::execute(&mut game_state, delta);
        render_system::render(&game_state);

        next_frame().await
    }
}
