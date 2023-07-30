use bevy::prelude::*;

#[derive(Component)]
pub struct Particle {
    pub lifetime: f32,
    pub burst: i32
}

impl Default for Particle {
    fn default() -> Self {
        Particle {
            lifetime: 0.0,
            burst: 0,
        }
    }
}

pub fn particle_effects(
    particle_query: Query<&Particle>,
) {
    for particle in &particle_query {
        println!("lifetime: {0}, burst: {1}", particle.lifetime, particle.burst);
    }
    if particle_query.is_empty() {
        println!("empty");
    }
}