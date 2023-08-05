use bevy::prelude::*;

pub mod player;
pub mod particles;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup,(
            player::setup.before(particles::particle_effects),
            particles::particle_effects,
        ))
        .run();
}