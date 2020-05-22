use crate::{linear_algebra::Vec3, utils};
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as Gl;

const SUBDIVISIONS: usize = 3;

const VERTEX_SHADER_SRC: &'static str = r#"
attribute vec4 vPosition;
attribute vec4 vColor;
varying vec4 fColor;

void main() {
    fColor = vColor;
    gl_Position = vPosition;
}
"#;

const FRAGMENT_SHADER_SRC: &'static str = r#"
precision mediump float;

varying vec4 fColor;

void main() {
    gl_FragColor = fColor;
}
"#;

const BASE_COLORS: [Vec3; 4] = [
    Vec3(1.0, 0.0, 0.0),
    Vec3(0.0, 1.0, 0.0),
    Vec3(0.0, 0.0, 1.0),
    Vec3(0.0, 0.0, 0.0),
];

pub fn run(context: Gl) -> Result<(), JsValue> {
    let vert_shader = utils::compile_shader(&context, Gl::VERTEX_SHADER, VERTEX_SHADER_SRC)?;

    let frag_shader = utils::compile_shader(&context, Gl::FRAGMENT_SHADER, FRAGMENT_SHADER_SRC)?;

    let program = utils::link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    let vertices = [
        Vec3(0.0000, 0.0000, -1.0000),
        Vec3(0.0000, 0.9428, 0.3333),
        Vec3(-0.8165, -0.4714, 0.3333),
        Vec3(0.8165, -0.4714, 0.3333),
    ];

    let mut points = vec![];
    let mut colors = vec![];

    divide_tetra(
        &mut points,
        &mut colors,
        &vertices[0],
        &vertices[1],
        &vertices[2],
        &vertices[3],
        SUBDIVISIONS,
    );

    let c_buffer = context.create_buffer().ok_or("failed to create c_buffer")?;
    context.bind_buffer(Gl::ARRAY_BUFFER, Some(&c_buffer));
    context.buffer_data_with_array_buffer_view(
        Gl::ARRAY_BUFFER,
        &Vec3::flatten(&colors),
        Gl::STATIC_DRAW,
    );
    let v_color = match context.get_attrib_location(&program, "vColor") {
        -1 => Err("unable to get location for vColor"),
        vc => Ok(vc as u32),
    }?;
    context.vertex_attrib_pointer_with_i32(v_color, 3, Gl::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(v_color);

    let v_buffer = context.create_buffer().ok_or("failed to create v_buffer")?;
    context.bind_buffer(Gl::ARRAY_BUFFER, Some(&v_buffer));
    context.buffer_data_with_array_buffer_view(
        Gl::ARRAY_BUFFER,
        &Vec3::flatten(&points),
        Gl::STATIC_DRAW,
    );
    let v_position = match context.get_attrib_location(&program, "vPosition") {
        -1 => Err("unable to get location for vPosition"),
        vp => Ok(vp as u32),
    }?;
    context.vertex_attrib_pointer_with_i32(v_position, 3, Gl::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(v_position);

    context.enable(Gl::DEPTH_TEST);
    context.clear_color(1.0, 1.0, 1.0, 1.0);
    context.clear(Gl::COLOR_BUFFER_BIT | Gl::DEPTH_BUFFER_BIT);
    context.draw_arrays(Gl::TRIANGLES, 0, points.len() as i32);

    Ok(())
}

fn triangle(
    points: &mut Vec<Vec3>,
    colors: &mut Vec<Vec3>,
    a: &Vec3,
    b: &Vec3,
    c: &Vec3,
    color_idx: usize,
) {
    points.push(*a);
    colors.push(BASE_COLORS[color_idx]);
    points.push(*b);
    colors.push(BASE_COLORS[color_idx]);
    points.push(*c);
    colors.push(BASE_COLORS[color_idx]);
}

fn tetra(points: &mut Vec<Vec3>, colors: &mut Vec<Vec3>, a: &Vec3, b: &Vec3, c: &Vec3, d: &Vec3) {
    triangle(points, colors, a, c, b, 0);
    triangle(points, colors, a, c, d, 1);
    triangle(points, colors, a, b, d, 2);
    triangle(points, colors, b, c, d, 3);
}

fn divide_tetra(
    points: &mut Vec<Vec3>,
    colors: &mut Vec<Vec3>,
    a: &Vec3,
    b: &Vec3,
    c: &Vec3,
    d: &Vec3,
    count: usize,
) {
    if count == 0 {
        tetra(points, colors, a, b, c, d);
    } else {
        let ab = a.mix(b, 0.5);
        let ac = a.mix(c, 0.5);
        let ad = a.mix(d, 0.5);
        let bc = b.mix(c, 0.5);
        let bd = b.mix(d, 0.5);
        let cd = c.mix(d, 0.5);

        divide_tetra(points, colors, a, &ab, &ac, &ad, count - 1);
        divide_tetra(points, colors, &ab, b, &bc, &bd, count - 1);
        divide_tetra(points, colors, &ac, &bc, c, &cd, count - 1);
        divide_tetra(points, colors, &ad, &bd, &cd, d, count - 1);
    }
}
