use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::Rng;

#[derive(Component, Reflect, Default)] // Reflect and Default for the inspector plugin
#[reflect(Component)]
struct RotateComponent {
    speed: f32,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    //Introduce inspector plugin
    app.add_plugins(WorldInspectorPlugin::new());
    //Register component, so the inpector cna work with it
    app.register_type::<RotateComponent>();
    app.add_systems(Startup, setup);
    app.add_systems(Update, rotate_shapes);
    app.run();
}

fn setup(mut commands: Commands) {
    // Camera creation
    commands.spawn(Camera2d::default());

    // Add some squares with random colors
    let mut rng = rand::thread_rng();
    for i in 0..5 {
        let x = rng.gen_range(-400.0..400.0);
        let y = rng.gen_range(-300.0..300.0);
        let color = Color::linear_rgb(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
        );

        commands.spawn((
            Sprite {
                color,
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            Name::new(format!("Sprite {}", i)), //Name the entity for inspector clarity. Else the name would be EntityXvY ...
            Transform::from_xyz(x, y, 0.0), // Random position
            RotateComponent {
                speed: rng.gen_range(-0.5..2.0),
            },
        ));
    }
}


// Basic entity rotation system
fn rotate_shapes(time: Res<Time>, mut query: Query<(&mut Transform, &RotateComponent)>) {
    for (mut transform, rotate) in &mut query {
        transform.rotate_z(rotate.speed * time.delta_secs());
    }
} 