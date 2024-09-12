#![warn(clippy::all, clippy::pedantic)]
mod snake;
mod game;
use crate::game::Game;
use piston_window::*;

const BLOCK_SIZE: f64 = 25.0;
const WIDTH: i32 = 20;
const HEIGHT: i32 = 20;

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct Block {
    x: i32,
    y: i32,
}

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Snake Game", [WIDTH as f64 * BLOCK_SIZE, HEIGHT as f64 * BLOCK_SIZE])
            .exit_on_esc(true)
            .build()
            .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let font_path = assets.join("FiraSans-Regular.ttf");

    let mut glyphs = window
        .load_font(font_path)
        .expect("Could not load font");

    let mut game = Game::new();
    let mut events = Events::new(EventSettings::new()).ups(8);

    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if game.game_over {
                match key {
                    Key::R => game.reset(), // Restart the game
                    Key::Q => return,       // Quit the game
                    _ => {}
                }
            } else {
                game.snake.change_direction(match key {
                    Key::Up => Direction::Up,
                    Key::Down => Direction::Down,
                    Key::Left => Direction::Left,
                    Key::Right => Direction::Right,
                    _ => game.snake.dir.clone(),
                });
            }
        }

        if let Some(_) = e.update_args() {
            game.update();
        }

        window.draw_2d(&e, |c, g, device| {
            clear([0.0, 0.0, 0.0, 1.0], g);

            // Draw Snake
            for block in &game.snake.body {
                rectangle(
                    [0.0, 1.0, 0.0, 1.0],
                    [block.x as f64 * BLOCK_SIZE, block.y as f64 * BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE],
                    c.transform,
                    g,
                );
            }

            // Draw Food
            rectangle(
                [1.0, 0.0, 0.0, 1.0],
                [game.food.x as f64 * BLOCK_SIZE, game.food.y as f64 * BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE],
                c.transform,
                g,
            );

            // Draw Game Over Modal
            if game.game_over {
                let transform = c.transform.trans((WIDTH as f64 * BLOCK_SIZE) / 4.0, (HEIGHT as f64 * BLOCK_SIZE) / 2.0);
                text(
                    [1.0, 1.0, 1.0, 1.0], 32, "Game Over!", &mut glyphs, transform, g
                ).unwrap();

                let transform = c.transform.trans((WIDTH as f64 * BLOCK_SIZE) / 6.0, (HEIGHT as f64 * BLOCK_SIZE) / 1.5);
                text(
                    [1.0, 1.0, 1.0, 1.0], 16, "Press 'R' to Restart or 'Q' to Quit", &mut glyphs, transform, g
                ).unwrap();
            }

            // Update glyphs cache
            glyphs.factory.encoder.flush(device);
        });
    }
}
