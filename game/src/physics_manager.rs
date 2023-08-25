use bevy::{
    prelude::*
};

const PHYSICS_SPEED: f32 = 50.0;

#[derive(Component)]
pub struct Physics{
    pub velocity: Vec2,
    pub gravity: f32,
}
impl Default for Physics {
    fn default() -> Self {
        Physics {
            velocity: Vec2::new(0.0, 0.0),
            gravity: -9.81,
        }
    }
}

pub fn apply_forces(
    mut query: Query<(&mut Transform, &mut Physics)>,
    time_step: Res<FixedTime>,
) {
    for (mut transform, mut physics) in &mut query {
        physics.velocity.y += &physics.gravity * time_step.period.as_secs_f32();

        transform.translation.x += &physics.velocity.x * time_step.period.as_secs_f32() * PHYSICS_SPEED;
        transform.translation.y += &physics.velocity.y * time_step.period.as_secs_f32() * PHYSICS_SPEED;
    }
}