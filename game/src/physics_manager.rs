use bevy::{
    prelude::*
};

const PHYSICS_SPEED: f32 = 40.0;
//////////////////////////////////////////////////////////////////////////
#[derive(Component)]
pub struct Physics{
    pub velocity: Vec2,
    pub angular_velocity: f32,
    pub gravity: f32,

    pub immovable: bool,

    pub force_sum: Vec2,
    pub torque_sum: f32,
}
impl Physics {
    pub fn add_force(&mut self, force: Vec2) {
        self.force_sum += force;
    }
    pub fn add_torque(&mut self, torque: f32) {
        self.torque_sum += torque;
    }
}
impl Default for Physics {
    fn default() -> Self {
        Physics {
            velocity: Vec2::new(0.0, 0.0),
            angular_velocity: 0.0,
            gravity: -9.81,

            immovable: false,

            force_sum: Vec2::new(0.0, 0.0),
            torque_sum: 0.0, 
        }
    }
}

pub fn apply_forces(
    mut query: Query<(&mut Transform, &mut Physics)>,
    time_step: Res<FixedTime>,
) {
    for (mut transform, mut physics) in &mut query {
        if !physics.immovable {
            // apply gravity
            let grav: f32 = physics.gravity;
            physics.add_force(Vec2::new(0.0, grav));
            // apply other forces
            let forces: Vec2 = physics.force_sum;
            physics.velocity += forces * time_step.period.as_secs_f32();
            // reset forces
            physics.force_sum = Vec2::ZERO;
    
            //apply torque
            let torque: f32 = physics.torque_sum;
            physics.angular_velocity += torque;
            // reset torque
            physics.torque_sum = 0.0;
    
            // move object based on velocity
            transform.translation.x += physics.velocity.x * time_step.period.as_secs_f32() * PHYSICS_SPEED;
            transform.translation.y += physics.velocity.y * time_step.period.as_secs_f32() * PHYSICS_SPEED;
            // rotate objects based on torque
            transform.rotate_z(physics.angular_velocity * time_step.period.as_secs_f32())
        }
    }
}
//////////////////////////////////////////////////////////////////////////
pub enum Collider {
    CircleCollider {
        radius: f32
    },
    PolygonCollider {
        points: Vec<Vec2>
    },
}
#[derive(Component)]
pub struct Collision {
    pub collider: Collider,
    pub trigger: bool,
}
impl Default for Collision {
    fn default() -> Self {
        Collision {
            collider: Collider::CircleCollider {
                radius: 1.0
            },
            trigger: false,
        }
    }
}
//////////////////////////////////////////////////////////////////////////