use crate::{linear_algebra::Vec2, utils};
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as Gl;

const SUBDIVISIONS: usize = 4;

pub fn run(context: Gl) -> Result<(), JsValue> {
    let vert_shader = utils::compile_shader(
        &context,
        Gl::VERTEX_SHADER,
        r#"
        attribute vec4 vPosition;

        void main() {
            float x = vPosition.x;
            float y = vPosition.y;
            float theta = sqrt(x * x + y * y);
            gl_Position = vec4(
                x * cos(theta) - y * sin(theta),
                x * sin(theta) + y * cos(theta),
                vPosition.zw
            );
        }
        "#,
    )?;

    let frag_shader = utils::compile_shader(
        &context,
        Gl::FRAGMENT_SHADER,
        r#"
        precision mediump float;

        void main() {
            gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#,
    )?;

    let program = utils::link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    let mut points = vec![];
    tesselate(
        &mut points,
        &Vec2(0.0, 0.75),
        &Vec2(-0.75, -0.75),
        &Vec2(0.75, -0.75),
        SUBDIVISIONS,
    );

    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(Gl::ARRAY_BUFFER, Some(&buffer));

    context.buffer_data_with_array_buffer_view(
        Gl::ARRAY_BUFFER,
        &Vec2::flatten(&points),
        Gl::STATIC_DRAW,
    );

    let v_position = match context.get_attrib_location(&program, "vPosition") {
        -1 => Err("unable to get location for vPosition"),
        vp => Ok(vp as u32),
    }?;

    context.vertex_attrib_pointer_with_i32(v_position, 2, Gl::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(v_position);

    context.clear_color(1.0, 1.0, 1.0, 1.0);
    context.clear(Gl::COLOR_BUFFER_BIT);
    context.draw_arrays(Gl::TRIANGLES, 0, points.len() as i32);

    Ok(())
}

fn tesselate(points: &mut Vec<Vec2>, a: &Vec2, b: &Vec2, c: &Vec2, count: usize) {
    if count == 0 {
        points.push(*a);
        points.push(*b);
        points.push(*c);
    } else {
        let ab = a.mix(b, 0.5);
        let ac = a.mix(c, 0.5);
        let bc = b.mix(c, 0.5);

        tesselate(points, &ab, &ac, &bc, count - 1);
        tesselate(points, a, &ab, &ac, count - 1);
        tesselate(points, b, &bc, &ab, count - 1);
        tesselate(points, c, &ac, &bc, count - 1);
    }
}
