use bevy::prelude::*;

use crate::util::*;
use super::Velocity;

#[derive(Debug, Default, PartialEq, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct FlockMemberMarker;

#[derive(Debug, Default, PartialEq, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct FlockMemberParams {
    pub max_speed: f32,
    pub safe_radius: f32
}

#[derive(Bundle, Clone, Debug)]
pub struct FlockMember {
    pub marker: FlockMemberMarker,
    pub velocity: Velocity,
    pub params: FlockMemberParams
}

#[derive(Debug, Default, PartialEq, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct Flock {
    pub flock_radius: f32,
    pub alignment_strength: f32,
    pub cohesion_strength: f32,
    pub separation_strength: f32
}

impl Default for FlockMember {
    fn default() -> Self {
        FlockMember {
            marker: FlockMemberMarker,
            velocity: Default::default(),
            params: FlockMemberParams {
                max_speed: 200.0,
                safe_radius: 50.0
            }
        }
    }
}

#[derive(Default)]
pub struct FlockingPlugin {
    include_wrapping: bool
}

impl FlockingPlugin {
    pub fn new(include_wrapping: bool) -> FlockingPlugin {
        FlockingPlugin {
            include_wrapping
        }
    }

    pub fn with_wrapping() -> FlockingPlugin {
        Self::new(true)
    }

    #[inline]
    fn calculate_alignment(max_speed: f32, average_forward: Vec2) -> Vec2 {
        let mut alignment: Vec2  = average_forward / max_speed;

        if alignment.length_squared() > 1.0 {
            alignment = alignment.normalize();
        }
    
        alignment
    }

    #[inline]
    fn calculate_cohesion(position: Vec2, average_position: Vec2, flock_radius: f32) -> Vec2 {
        let mut cohesion: Vec2 = average_position - position;
    
        if cohesion.length_squared() < flock_radius * flock_radius {
            cohesion /= flock_radius;
        } else {
            cohesion = cohesion.normalize();
        }
    
        cohesion
    }

    #[inline]
    fn calculate_separation(entity: &Entity, params: &FlockMemberParams, position: Vec2, boids: &[(&Entity, Vec2, FlockMemberParams)]) -> Vec2 {
        let entity_id = entity.id();
        let mut separation = Vec2::zero();

        for (other_entity, other_position, other_params) in boids.into_iter() {
            if other_entity.id() != entity_id {
                let difference: Vec2 = position - *other_position;
                let distance_squared = difference.length_squared();
                let minimum_distance = params.safe_radius + other_params.safe_radius;

                if distance_squared < minimum_distance * minimum_distance {
                    separation += difference.normalize() * (minimum_distance - distance_squared.sqrt()) / minimum_distance;
                }
            }
        }

        if separation.length_squared() > 1.0 {
            separation = separation.normalize();
        }

        separation
    }


    fn flocking(time: Res<Time>, windows: Res<Windows>, query: Query<(&Flock, &Children)>, mut child_query: Query<(&mut Velocity, &GlobalTransform, &FlockMemberParams), With<FlockMemberMarker>>) {
        let bounds: Bounds<Vec2> = windows.get_primary().unwrap().into();

        for (flock, children) in query.iter() {
            let mut average_position = Vec2::zero();
            let mut average_forward = Vec2::zero();
            let mut boids = Vec::new();

            for child in children.iter() {
                if let Ok((velocity, transform, params)) = child_query.get_mut(*child) {
                    let mut current_average = average_position;
                    if boids.len() > 0 {
                        current_average = (current_average / boids.len() as f32).bound_to(Vec2::zero(), bounds);
                    }

                    average_position += transform.translation.truncate().bound_to(current_average, bounds);
                    average_forward += velocity.0;
                    boids.push((child, transform.translation.truncate(), params.clone()));
                }
            }

            if boids.len() > 0 {
                average_position /= boids.len() as f32;
                average_forward /= boids.len() as f32;

                for (_, mut position, _) in boids.iter_mut() {
                    position.clone_from(&position.bound_to(average_position, bounds));
                }

                for child in children.iter() {
                    if let Ok((mut velocity, transform, params)) = child_query.get_mut(*child) {
                        let position = transform.translation.truncate().bound_to(average_position, bounds);

                        let alignment = flock.alignment_strength * Self::calculate_alignment(params.max_speed, average_forward);
                        let cohesion = flock.cohesion_strength * Self::calculate_cohesion(position, average_position, flock.flock_radius);
                        let separation = flock.separation_strength * Self::calculate_separation(child, params, position, &boids);

                        let mut new_velocity: Vec2 = velocity.0 + params.max_speed * time.delta_seconds() * (alignment + cohesion + separation);
                        if new_velocity.length_squared() > params.max_speed * params.max_speed {
                            new_velocity = new_velocity.normalize() * params.max_speed;
                        }

                        velocity.0 = new_velocity;
                    }
                }
            }
        }
    }

    fn wrapping(windows: Res<Windows>, mut query: Query<&mut GlobalTransform, With<FlockMemberMarker>>) {
        let bounds: Bounds<Vec2> = windows.get_primary().unwrap().into();
        for mut transform in query.iter_mut() {
            let current_layer = transform.translation.z;
            transform.translation = transform.translation.truncate().bound_to(Vec2::zero(), bounds).extend(current_layer);
        }
    }
}

impl Plugin for FlockingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(Self::flocking.system());

        if self.include_wrapping {
            app.add_system_to_stage(stage::LAST, Self::wrapping.system());
        }
    }
}
