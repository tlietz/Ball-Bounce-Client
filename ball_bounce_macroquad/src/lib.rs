mod constants;
pub mod systems;

extern crate rand;
use constants::*;
use macroquad::{color, prelude::*};
use rand::Rng;

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

pub async fn initial_game_state() -> GameState {
    let platform_height = SCREEN_W * 0.02;
    let player1 = Player {
        position: Position {
            x: SCREEN_W / 4.,
            y: SCREEN_H - (PLATFORM_FLOAT_H + platform_height),
        },
        speed: SCREEN_W / 4.,
        width: PLATFORM_START_W,
        height: platform_height,
        control: Control {
            left: KeyCode::A,
            right: KeyCode::D,
        },
        color: WHITE,
    };
    let mut players = vec![player1];

    let player2 = Player {
        position: Position {
            x: SCREEN_W * 3. / 4.,
            y: SCREEN_H - (PLATFORM_FLOAT_H + platform_height),
        },
        speed: SCREEN_W * 3. / 4.,
        width: PLATFORM_START_W,
        height: platform_height,
        control: Control {
            left: KeyCode::Left,
            right: KeyCode::Right,
        },
        color: PURPLE,
    };
    players.push(player2);

    // Ball initialized sitting on the top of a random player paddle,
    // randomly deviated from the center
    let ball_radius = platform_height * 0.5;
    let offset = new_ball_offset();
    let ball = Ball {
        position: Position {
            x: offset + SCREEN_W / 2.,
            y: SCREEN_H - (ball_radius + platform_height + PLATFORM_FLOAT_H),
        },
        velocity: Velocity { dx: 0., dy: 0. },
        radius: ball_radius,
        state: BallState::Ready {
            player_entity_index: random_player_index(players.len()),
            offset,
        },
        color: RED,
    };
    let balls = vec![ball];

    let text = Text {
        text: "Press spacebar to start",
        position: Position {
            x: SCREEN_W * 0.1,
            y: SCREEN_H * 0.4,
        },
        font_size: 40,
    };
    let texts = vec![text];

    let left_border = Border {
        position: Position { x: 0., y: 0. },
        width: BORDER_W,
        height: SCREEN_H,
        color: GRAY,
    };
    let mut borders = vec![left_border];

    let right_border = Border {
        position: Position {
            x: SCREEN_W - BORDER_W,
            y: 0.,
        },
        width: BORDER_W,
        height: SCREEN_H,
        color: GRAY,
    };
    borders.push(right_border);

    let top_border = Border {
        position: Position { x: 0., y: 0. },
        width: SCREEN_W,
        height: BORDER_W,
        color: GRAY,
    };
    borders.push(top_border);

    let font = load_ttf_font("../assets/MinimalPixelv2.ttf").await.unwrap();
    GameState {
        players,
        balls,
        texts,
        borders,
        score: 0,
        font,
    }
}

fn random_player_index(num_players: usize) -> usize {
    rand::thread_rng().gen_range(0..num_players)
}

fn new_ball_offset() -> f32 {
    rand::thread_rng().gen_range(((-PLATFORM_START_W / 2.) * 0.5)..=((PLATFORM_START_W / 2.) * 0.5))
}
