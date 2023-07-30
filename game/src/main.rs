use bevy::prelude::*;

pub mod particles;
use particles::*;

const STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const SQUARE_SCALE: Vec3 = Vec3::new(1.0, 1.0, 1.0); 

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup,(
            setup.before(particle_effects),
            particle_effects,
        ))
        .run();
}

#[derive(Component)]
struct Player;

fn setup (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    println!("starting...");
    // create camera
    commands.spawn(Camera2dBundle::default());

    // load player
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/square.png"),
            transform: Transform::from_translation(STARTING_POSITION).with_scale(SQUARE_SCALE),
            ..default()
        },
        Player,
        Particle {
            lifetime: 5.0,
            ..default()
        },
    ));
}