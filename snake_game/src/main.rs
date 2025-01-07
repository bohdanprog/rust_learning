mod game;
mod snake;
mod render;

use crate::game::Game;
use crate::snake::Direction;
use piston_window::*;
use std::process;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake Game", [500.0, 500.0])
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
        handle_input(&mut game, &e);

        if let Some(_) = e.update_args() {
            game.update();
        }

        window.draw_2d(&e, |c, g, device| {
            clear(render::BG_COLOR, g);

            if game.welcome {
                render::draw_welcome_screen(&mut glyphs, c, g);
            } else {
                render::draw_snake(&game, c, g);
                render::draw_food(&game, c, g);

                if game.game_over {
                    render::draw_game_over(&mut glyphs, c, g);
                }
            }

            glyphs.factory.encoder.flush(device);
        });
    }
}

fn handle_input(game: &mut Game, e: &Event) {
    if let Some(Button::Keyboard(key)) = e.press_args() {
        if game.welcome {
            if key == Key::Space {
                game.welcome = false;
            }
        } else if game.game_over {
            match key {
                Key::R => game.reset(),
                Key::Q => process::exit(0),
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
}

