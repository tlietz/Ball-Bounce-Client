use crate::*;
use macroquad::prelude::*;

pub async fn initial_game_state() -> GameState {
    let players = create_players();
    let balls = create_balls(random_player_index(players.len()));
    let texts = create_texts();
    let borders = create_borders();
    let font = load_ttf_font("assets/MinimalPixelv2.ttf").await.unwrap();
    GameState {
        players,
        balls,
        texts,
        borders,
        score: 0,
        font,
    }
}

fn create_players() -> Vec<Player> {
    let player1 = Player {
        position: Position {
            x: SCREEN_W / 4.,
            y: SCREEN_H - (PLATFORM_FLOAT_H + PLATFORM_HEIGHT),
        },
        speed: SCREEN_W / 2.,
        width: PLATFORM_START_W,
        height: PLATFORM_HEIGHT,
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
            y: SCREEN_H - (PLATFORM_FLOAT_H + PLATFORM_HEIGHT),
        },
        speed: SCREEN_W / 2.,
        width: PLATFORM_START_W,
        height: PLATFORM_HEIGHT,
        control: Control {
            left: KeyCode::Left,
            right: KeyCode::Right,
        },
        color: PURPLE,
    };
    players.push(player2);
    players
}

fn create_balls(player_index: usize) -> Vec<Ball> {
    // Ball initialized sitting on the top of a random player paddle,
    // randomly deviated from the center
    let offset = rand_ball_offset();
    let ball = Ball {
        position: Position {
            x: offset + SCREEN_W / 2.,
            y: init_ball_offset(BALL_START_RADIUS),
        },
        velocity: Velocity { dx: 0., dy: 0. },
        speed: BALL_START_SPEED,
        radius: BALL_START_RADIUS,
        state: BallState::Ready {
            player_entity_index: player_index,
            offset,
        },
        color: RED,
    };
    vec![ball]
}

fn create_texts() -> Vec<Text> {
    let text = Text {
        text: "Press spacebar to start",
        position: Position {
            x: SCREEN_W * 0.1,
            y: SCREEN_H * 0.4,
        },
        font_size: 40,
    };
    vec![text]
}

fn create_borders() -> Vec<Border> {
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
    borders
}
