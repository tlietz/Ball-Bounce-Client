use crate::*;
use macroquad::prelude::*;

pub fn execute(game_state: &mut GameState, delta: f32) {
    // iterates through all players and moves their platform corresponding to the keys being pressed.
    for player in &mut game_state.players {
        if is_key_down(player.control.left) && player.position.x - player.width / 2. > BORDER_W {
            player.position.x -= delta * player.speed;
        }

        if is_key_down(player.control.right)
            && player.position.x + player.width / 2. < SCREEN_W - BORDER_W
        {
            player.position.x += delta * player.speed;
        }
    }
}
