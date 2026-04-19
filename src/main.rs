use core::panic;

use macroquad::prelude::*;
#[derive(Debug)]
enum STATE {
    MENU,
    SETTINGS,
    GAMEPLAY(i32),
}

#[macroquad::main("Project: VoidMarch")]
pub async fn main() {
    let state: STATE = STATE::MENU;

    loop {
        clear_background(BLACK);

        match state {
            STATE::MENU => Draw_Menu(),
            _ => {
                panic!("not implemented {:?}", state)
            }
        }

        next_frame().await
    }
}

fn Draw_Menu() {
    draw_text(
        "Project: Voidmarch",
        screen_width() / 2.0 - get_text_center("Project: VoidMarch", None, 64, 1.0, 0.0).x,
        (screen_height() / 2.0) - 120.0,
        64.0,
        WHITE,
    );

    draw_circle(screen_width() / 2.0, screen_height() / 2.0, 100.0, RED);
    draw_text(
        ":D",
        screen_width() / 2.0 - get_text_center(":D", None, 64, 1.0, 0.0).x,
        (screen_height() / 2.0) - get_text_center(":D", None, 64, 1.0, 0.0).y,
        64.0,
        WHITE,
    );
}
