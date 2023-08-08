use bevy::prelude::*;

use crate::physics_manager::*;

#[derive(Component)]
pub struct Player;

const STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const SQUARE_SCALE: Vec3 = Vec3::new(0.5, 0.5, 0.5);
const PLAYER_SPEED: f32 = 5.0;

pub fn setup (
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
        Physics{..default()},
    ));
}

pub fn player_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Physics, With<Player>)>,
) {
    let mut x_dir: i32 = 0;

    if keys.pressed(KeyCode::Left){
        x_dir += -1;
    }
    if keys.pressed(KeyCode::Right){
        x_dir += 1;
    }

    for mut physics in &mut query {
        let d = x_dir as f32;
        physics.velocity.x = d*PLAYER_SPEED;
    }
}