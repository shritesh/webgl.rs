use crate::linear_algebra::Vec2;
use crate::utils;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as Gl;

const VERTEX_SHADER_SRC: &'static str = r#"
attribute vec4 v_position;
uniform float theta;

void main() {
    float x = -sin(theta) * v_position.x + cos(theta) * v_position.y;
    float y = sin(theta) * v_position.y + cos(theta) * v_position.x;
    gl_Position = vec4(x, y, 0.0, 1.0);
}
"#;

const FRAGMENT_SHADER_SRC: &'static str = r#"
precision mediump float;

void main() {
    gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
}
"#;

pub fn run(context: Gl) -> Result<(), JsValue> {
    let vertex_shader = utils::compile_shader(&context, Gl::VERTEX_SHADER, VERTEX_SHADER_SRC)?;
    let fragment_shader =
        utils::compile_shader(&context, Gl::FRAGMENT_SHADER, FRAGMENT_SHADER_SRC)?;
    let program = utils::link_program(&context, &vertex_shader, &fragment_shader)?;
    context.use_program(Some(&program));

    let points = [
        Vec2(0.0, 1.0),
        Vec2(-1.0, 0.0),
        Vec2(1.0, 0.0),
        Vec2(0.0, -1.0),
    ];

    let v_position_buffer = context
        .create_buffer()
        .ok_or("failed to create v_position buffer")?;
    context.bind_buffer(Gl::ARRAY_BUFFER, Some(&v_position_buffer));
    context.buffer_data_with_array_buffer_view(
        Gl::ARRAY_BUFFER,
        &Vec2::flatten(&points),
        Gl::STATIC_DRAW,
    );
    let v_position = match context.get_attrib_location(&program, "v_position") {
        -1 => Err("unable to get location for v_position"),
        p => Ok(p as u32),
    }?;
    context.vertex_attrib_pointer_with_i32(v_position, 2, Gl::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(v_position);

    let theta_loc = context
        .get_uniform_location(&program, "theta")
        .ok_or("failed to get location for theta")?;

    let mut theta = 0.0f32;

    context.clear_color(1.0, 1.0, 1.0, 1.0);

    utils::render_loop(move || {
        context.clear(Gl::COLOR_BUFFER_BIT);
        theta += 0.1;
        context.uniform1f(Some(&theta_loc), theta);
        context.draw_arrays(Gl::TRIANGLE_STRIP, 0, 4);
    });

    Ok(())
}
