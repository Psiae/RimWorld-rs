use bevy::app::App;
use bevy::prelude::{default, Plugin, WindowDescriptor, WindowPlugin};
use bevy::render::camera::RenderTarget::Window;
use bevy::window::PresentMode;

pub(crate) fn get_window_plugin() -> WindowPlugin {
    return WindowPlugin {
        window: WindowDescriptor {
            title: "RimWorld-rs".to_string(),
            width: 1280.,
            height: 720.,
            present_mode: PresentMode::AutoVsync,
            ..default()
        },
        ..default()
    }
}