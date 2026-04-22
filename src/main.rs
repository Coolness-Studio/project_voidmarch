use macroquad::prelude::*;
use std::fmt::Display;

mod ui;
use ui::{SettingsMenu, draw_menu, draw_settings};

mod levels;
use levels::draw_level;

mod assets;
use assets::*;

#[derive(Debug)]
enum State {
    Menu,
    Settings(SettingsMenu),
    Level(u8),
}

#[macroquad::main("Project: VoidMarch")]
pub async fn main() {
    let mut state: State = State::Menu;

    let assets = Assets::load().await;

    loop {
        clear_background(BLACK);
        let mut return_to_menu = false;

        match &mut state {
            State::Menu => draw_menu(&mut state),
            State::Settings(menu) => draw_settings(&menu),
            State::Level(id) => {
                if let Err(err) = draw_level(*id, &assets) {
                    draw_level_error(*id, &err);

                    if is_key_pressed(KeyCode::Escape) {
                        return_to_menu = true;
                    }
                }
            }
        }

        if return_to_menu {
            state = State::Menu;
        }

        next_frame().await
    }
}

fn draw_level_error(id: u8, error: &impl Display) {
    let title = "Level error";
    let message = format!("Level {id}: {error}");
    let hint = "Press Escape to return to the menu";

    draw_rectangle(
        32.0,
        screen_height() * 0.5 - 96.0,
        screen_width() - 64.0,
        192.0,
        Color::new(0.08, 0.05, 0.05, 0.92),
    );
    draw_text(title, 56.0, screen_height() * 0.5 - 36.0, 40.0, RED);
    draw_text(&message, 56.0, screen_height() * 0.5 + 8.0, 28.0, WHITE);
    draw_text(hint, 56.0, screen_height() * 0.5 + 52.0, 24.0, GRAY);
}
