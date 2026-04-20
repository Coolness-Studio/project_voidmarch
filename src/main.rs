use macroquad::prelude::*;

mod ui;
use ui::{SettingsMenu, draw_menu, draw_settings};

mod levels;
use levels::draw_level;

mod assets;

#[derive(Debug)]
enum State {
    Menu,
    Settings(SettingsMenu),
    Level(u8),
}

#[macroquad::main("Project: VoidMarch")]
pub async fn main() {
    let state: State = State::Menu;

    loop {
        clear_background(BLACK);

        match state {
            State::Menu => draw_menu(),
            State::Settings(menu) => draw_settings(&menu),
            State::Level(id) => draw_level(id),
        }

        next_frame().await
    }
}
