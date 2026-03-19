use bevy::{
    prelude::*, window::{PresentMode, PrimaryWindow, WindowMode, WindowResolution}
};

fn main() {
    // Create the window
    let window_settings = Window {
        resolution: WindowResolution::new(800.0, 600.0),
        title: "P00 Window example".to_string(),
        mode: WindowMode::Windowed,
        present_mode: PresentMode::AutoVsync,
        ..default()
    };
    // Create the app
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(window_settings),
        ..default()
    }));
    app.add_systems(Update, handle_io);
    app.init_resource::<WinState>();
    app.run();
}

#[derive(Default, Resource)]
struct WinState(bool);

fn handle_io(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<WinState>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
){
    if let Ok(mut window) = primary_window.get_single_mut() {
        if keyboard.just_pressed(KeyCode::KeyZ){
            state.0 = !state.0;
            window.set_maximized(state.0);
        }
        if keyboard.just_pressed(KeyCode::KeyX){
            window.set_minimized(true);
        }
    }
}