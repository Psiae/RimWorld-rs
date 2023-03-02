use bevy::app::App;
use bevy::prelude::{default, Plugin, Query, Res, ResMut, Window, WindowDescriptor, WindowPlugin, Windows};
use bevy::window::PresentMode;

pub(crate) struct AppWindowPlugin;

impl Plugin for AppWindowPlugin {

    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_window);
    }
}

fn setup_window(mut windows: ResMut<Windows>) {
    let mut window = windows.primary_mut();
    window.set_title("RimWorld-rs".to_string())
}