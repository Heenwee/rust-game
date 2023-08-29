use bevy::prelude::*;

use crate::physics_manager::*;

#[derive(Component)]
pub struct Player;

const STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const SQUARE_SCALE: Vec3 = Vec3::new(0.25, 0.25, 0.25);
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
        Physics{
            ..default()
        },
        Collision{
            collider: Collider::PolygonCollider {
                points: vec![
                    Vec2::new(-0.25/2.0, -0.25/2.0),
                    Vec2::new(0.25/2.0, -0.25/2.0),
                    Vec2::new(0.25/2.0, 0.25/2.0),
                    Vec2::new(-0.25/2.0, 0.25/2.0),
                ]
            },
            ..default()
        }
    ));
}

pub fn player_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Physics, With<Player>>,
) {
    // reset direction at the start of every frame and then set direction with input
    let mut x_dir: i32 = 0;
    if keys.pressed(KeyCode::Left){
        x_dir += -1;
    }
    if keys.pressed(KeyCode::Right){
        x_dir += 1;
    }

    for mut physics in &mut query.iter_mut() {
        let d = x_dir as f32;
        physics.velocity.x = d*PLAYER_SPEED;
    }
}