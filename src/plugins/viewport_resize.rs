use bevy::prelude::*;
use wasm_bindgen::prelude::*;

use futures::channel::mpsc;

#[derive(Debug)]
struct ViewportResized{
    width: f32,
    height: f32
}

#[derive(Debug)]
struct ViewportState {
    receiver: mpsc::UnboundedReceiver<ViewportResized>
}

pub struct ViewportResizedPlugin;

impl From<(f32, f32)> for ViewportResized {
    fn from((width, height): (f32, f32)) -> Self {
        ViewportResized { width, height }
    }
}

fn get_viewport_size() -> (f32, f32) {
    let window = web_sys::window().expect("could not get window");
    let document_element = window
                .document()
                .expect("could not get document")
                .document_element()
                .expect("could not get document element");

    (document_element.client_width() as f32, document_element.client_height() as f32)
}

fn resized_event(mut windows: ResMut<Windows>, mut state: ResMut<ViewportState>) {
    if let Ok(Some(event)) = state.receiver.try_next() {
        if let Some(window) = windows.get_primary_mut() {
            window.set_resolution(event.width, event.height);
        }
    }
}

impl Plugin for ViewportResizedPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let (sender, receiver) = mpsc::unbounded();

        let window = web_sys::window().expect("could not get window");
        gloo_events::EventListener::new(&window, "resize", move |_event| {
            sender.unbounded_send(get_viewport_size().into()).unwrap_throw();
        }).forget();

        app
            .add_resource(ViewportState { receiver })
            .add_system(resized_event.system());
    }
}
