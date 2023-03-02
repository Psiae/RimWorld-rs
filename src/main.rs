use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::{Camera2dBundle, Commands};
use bevy::window::{PresentMode};
use bevy_editor_pls::EditorPlugin;

mod app_window;
mod colony;
mod main_game;
mod main_menu;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        .add_plugin(app_window::AppWindowPlugin)
        .add_state(main_game::MainGameState::Menu)
        .add_plugin(main_menu::MainMenuPlugin)
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}