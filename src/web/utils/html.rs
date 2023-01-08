use web_sys::{Document, HtmlCanvasElement, Window};

use wasm_bindgen::prelude::*;

pub fn window() -> Window {
    web_sys::window().expect("cannot get winow")
}

pub fn document() -> Document {
    window().document().expect("cannot get document")
}

pub fn get_canvas_context(
    canvas: &web_sys::HtmlCanvasElement,
) -> web_sys::CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

pub fn create_canvas() -> HtmlCanvasElement {
    document()
        .create_element("canvas")
        .expect("cannot create canvas")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap()
}
