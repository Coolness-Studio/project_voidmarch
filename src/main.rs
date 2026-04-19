use macroquad::prelude::*;

mod ui;
use ui::draw_menu;
#[derive(Debug)]
enum State {
    Menu,
    Settings,
    Level(u8),
}

#[macroquad::main("Project: VoidMarch")]
pub async fn main() {
    let state: State = State::Menu;

    loop {
        clear_background(BLACK);

        match state {
            State::Menu => draw_menu(),
            State::Settings => ,
            State::Level(id) => {
                // This should prob be a function instead
                }
            },
        }

        next_frame().await
    }
}
