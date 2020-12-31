use bevy::prelude::*;
use rand::prelude::*;

use crate::plugins::bidimensional::{Flock, FlockMember, FlockMemberParams, FlockingPlugin, VelocityPlugin};

pub struct SimpleExamplePlugin;

impl SimpleExamplePlugin {
    fn setup(commands: &mut Commands, window: Res<WindowDescriptor>, mut materials: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>) {
        commands
            .spawn(Camera2dBundle::default())

            // Background
            .spawn(SpriteBundle {
                material: materials.add(ColorMaterial {
                    texture: Some(asset_server.load("sprite/spacefield.png")),
                    color: Color::default()
                }),
                sprite: Sprite {
                    size: Vec2::new(window.width, window.height),
                    ..Default::default()
                },
                ..Default::default()
            })

            // Flock 1
            .spawn((Flock {
                flock_radius: 50.0,
                alignment_strength: 1.0,
                cohesion_strength: 1.0,
                separation_strength: 1.0
            }, ))
            .with_children(|flock| {
                let ship_handle = asset_server.load("sprite/ship.png");
                let mut rng = rand::thread_rng();

                for i in 1..100 {
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
                            sprite: Sprite::new(Vec2::new(16.0, 16.0)),
                            global_transform: GlobalTransform::from_translation(Vec3::new(rng.gen_range(-100f32..=100f32), rng.gen_range(-100f32..=100f32), i as f32)),
                            ..Default::default()
                        }).with_bundle(FlockMember {
                            velocity: Vec2::new(0.0, -0.5).into(),
                            params: FlockMemberParams {
                                safe_radius: 50.0,
                                max_speed: 250.0
                            },
                            ..Default::default()
                        });
                }
            });
    }
}

impl Plugin for SimpleExamplePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_plugin(VelocityPlugin)
            .add_plugin(FlockingPlugin::with_wrapping())
            .add_startup_system(Self::setup.system());
    }
}
