use crate::{linear_algebra::Vec3, utils};
use rand::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext;

const NUM_POINTS: usize = 500_000;

const VERTEX_SHADER_SRC: &'static str = r#"
attribute vec4 vPosition;
varying vec4 fColor;

void main() {
    fColor = vec4((1.0 + vPosition.xyz) / 2.0, 1.0);
    gl_Position = vPosition;
    gl_PointSize = 1.0;
}
"#;

const FRAGMENT_SHADER_SRC: &'static str = r#"
precision mediump float;

varying vec4 fColor;

void main() {
    gl_FragColor = fColor;
}
"#;

pub fn run(context: WebGlRenderingContext) -> Result<(), JsValue> {
    let vert_shader = utils::compile_shader(
        &context,
        WebGlRenderingContext::VERTEX_SHADER,
        VERTEX_SHADER_SRC,
    )?;

    let frag_shader = utils::compile_shader(
        &context,
        WebGlRenderingContext::FRAGMENT_SHADER,
        FRAGMENT_SHADER_SRC,
    )?;

    let program = utils::link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    let vertices = [
        Vec3(0.0000, 0.0000, -1.0000),
        Vec3(0.0000, 0.9428, 0.3333),
        Vec3(-0.8165, -0.4714, 0.3333),
        Vec3(0.8165, -0.4714, 0.3333),
    ];

    let mut points = vec![Vec3(0.0, 0.0, 0.0)];

    let mut rng = rand::thread_rng();

    for i in 0..NUM_POINTS {
        let vertex = vertices.choose(&mut rng).unwrap();
        points.push(points[i].mix(vertex, 0.5));
    }

    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    context.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &Vec3::flatten(&points),
        WebGlRenderingContext::STATIC_DRAW,
    );

    let v_position = match context.get_attrib_location(&program, "vPosition") {
        -1 => Err("unable to get location for vPosition"),
        vp => Ok(vp as u32),
    }?;

    context.vertex_attrib_pointer_with_i32(
        v_position,
        3,
        WebGlRenderingContext::FLOAT,
        false,
        0,
        0,
    );
    context.enable_vertex_attrib_array(v_position);

    context.clear_color(1.0, 1.0, 1.0, 1.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    context.draw_arrays(WebGlRenderingContext::POINTS, 0, points.len() as i32);

    Ok(())
}
