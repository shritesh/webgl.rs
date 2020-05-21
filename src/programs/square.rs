use crate::linarg::Vec2;
use crate::utils;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext;

pub fn run(context: WebGlRenderingContext) -> Result<(), JsValue> {
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
            gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "#,
    )?;

    let program = utils::link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    let vertices = [
        Vec2(-0.5, -0.5),
        Vec2(-0.5, 0.5),
        Vec2(0.5, 0.5),
        Vec2(0.5, -0.5),
    ];

    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    context.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &Vec2::flatten(&vertices),
        WebGlRenderingContext::STATIC_DRAW,
    );

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

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    context.draw_arrays(
        WebGlRenderingContext::TRIANGLE_FAN,
        0,
        vertices.len() as i32,
    );

    Ok(())
}
