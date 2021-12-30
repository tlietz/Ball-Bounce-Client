use crate::*;
use macroquad::prelude::*;

pub fn execute(game_state: &mut GameState, delta: f32) {
    println!("Executing");
    for player_entity_index in &game_state.players {
        if let Some(Entity::Player(ref mut player)) =
            game_state.entities[player_entity_index.clone()]
        {
            if is_key_down(player.control.left) {
                player.position.x -= delta * player.speed;
            }
        }
    }
}
