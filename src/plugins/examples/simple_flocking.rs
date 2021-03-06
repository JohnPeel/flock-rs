use bevy::{prelude::*, window::WindowResized};
use rand::prelude::*;

use crate::plugins::bidimensional::{Flock, FlockMember, FlockMemberParams, FlockingPlugin, MovementPlugin};

struct BackgroundMarker;

pub struct SimpleExamplePlugin;

impl SimpleExamplePlugin {
    fn setup(commands: &mut Commands, window: Res<WindowDescriptor>, mut materials: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>) {
        let mut rng = rand::thread_rng();
        let ship_handle = asset_server.load("sprite/ship.png");

        commands
            .spawn(Camera2dBundle::default())

            // Background
            .spawn(SpriteBundle {
                material: materials.add(ColorMaterial {
                    texture: Some(asset_server.load("sprite/spacefield.png")),
                    color: Color::default()
                }),
                sprite: Sprite::new(Vec2::new(window.width, window.height)),
                ..Default::default()
            }).with(BackgroundMarker)

            // Flock 1
            .spawn((Flock {
                flock_radius: 50.0,
                alignment_strength: 1.0,
                cohesion_strength: 1.0,
                separation_strength: 1.0
            }, ))
            .with_children(|flock| {
                for i in 1..100 {
                    let size = rng.gen_range(12f32..20f32);
                    flock
                        .spawn(SpriteBundle {
                            material: materials.add(ColorMaterial {
                                color: Color::RED,
                                texture: Some(ship_handle.clone())
                            }),
                            visible: Visible {
                                is_transparent: true,
                                ..Default::default()
                            },
                            sprite: Sprite::new(Vec2::new(size, size)),
                            global_transform: GlobalTransform::from_translation(Vec3::new(rng.gen_range(-100f32..=100f32), rng.gen_range(-100f32..=100f32), i as f32)),
                            ..Default::default()
                        }).with_bundle(FlockMember {
                            velocity: Vec2::new(rng.gen_range(-2f32..=2f32), rng.gen_range(-2f32..=2f32)).into(),
                            params: FlockMemberParams {
                                max_speed: 200.0 * (12.0 / size),
                                max_accel: 100.0 * (12.0 / size),
                                safe_radius: size * 5.0,
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                }
            })
            
            // Flock 2
            .spawn((Flock {
                flock_radius: 50.0,
                alignment_strength: 1.0,
                cohesion_strength: 1.0,
                separation_strength: 1.0
            }, ))
            .with_children(|flock| {
                for i in 1..100 {
                    let size = rng.gen_range(12f32..20f32);
                    flock
                        .spawn(SpriteBundle {
                            material: materials.add(ColorMaterial {
                                color: Color::BLUE,
                                texture: Some(ship_handle.clone())
                            }),
                            visible: Visible {
                                is_transparent: true,
                                ..Default::default()
                            },
                            sprite: Sprite::new(Vec2::new(size, size)),
                            global_transform: GlobalTransform::from_translation(Vec3::new(rng.gen_range(-100f32..=100f32), rng.gen_range(-100f32..=100f32), i as f32)),
                            ..Default::default()
                        }).with_bundle(FlockMember {
                            velocity: Vec2::new(rng.gen_range(-2f32..=2f32), rng.gen_range(-2f32..=2f32)).into(),
                            params: FlockMemberParams {
                                max_speed: 200.0 * (12.0 / size),
                                max_accel: 100.0 * (12.0 / size),
                                safe_radius: size * 5.0,
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                }
            });
    }

    fn resized(mut reader: Local<EventReader<WindowResized>>, resize_event: Res<Events<WindowResized>>, mut query: Query<&mut Sprite, With<BackgroundMarker>>) {
        for event in reader.iter(&resize_event) {
            for mut sprite in query.iter_mut() {
                sprite.size = Vec2::new(event.width, event.height);
            }
        }
    }
}

impl Plugin for SimpleExamplePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(MovementPlugin)
            .add_plugin(FlockingPlugin::with_wrapping())
            .add_startup_system(Self::setup.system())
            .add_system(Self::resized.system());
    }
}
