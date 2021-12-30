extern crate rand;
use macroquad::{color, prelude::*};
use rand::Rng;

pub const SCREEN_W: f32 = 600.0;
pub const SCREEN_H: f32 = 600.0;

pub const BALL_START_SPEED: f32 = SCREEN_H / 1.5;
pub const PLATFORM_FLOAT_H: f32 = SCREEN_H * 0.025;

pub const PLATFORM_START_W: f32 = SCREEN_W * 0.1;

#[derive(Debug)]
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
    color: color::Color,
}

#[derive(Debug)]
struct Player {
    position: Position,
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

#[derive(Debug)]
enum Entity {
    Ball(Ball),
    Player(Player),
    Text(Text),
    Border(Border),
}

type EntityIndex = usize;
pub struct GameState {
    entities: Vec<Option<Entity>>,
    players: Vec<EntityIndex>,
    balls: Vec<EntityIndex>,
    texts: Vec<EntityIndex>,
    borders: Vec<EntityIndex>,
    score: u32,
    font: Font,
}

pub async fn initial_game_state() -> GameState {
    let mut entity_index: usize = 0;
    let mut entities = Vec::new();

    let platform_height = SCREEN_W * 0.02;
    let player = Some(Entity::Player(Player {
        position: Position {
            x: SCREEN_W / 2.,
            y: SCREEN_H - (PLATFORM_FLOAT_H + platform_height),
        },
        width: PLATFORM_START_W,
        height: platform_height,
        control: Control {
            left: KeyCode::A,
            right: KeyCode::D,
        },
        color: WHITE,
    }));
    entities.push(player);
    let players = vec![entity_index];
    entity_index += 1;

    // Ball initialized sitting on the top of the player platform,
    // randomly deviated from the center
    let ball_radius = platform_height * 0.5;
    let ball = Some(Entity::Ball(Ball {
        position: Position {
            x: new_ball_offset() + SCREEN_W / 2.,
            y: SCREEN_H - (ball_radius + platform_height + PLATFORM_FLOAT_H),
        },
        velocity: Velocity { dx: 0., dy: 0. },
        radius: ball_radius,
        color: RED,
    }));
    entities.push(ball);
    let balls = vec![entity_index];
    entity_index += 1;

    let text = Some(Entity::Text(Text {
        text: "Press spacebar to start",
        position: Position {
            x: SCREEN_W * 0.1,
            y: SCREEN_H * 0.4,
        },
        font_size: 40,
    }));
    entities.push(text);
    let texts = vec![entity_index];

    let border_width = SCREEN_W * 0.025;
    let left_border = Some(Entity::Border(Border {
        position: Position { x: 0., y: 0. },
        width: border_width,
        height: SCREEN_H,
        color: GRAY,
    }));
    entities.push(left_border);
    let mut borders = vec![entity_index];
    entity_index += 1;

    let right_border = Some(Entity::Border(Border {
        position: Position {
            x: SCREEN_W - border_width,
            y: 0.,
        },
        width: border_width,
        height: SCREEN_H,
        color: GRAY,
    }));
    entities.push(right_border);
    borders.push(entity_index);
    entity_index += 1;

    let top_border = Some(Entity::Border(Border {
        position: Position { x: 0., y: 0. },
        width: SCREEN_W,
        height: border_width,
        color: GRAY,
    }));
    entities.push(top_border);
    borders.push(entity_index);

    let font = load_ttf_font("../assets/MinimalPixelv2.ttf").await.unwrap();
    GameState {
        entities,
        players,
        balls,
        texts,
        borders,
        score: 0,
        font,
    }
}

fn new_ball_offset() -> f32 {
    rand::thread_rng().gen_range(((-PLATFORM_START_W / 2.) * 0.5)..=((PLATFORM_START_W / 2.) * 0.5))
}

// Launches the ball from the platform by changing its velocity corresponding to where the ball hits the platform.
// This is used when the ball hits the platform, or when launching the ball with spacebar.
// The ball is launched at a maximum angle with respect to the normal.
fn launch_ball(
    platform_x: f32,
    platform_width: f32,
    ball_x: f32,
    ball_speed: f32,
    ball_vel: &mut Vec2,
) {
    // maximum offset calculated to determine angle of launch
    let max_offset = 0.7;
    let mut percent_offset = (ball_x - platform_x) / (platform_width / 2.);
    if percent_offset < -max_offset {
        percent_offset = -max_offset;
    } else if percent_offset > max_offset {
        percent_offset = max_offset;
    } else if percent_offset == 0.00 {
        // if the ball hits the center of platform, launch at a random angle
        percent_offset = rand::thread_rng().gen_range(-0.10..=0.10)
    }

    ball_vel.x = percent_offset * ball_speed;
    ball_vel.y = -(1. - percent_offset.abs()) * ball_speed;
}

fn render_background() {
    draw_rectangle(0.0, 0.0, SCREEN_W, SCREEN_H, DARKBLUE);
}

fn render_ball(ball: &Ball) {
    draw_circle(ball.position.x, ball.position.y, ball.radius, ball.color);
}

fn render_player(player: &Player) {
    draw_rectangle(
        player.position.x - player.width / 2.,
        SCREEN_H - (player.height + PLATFORM_FLOAT_H),
        player.width,
        player.height,
        player.color,
    );
}

fn render_text(text: &Text, font: Font) {
    draw_text_ex(
        text.text,
        text.position.x,
        text.position.y,
        TextParams {
            font_size: text.font_size,
            font,
            color: BLACK,
            ..Default::default()
        },
    );
}

fn render_border(border: &Border) {
    draw_rectangle(
        border.position.x,
        border.position.y,
        border.width,
        border.height,
        border.color,
    );
}

pub fn render_system(game_state: &GameState) {
    render_background();

    // renders all entities
    for entity in &game_state.entities {
        match entity {
            Some(Entity::Ball(ball)) => render_ball(ball),
            Some(Entity::Player(player)) => render_player(player),
            Some(Entity::Text(text)) => render_text(text, game_state.font),
            Some(Entity::Border(border)) => render_border(border),
            None => continue,
        }
    }

    // draw score
    draw_text_ex(
        &game_state.score.to_string(),
        SCREEN_W * 0.05,
        SCREEN_W * 0.15,
        TextParams {
            font_size: 50,
            font: game_state.font,
            ..Default::default()
        },
    );
}
