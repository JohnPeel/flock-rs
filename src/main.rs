
#[cfg(feature = "web")]
extern crate wee_alloc;
#[cfg(feature = "web")]
extern crate console_error_panic_hook;

#[cfg(feature = "web")]
use std::panic;

#[cfg(feature = "web")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use bevy::prelude::*;

mod util;
mod plugins;

use plugins::*;
use plugins::examples::SimpleExamplePlugin;

#[bevy_main]
fn main() {
    #[cfg(feature = "web")]
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let (mut width, mut height) = (1024.0, 800.0);
    let mut app = App::build();

    #[cfg(feature = "web")]
    {
        let window: web_sys::Window = web_sys::window().unwrap();
        let document_element = window.document().unwrap().document_element().unwrap();
        width = document_element.client_width() as f32;
        height = document_element.client_height() as f32;
    }

    app
        .add_resource(WindowDescriptor {
            title: "Flocking Example".to_string(),
            width,
            height,
            vsync: false,
            resizable: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins);
    
    #[cfg(feature = "web")]
    app
        .add_plugin(bevy_webgl2::WebGL2Plugin)
        .add_plugin(viewport_resize::ViewportResizedPlugin);

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
