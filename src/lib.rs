// Uses ECS (Entity component system) design pattern.
// Note how the complexity lives in the systems, instead
// of the objects defined here in `lib`.

mod constants;
pub mod init;
pub mod systems;

use constants::*;
use macroquad::{color, prelude::*, rand};

#[derive(Debug, Clone)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Velocity {
    dx: f32,
    dy: f32,
}

#[derive(Debug)]
struct Control {
    left: KeyCode,
    right: KeyCode,
}

#[derive(Debug)]
struct Ball {
    position: Position,
    velocity: Velocity,
    speed: f32,
    radius: f32,
    state: BallState,
    color: color::Color,
}

type EntityIndex = usize;
#[derive(Debug)]
enum BallState {
    // `player_entity_index` determines which player the ball will follow before
    // the game starts.
    //
    // `offset` determines the offset the ball has from the center of the player's paddle.
    Ready {
        player_entity_index: EntityIndex,
        offset: f32,
    },
    Playing,
}

#[derive(Debug)]
struct Player {
    position: Position,
    speed: f32,
    width: f32,
    height: f32,
    control: Control,
    color: color::Color,
}

#[derive(Debug)]
struct Text {
    text: &'static str,
    position: Position,
    font_size: u16,
}

#[derive(Debug)]
struct Border {
    position: Position,
    width: f32,
    height: f32,
    color: color::Color,
}

pub struct GameState {
    players: Vec<Player>,
    balls: Vec<Ball>,
    texts: Vec<Text>,
    borders: Vec<Border>,
    score: u32,
    font: Font,
}

fn random_player_index(num_players: usize) -> usize {
    rand::gen_range(0, num_players)
}

fn rand_ball_offset() -> f32 {
    rand::gen_range(
        (-PLATFORM_START_W / 2.) * 0.5,
        (PLATFORM_START_W / 2.) * 0.5,
    )
}

fn restart_ball(ball: &mut Ball, player_index: EntityIndex) {
    ball.state = BallState::Ready {
        player_entity_index: player_index,
        offset: rand_ball_offset(),
    };
    ball.position.y = init_ball_offset(ball.radius);
    ball.speed = BALL_START_SPEED;
}

fn init_ball_offset(radius: f32) -> f32 {
    SCREEN_H - (radius + PLATFORM_HEIGHT + PLATFORM_FLOAT_H)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_lose() {
        let game_state = {};
        let offset: f32 = 0.7;
        assert_eq!(2 + 2, 4);
    }
}
