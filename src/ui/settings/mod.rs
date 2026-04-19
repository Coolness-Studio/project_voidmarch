/// This file contains everything required for the settings menu, and relevant logic

#[derive(Debug, Clone, Copy)]
pub enum SettingsMenu {
    Audio,
    Graphics,
    Display,
}


pub fn draw_settings(menu: &SettingsMenu) {
    todo!("implement settings submenu {:?}", menu)
}
