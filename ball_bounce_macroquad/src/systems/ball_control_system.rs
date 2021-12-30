use crate::*;
use macroquad::prelude::*;

pub fn execute(game_state: &mut GameState, delta: f32) {
    // Hard coded for 1 ball. There might be multiple balls added later.
    let ball_index = game_state.balls[0];
    if let Some(Entity::Ball(ref mut ball)) = game_state.entities[ball_index] {
        match &ball.state {
            BallState::Ready {
                player_entity_index,
                offset,
            } => {
                // follow the paddle of the player
                ball.position.x = offset.clone();
            }
            Playing => {}
        }
    }
}
