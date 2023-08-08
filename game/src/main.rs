use bevy::{
    prelude::*,
};

pub mod player;
pub mod particles;
pub mod physics_manager;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
        .add_systems(Startup,(
            player::setup.before(particles::particle_effects),
            particles::particle_effects,
        ))
        .add_systems(Update,(
            player::player_input,
        ))
        .add_systems(FixedUpdate, (
            physics_manager::apply_forces,
        ))
        .run();
}