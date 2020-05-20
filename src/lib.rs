mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
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
    context.clear_color(1.0, 1.0, 1.0, 1.0);

    let vert_shader = utils::compile_shader(
        &context,
        WebGlRenderingContext::VERTEX_SHADER,
        r#"
        attribute vec4 vPosition;

        void main() {
            gl_Position = vPosition;
        }
        "#,
    )?;

    let frag_shader = utils::compile_shader(
        &context,
        WebGlRenderingContext::FRAGMENT_SHADER,
        r#"
        precision mediump float;

        void main() {
            gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#,
    )?;

    let program = utils::link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    let vertices: [f32; 6] = [0.0, 1.0, -1.0, -1.0, 1.0, -1.0];

    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    unsafe {
        let vert_array = js_sys::Float32Array::view(&vertices);
        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    let v_position = match context.get_attrib_location(&program, "vPosition") {
        -1 => Err("unable to get location for vPosition"),
        vp => Ok(vp as u32),
    }?;

    context.vertex_attrib_pointer_with_i32(
        v_position,
        2,
        WebGlRenderingContext::FLOAT,
        false,
        0,
        0,
    );
    context.enable_vertex_attrib_array(v_position);

    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    context.draw_arrays(
        WebGlRenderingContext::TRIANGLES,
        0,
        (vertices.len() / 2) as i32,
    );

    Ok(())
}
