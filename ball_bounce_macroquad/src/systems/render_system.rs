use crate::*;
use macroquad::prelude::*;

pub fn render(game_state: &GameState) {
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
