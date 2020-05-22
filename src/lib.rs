mod linear_algebra;
mod programs;
mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn get_context() -> Result<WebGlRenderingContext, JsValue> {
    utils::set_panic_hook();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.query_selector("canvas").unwrap().unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    context.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);

    Ok(context)
}

#[wasm_bindgen]
pub fn sierpinski() -> Result<(), JsValue> {
    programs::sierpinski::run(get_context()?)
}

#[wasm_bindgen]
pub fn sierpinski_3d_points() -> Result<(), JsValue> {
    programs::sierpinski_3d_points::run(get_context()?)
}
#[wasm_bindgen]
pub fn sierpinski_points() -> Result<(), JsValue> {
    programs::sierpinski_points::run(get_context()?)
}

#[wasm_bindgen]
pub fn square() -> Result<(), JsValue> {
    programs::square::run(get_context()?)
}

#[wasm_bindgen]
pub fn triangle() -> Result<(), JsValue> {
    programs::triangle::run(get_context()?)
}
