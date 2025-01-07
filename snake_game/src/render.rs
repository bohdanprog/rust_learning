use crate::game::Game;
use piston_window::*;

const BLOCK_SIZE: f64 = 25.0;
const WIDTH: i32 = 20;
const HEIGHT: i32 = 20;

pub const BG_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const SNAKE_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const FOOD_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const TEXT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub fn draw_snake(game: &Game, c: Context, g: &mut G2d) {
    for block in &game.snake.body {
        rectangle(
            SNAKE_COLOR,
            [block.x as f64 * BLOCK_SIZE, block.y as f64 * BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE],
            c.transform,
            g,
        );
    }
}

pub fn draw_food(game: &Game, c: Context, g: &mut G2d) {
    rectangle(
        FOOD_COLOR,
        [game.food.x as f64 * BLOCK_SIZE, game.food.y as f64 * BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE],
        c.transform,
        g,
    );
}

pub fn draw_game_over(glyphs: &mut Glyphs, c: Context, g: &mut G2d) {
    let game_over_text = "Game Over!";
    let restart_prompt = "Press 'R' to Restart or 'Q' to Quit";

    let game_over_font_size = 32;
    let prompt_font_size = 16;

    let game_over_transform = c.transform.trans(
        (WIDTH as f64 * BLOCK_SIZE) / 2.0 - (game_over_text.len() as f64 * game_over_font_size as f64 * 0.3) / 2.0,
        (HEIGHT as f64 * BLOCK_SIZE) / 2.0,
    );
    text(TEXT_COLOR, game_over_font_size, game_over_text, glyphs, game_over_transform, g).unwrap();

    let prompt_transform = c.transform.trans(
        (WIDTH as f64 * BLOCK_SIZE) / 2.0 - (restart_prompt.len() as f64 * prompt_font_size as f64 * 0.3) / 2.0,
        (HEIGHT as f64 * BLOCK_SIZE) / 2.0 + 50.0,
    );
    text(
        TEXT_COLOR,
        prompt_font_size,
        restart_prompt,
        glyphs,
        prompt_transform,
        g,
    ).unwrap();
}

pub fn draw_welcome_screen(glyphs: &mut Glyphs, c: Context, g: &mut G2d) {
    let welcome_text = "Welcome to the game!";
    let prompt_text = "When you are ready, press Space.";

    let welcome_font_size = 32;
    let prompt_font_size = 16;

    let welcome_transform = c.transform.trans(
        (WIDTH as f64 * BLOCK_SIZE) / 2.0 - (welcome_text.len() as f64 * welcome_font_size as f64 * 0.3) / 2.0,
        (HEIGHT as f64 * BLOCK_SIZE) / 2.0 - 50.0,
    );
    text(TEXT_COLOR, welcome_font_size, welcome_text, glyphs, welcome_transform, g).unwrap();

    let prompt_transform = c.transform.trans(
        (WIDTH as f64 * BLOCK_SIZE) / 2.0 - (prompt_text.len() as f64 * prompt_font_size as f64 * 0.3) / 2.0,
        (HEIGHT as f64 * BLOCK_SIZE) / 2.0 + 20.0,
    );
    text(TEXT_COLOR, prompt_font_size, prompt_text, glyphs, prompt_transform, g).unwrap();
}

