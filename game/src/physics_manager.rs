use bevy::{
    prelude::*
};

#[derive(Component)]
pub struct Physics{
    pub velocity: Vec2,
}
impl Default for Physics {
    fn default() -> Self {
        Physics {
            velocity: Vec2::new(0.0, 0.0),
        }
    }
}

pub fn apply_forces(
    mut query: Query<(&mut Transform, &Physics)>,
    time_step: Res<FixedTime>,
) {
    for (mut transform, physics) in &mut query {
        transform.translation.x += physics.velocity.x * time_step.period.as_secs_f32();
        transform.translation.y += physics.velocity.y * time_step.period.as_secs_f32();
    }
}