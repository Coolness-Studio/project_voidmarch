use macroquad::prelude::*;

mod ui;
use ui::draw_menu;
#[derive(Debug)]
enum State {
    Menu,
    Settings,
    Level(i32),
}

#[macroquad::main("Project: VoidMarch")]
pub async fn main() {
    let state: State = State::Menu;

    loop {
        clear_background(BLACK);

        match state {
            State::Menu => draw_menu(),
            _ => {
                unimplemented!("{:?} (game state)", state);
            }
        }

        next_frame().await
    }
}
