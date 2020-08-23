
use bevy::{prelude::*, diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin}, ecs::Mut};

pub struct OnscreenFpsPlugin;

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
                    value: "FPS".to_string(),
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
