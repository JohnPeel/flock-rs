use bevy::{diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin}, prelude::*, ecs::Mut};
use rand::Rng;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
struct Boid {
    id: usize,
    flock_id: usize,
    velocity: Vec3,
    max_speed: f32,
    safe_radius: f32
}

#[derive(Debug, Clone)]
struct FlockingPlugin {
    flocks: Vec<FlockParameters>
}

#[derive(Debug, Copy, Clone)]
struct FlockParameters {
    id: usize,
    boid_count: usize,
    color: Color,
    flock_radius: f32,
    alignment_strength: f32,
    cohesion_strength: f32,
    separation_strength: f32
}

struct FlockAverages {
    average_position: Vec3,
    average_forward: Vec3,
    boids: Vec<(Boid, Vec3)>
}

struct OnscreenFpsPlugin;

impl Plugin for OnscreenFpsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_startup_system(Self::setup_system.system())
            .add_system(Self::fps_update.system());
    }
}

impl OnscreenFpsPlugin {
    fn fps_update(diagnostics: Res<Diagnostics>, mut text: Mut<Text>) {
        if let Some((Some(fps), Some(average))) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS).map(|x| (x.value(), x.average())) {
            text.value = format!("{:<3.3} ({:<3.3})", fps, average);
        }
    }

    fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands
            .spawn(UiCameraComponents::default())
            .spawn(TextComponents {
                text: Text {
                    value: "Hello from Bevy UI!".to_string(),
                    font: asset_server.load("assets/fonts/Inconsolata.ttf").unwrap(),
                    style: TextStyle {
                        font_size: 25.0,
                        color: Color::WHITE,
                    },
                },
                transform: Transform::new(Mat4::from_translation(Vec3::new(0.0, 0.0, 2.0))),
                ..Default::default()
            });
    }
}

impl Plugin for FlockingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(self.flocks.clone())
            .add_startup_system(Self::setup_system.system())
            .add_system(Self::update_flocks.system())
            .add_stage_after("update", "movement")
            .add_system_to_stage("movement", Self::movement_system.system());
    }
}

impl FlockingPlugin {
    fn calculate_alignment(max_speed: f32, average_forward: Vec3) -> Vec3 {
        let mut alignment = average_forward / max_speed;
    
        if alignment.length_squared() > 1.0 {
            alignment = alignment.normalize();
        }
    
        alignment
    }
    
    fn calculate_cohesion(position: Vec3, average_position: Vec3, flock_radius: f32) -> Vec3 {
        let mut cohesion = average_position - position;
    
        if cohesion.length_squared() < flock_radius * flock_radius {
            cohesion /= flock_radius;
        } else {
            cohesion = cohesion.normalize();
        }
    
        cohesion
    }
    
    fn calculate_separation(boid: Boid, position: Vec3, boids: &[(Boid, Vec3)], width: f32, height: f32) -> Vec3 {
        let mut separation = Vec3::zero();
        
        for (other, other_pos) in boids.iter() {
            if boid.id != other.id {
                let difference = position - Self::normalize_pos_to(*other_pos, position, width, height);
                let distance_squared = difference.length_squared();
                let minimum_distance = boid.safe_radius + other.safe_radius;
    
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
    
    fn normalize_pos_to(position: Vec3, center: Vec3, width: f32, height: f32) -> Vec3 {
        let mut new_position = position;
        if position.x() < center.x() - width {
            new_position.set_x(position.x() + 2.0 * width);
        } else if position.x() > center.x() + width {
            new_position.set_x(position.x() - 2.0 * width);
        }
    
        if position.y() < center.y() - height {
            new_position.set_y(position.y() + 2.0 * height);
        } else if position.y() > center.y() + height {
            new_position.set_y(position.y() - 2.0 * height);
        }
    
        new_position
    }
    
    fn calculate_averages(params: Vec<FlockParameters>, query: &mut Query<(&mut Boid, &Translation)>) -> Vec<FlockAverages> {
        let mut result = Vec::<FlockAverages>::with_capacity(params.len());
    
        for flock in params.iter() {
            result.insert(flock.id, FlockAverages {
                average_position: Vec3::zero(),
                average_forward: Vec3::zero(),
                boids: Vec::with_capacity(flock.boid_count)
            });
        }
    
        for (boid, position) in &mut query.iter() {
            result[boid.flock_id].average_position += position.0;
            result[boid.flock_id].average_forward += boid.velocity;
            result[boid.flock_id].boids.push((*boid, position.0.to_owned()));
        }
    
        for flock in params.iter() {
            result[flock.id].average_position /= flock.boid_count as f32;
            result[flock.id].average_forward /= flock.boid_count as f32;
        }
    
        result
    }

    fn setup_system(mut commands: Commands, window: Res<WindowDescriptor>, params: Res<Vec<FlockParameters>>, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
        commands
            .spawn(Camera2dComponents::default());
    
        commands
            .spawn(SpriteComponents {
                material: materials.add(ColorMaterial {
                    texture: Some(asset_server.load("assets/sprite/spacefield.png").unwrap()),
                    color: Color::default()
                }),
                translation: Translation::new(0.0, 0.0, 0.0),
                sprite: Sprite {
                    size: Vec2::new(window.width as f32, window.height as f32)
                },
                ..Default::default()
            });
    
        let ship_handle = asset_server.load("assets/sprite/ship.png").unwrap();
        let mut rng = rand::thread_rng();
        for flock in params.iter() {
            let material = materials.add(ColorMaterial {
                texture: Some(ship_handle),
                color: flock.color
            });
    
            for id in 0..flock.boid_count {
                commands
                    .spawn(SpriteComponents {
                        material,
                        translation: Translation::new(
                            rng.gen_range(-300.0, 300.0),
                            rng.gen_range(-300.0, 300.0),
                            1.0
                        ),
                        sprite: Sprite {
                            size: Vec2::new(8.0, 8.0)
                        },
                        ..Default::default()
                    })
                    .with(Boid { id, flock_id: flock.id, velocity: Vec3::zero(), max_speed: 200.0, safe_radius: 50.0 });
            }
        }
    }

    fn update_flocks(time: Res<Time>, window: Res<WindowDescriptor>, params: Res<Vec<FlockParameters>>, mut query: Query<(&mut Boid, &Translation)>) {
        let averages = Self::calculate_averages(params.clone(),  &mut query);
        
        let width = (window.width / 2) as f32;
        let height = (window.height / 2) as f32;
        for (mut boid, position) in &mut query.iter() {
            let alignment = Self::calculate_alignment(boid.max_speed, averages[boid.flock_id].average_forward);
            let cohesion = Self::calculate_cohesion(position.0, averages[boid.flock_id].average_position, params[boid.flock_id].flock_radius);
            let separation = Self::calculate_separation(*boid, position.0, &averages[boid.flock_id].boids, width, height);
    
            let new_velocity = boid.velocity + (
                alignment * params[boid.flock_id].alignment_strength +
                cohesion * params[boid.flock_id].cohesion_strength +
                separation * params[boid.flock_id].separation_strength
            ) * boid.max_speed * time.delta_seconds;
            boid.velocity = new_velocity;
    
            if boid.velocity.length_squared() > boid.max_speed * boid.max_speed {
                let new_velocity = boid.velocity.normalize() * boid.max_speed;
                boid.velocity = new_velocity;
            }
        }
    }

    fn movement_system(time: Res<Time>, window: Res<WindowDescriptor>, mut query: Query<(&Boid, &mut Rotation, &mut Translation)>) {
        for (boid, mut heading, mut position) in &mut query.iter() {
            let old_position = position.0;
            position.0 += boid.velocity * time.delta_seconds;
            let new_position = position.0;
    
            if new_position.x().is_nan() {
                if old_position.x().is_nan() {
                    position.0.set_x(0.0);
                } else {
                    position.0.set_x(old_position.x());
                }
            }
    
            if new_position.y().is_nan() {
                if old_position.y().is_nan() {
                    position.0.set_y(0.0);
                } else {
                    position.0.set_y(old_position.y());
                }
            }
    
            position.0 = Self::normalize_pos_to(position.0, Vec3::zero(), (window.width / 2) as f32, (window.height / 2) as f32);
            position.0.set_z(1.0);
    
            let mut new_heading = 0.0;
            if boid.velocity.x() != 0.0 || boid.velocity.y() != 0.0 {
                let normalized_velocity = boid.velocity.normalize();
    
                if normalized_velocity.y() < 0.0 {
                    new_heading = -normalized_velocity.x().acos();
                } else {
                    new_heading = normalized_velocity.x().acos();
                }
    
                if new_heading.is_nan() || new_heading.is_infinite() {
                    new_heading = 0.0;
                }
            }
            *heading = Rotation::from_rotation_z(new_heading);
        }
    }
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Flocking Example".to_string(),
            width: 1024,
            height: 800,
            vsync: true
        })
        .add_default_plugins()
        .add_plugin(OnscreenFpsPlugin)
        .add_plugin(FlockingPlugin {
            flocks: vec![
                FlockParameters {
                    id: 0,
                    boid_count: 10,
                    color: Color::rgb(0.8, 0.1, 0.1),
                    flock_radius: 50.0,
                    alignment_strength: 1.0,
                    cohesion_strength: 1.0,
                    separation_strength: 1.0
                },
                FlockParameters {
                    id: 1,
                    boid_count: 5,
                    color: Color::rgb(0.1, 0.8, 0.1),
                    flock_radius: 50.0,
                    alignment_strength: 1.0,
                    cohesion_strength: 1.0,
                    separation_strength: 1.0
                },
                FlockParameters {
                    id: 2,
                    boid_count: 2,
                    color: Color::rgb(0.1, 0.1, 0.8),
                    flock_radius: 50.0,
                    alignment_strength: 1.0,
                    cohesion_strength: 1.0,
                    separation_strength: 1.0
                }
            ]
        })
        .run();
}
