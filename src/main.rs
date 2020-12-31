use bevy::prelude::*;

mod util;
mod plugins;

use plugins::{OnScreenFpsConfig, OnScreenFpsPlugin };
use plugins::examples::SimpleExamplePlugin;

#[bevy_main]
fn main() {
    let mut app = App::build();

    app
        .add_resource(WindowDescriptor {
            title: "Flocking Example".to_string(),
            width: 1024.0,
            height: 800.0,
            vsync: false,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins);
    
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app
        .add_plugin(OnScreenFpsPlugin::new(OnScreenFpsConfig {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(1.0),
                    left: Val::Px(1.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text_style: TextStyle {
                font_size: 22.0,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(SimpleExamplePlugin)
        .run();
}
