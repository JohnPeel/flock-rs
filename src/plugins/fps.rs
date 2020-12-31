use bevy::prelude::*;
use bevy::diagnostic::{ Diagnostics, FrameTimeDiagnosticsPlugin };

#[derive(Clone, Debug)]
pub struct OnScreenFpsConfig {
    pub font: &'static str,
    pub text_style: TextStyle,
    pub style: Style
}

impl Default for OnScreenFpsConfig {
    fn default() -> Self {
        OnScreenFpsConfig {
            font: "fonts/Inconsolata.ttf",
            text_style: Default::default(),
            style: Default::default()
        }
    }
}

#[derive(Clone, Debug)]
struct OnScreenFpsMarker;
#[derive(Clone, Debug)]
pub struct OnScreenFpsPlugin(OnScreenFpsConfig);

fn fps_setup(commands: &mut Commands, config: Res<OnScreenFpsConfig>, asset_server: Res<AssetServer>) {
    commands
        .spawn(CameraUiBundle::default())
        .spawn(TextBundle {
            text: Text {
                value: "FPS".to_string(),
                font: asset_server.load(config.font),
                style: config.text_style.clone(),
                ..Default::default()
            },
            style: config.style.clone(),
            ..Default::default()
        }).with(OnScreenFpsMarker);
}

fn fps_update(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<OnScreenFpsMarker>>) {
    if let Some(Some(fps)) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS).map(|x| x.value()) {
        for mut text in query.iter_mut() {
            text.value = format!("{:<3.3}", fps);
        }
    }
}

impl OnScreenFpsPlugin {
    pub fn new(config: OnScreenFpsConfig) -> Self {
        Self(config)
    }
}

impl Plugin for OnScreenFpsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(self.0.clone())
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_startup_system(fps_setup.system())
            .add_system(fps_update.system());
    }
}
