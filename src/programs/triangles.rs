use crate::{
    linear_algebra::{Vec2, Vec3},
    utils,
};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as Gl;

const VERTEX_SHADER_SRC: &'static str = r#"
attribute vec4 v_Position;
attribute vec3 v_Color;

varying vec4 f_Color;

void main() {
    gl_Position = v_Position;
    f_Color = vec4(v_Color, 1.0);
}
"#;

const FRAGMENT_SHADER_SRC: &'static str = r#"
precision mediump float;

varying vec4 f_Color;

void main() {
    gl_FragColor = f_Color;
}
"#;

const MAX_VERTICES: i32 = 200;

const COLORS: [Vec3; 8] = [
    Vec3(0.0, 0.0, 0.0), // black
    Vec3(1.0, 0.0, 0.0), // red
    Vec3(1.0, 1.0, 0.0), // yellow
    Vec3(0.0, 1.0, 0.0), // green
    Vec3(0.0, 0.0, 1.0), // blue
    Vec3(1.0, 0.0, 1.0), // magenta
    Vec3(0.0, 1.0, 1.0), // cyan
    Vec3(1.0, 1.0, 1.0), // white
];

pub fn run(context: Gl) -> Result<(), JsValue> {
    let canvas = context.canvas().unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let vertex_shader = utils::compile_shader(&context, Gl::VERTEX_SHADER, VERTEX_SHADER_SRC)?;
    let fragment_shader =
        utils::compile_shader(&context, Gl::FRAGMENT_SHADER, FRAGMENT_SHADER_SRC)?;
    let program = utils::link_program(&context, &vertex_shader, &fragment_shader)?;
    context.use_program(Some(&program));

    let v_position_buffer = context
        .create_buffer()
        .ok_or("failed to create v_Position buffer")?;
    context.bind_buffer(Gl::ARRAY_BUFFER, Some(&v_position_buffer));
    context.buffer_data_with_i32(Gl::ARRAY_BUFFER, Vec2::SIZE * MAX_VERTICES, Gl::STATIC_DRAW);
    let v_position = match context.get_attrib_location(&program, "v_Position") {
        -1 => Err("unable to get location for v_Position"),
        p => Ok(p as u32),
    }?;
    context.vertex_attrib_pointer_with_i32(v_position, 2, Gl::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(v_position);

    let v_color_buffer = context
        .create_buffer()
        .ok_or("failed to create v_Color buffer")?;
    context.bind_buffer(Gl::ARRAY_BUFFER, Some(&v_color_buffer));
    context.buffer_data_with_i32(Gl::ARRAY_BUFFER, Vec3::SIZE * MAX_VERTICES, Gl::STATIC_DRAW);
    let v_color = match context.get_attrib_location(&program, "v_Color") {
        -1 => Err("unable to get location for v_Color"),
        p => Ok(p as u32),
    }?;
    context.vertex_attrib_pointer_with_i32(v_color, 3, Gl::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(v_color);

    context.clear_color(0.5, 0.5, 0.5, 1.0);

    let vertices = Rc::new(RefCell::new(0));
    let context = Rc::new(context);
    let canvas = Rc::new(canvas);
    {
        let canvas_ref = canvas.clone();
        let context = context.clone();
        let vertices = vertices.clone();
        utils::add_event_listener(&canvas, "mousedown", move |event| {
            let event = event.dyn_into::<web_sys::MouseEvent>().unwrap();
            let mut vertices = vertices.borrow_mut();

            if *vertices == MAX_VERTICES {
                return;
            }

            context.bind_buffer(Gl::ARRAY_BUFFER, Some(&v_position_buffer));
            let t = Vec2(
                -1.0 + (2.0 * event.offset_x() as f32) / canvas_ref.width() as f32,
                -1.0 + (2.0 * (canvas_ref.height() as f32 - event.offset_y() as f32))
                    / canvas_ref.height() as f32,
            );
            context.buffer_sub_data_with_i32_and_array_buffer_view(
                Gl::ARRAY_BUFFER,
                Vec2::SIZE * *vertices,
                &Vec2::flatten(&[t]),
            );

            context.bind_buffer(Gl::ARRAY_BUFFER, Some(&v_color_buffer));
            context.buffer_sub_data_with_i32_and_array_buffer_view(
                Gl::ARRAY_BUFFER,
                Vec3::SIZE * *vertices,
                &Vec3::flatten(&[COLORS[(*vertices as usize % COLORS.len())]]),
            );

            *vertices += 1;
        });
    }

    utils::render_loop(move || {
        context.clear(Gl::COLOR_BUFFER_BIT);
        context.draw_arrays(Gl::TRIANGLE_STRIP, 0, vertices.borrow().min(MAX_VERTICES));
    });
    Ok(())
}
