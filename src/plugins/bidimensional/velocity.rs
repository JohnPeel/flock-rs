use bevy::prelude::*;

#[derive(Debug, Default, PartialEq, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct Velocity(pub Vec2);

impl From<Vec2> for Velocity {
    fn from(x: Vec2) -> Velocity {
        Velocity(x)
    }
}

impl Into<Vec2> for Velocity {
    fn into(self) -> Vec2 {
        self.0
    }
}

#[derive(Clone, Debug)]
pub struct VelocityPlugin;

fn movement(time: Res<Time>, mut query: Query<(&mut GlobalTransform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        let old_position = transform.translation;
        transform.translation += (velocity.0 * time.delta_seconds()).extend(0.0);

        if transform.translation.x.is_nan() {
            if old_position.x.is_nan() {
                transform.translation.x = 0.0;
            } else {
                transform.translation.x = old_position.x;
            }
        }

        if transform.translation.y.is_nan() {
            if old_position.y.is_nan() {
                transform.translation.y = 0.0;
            } else {
                transform.translation.y = old_position.x;
            }
        }
        
        let mut heading = 0.0;
        if velocity.0.x != 0.0 || velocity.0.y != 0.0 {
            let normal_velocity = velocity.0.normalize();

            if normal_velocity.y < 0.0 {
                heading = -normal_velocity.x.acos();
            } else {
                heading = normal_velocity.x.acos();
            }

            if heading.is_nan() || heading.is_infinite() {
                heading = 0.0;
            }
        }

        transform.rotation = Quat::from_rotation_z(heading);
    }
}

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_to_stage(stage::POST_UPDATE, movement.system());
    }
}
