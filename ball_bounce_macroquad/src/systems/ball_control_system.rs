use crate::*;
use macroquad::prelude::*;

pub fn execute(game_state: &mut GameState, delta: f32) {
    // Hard coded for 1 ball. There might be multiple balls added later.
    let ball = &mut game_state.balls[0];
    match &ball.state {
        BallState::Ready {
            player_entity_index,
            offset,
        } => {
            // follow the paddle of the player

            let player_x = game_state.players[player_entity_index.clone()].position.x;

            ball.position.x = player_x + offset.clone();
        }
        Playing => {}
    }
}
