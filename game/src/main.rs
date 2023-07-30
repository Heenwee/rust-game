use bevy::prelude::*;

const STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const SQUARE_SCALE: Vec3 = Vec3::new(1.0, 1.0, 1.0); 

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup,
            setup,
        )
        .run();
}

fn setup (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    println!("starting...");
    // create camera
    commands.spawn(Camera2dBundle::default());

    // load sprite
    commands.spawn(
        SpriteBundle {
            texture: asset_server.load("sprites/square.png"),
            transform: Transform::from_translation(STARTING_POSITION).with_scale(SQUARE_SCALE),
            ..default()
    });
}