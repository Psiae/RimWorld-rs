use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::{Camera2dBundle, Commands, default, PluginGroup, WindowDescriptor};
use bevy::window::{PresentMode, WindowPlugin};

mod app_window;
mod main_game;
mod main_menu;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(app_window::get_window_plugin()))
        .add_state(main_game::MainGameState::Menu)
        .add_plugin(main_menu::MainMenuPlugin)
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}