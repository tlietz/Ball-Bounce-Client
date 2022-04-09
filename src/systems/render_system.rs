use crate::*;
use macroquad::prelude::*;

pub fn render(game_state: &GameState) {
    render_background();

    for ball in &game_state.balls {
        render_ball(ball)
    }
    for player in &game_state.players {
        render_player(player)
    }
    for border in &game_state.borders {
        render_border(border)
    }
    if let BallState::Ready {
        player_entity_index: _,
        offset: _,
    } = game_state.balls[0].state
    {
        for text in &game_state.texts {
            render_text(text, game_state.font)
        }
    }

    draw_score(&game_state);
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

fn draw_score(game_state: &GameState) {
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
