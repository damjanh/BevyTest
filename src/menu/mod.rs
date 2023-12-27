use bevy::prelude::*;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(main_menu);
    }
}

pub fn main_menu() {
    println!("Main Menu here!");
}
