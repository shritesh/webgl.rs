use crate::{linear_algebra::Vec2, utils};
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext;

const SUBDIVISIONS: usize = 5;

pub fn run(context: WebGlRenderingContext) -> Result<(), JsValue> {
    let vert_shader = utils::compile_shader(
        &context,
        WebGlRenderingContext::VERTEX_SHADER,
        r#"
        attribute vec4 vPosition;

        void main() {
            gl_PointSize = 1.0;
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

    let vertices = [Vec2(-1.0, -1.0), Vec2(0.0, 1.0), Vec2(1.0, -1.0)];

    let mut points = vec![];

    divide_triangle(
        &mut points,
        &vertices[0],
        &vertices[1],
        &vertices[2],
        SUBDIVISIONS,
    );

    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    context.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &Vec2::flatten(&points),
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

    context.clear_color(1.0, 1.0, 1.0, 1.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, points.len() as i32);

    Ok(())
}

fn divide_triangle(points: &mut Vec<Vec2>, a: &Vec2, b: &Vec2, c: &Vec2, count: usize) {
    if count == 0 {
        points.push(*a);
        points.push(*b);
        points.push(*c);
    } else {
        let ab = a.mix(b, 0.5);
        let ac = a.mix(c, 0.5);
        let bc = b.mix(c, 0.5);

        divide_triangle(points, a, &ab, &ac, count - 1);
        divide_triangle(points, b, &bc, &ab, count - 1);
        divide_triangle(points, c, &ac, &bc, count - 1);
    }
}
