use crate::linear_algebra::{Vec2, Vec4};
use crate::utils;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as Gl;

const VERTEX_SHADER_SRC: &'static str = r#"
attribute vec4 v_position;
attribute vec4 v_color;

varying vec4 f_color;

void main() {
    gl_Position = v_position;
    f_color = v_color;
    gl_PointSize = 10.0;
}
"#;

const FRAGMENT_SHADER_SRC: &'static str = r#"
precision mediump float;

varying vec4 f_color;

void main() {
    gl_FragColor = f_color;
}
"#;

const COLORS: [Vec4; 8] = [
    Vec4(0.0, 0.0, 0.0, 1.0), // black
    Vec4(1.0, 0.0, 0.0, 1.0), // red
    Vec4(1.0, 1.0, 0.0, 1.0), // yellow
    Vec4(0.0, 1.0, 0.0, 1.0), // green
    Vec4(0.0, 0.0, 1.0, 1.0), // blue
    Vec4(1.0, 0.0, 1.0, 1.0), // magenta
    Vec4(0.0, 1.0, 1.0, 1.0), // cyan
    Vec4(1.0, 1.0, 1.0, 1.0), // white
];

const MAX_POINTS: i32 = 200;

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
        .ok_or("failed to create v_position buffer")?;
    context.bind_buffer(Gl::ARRAY_BUFFER, Some(&v_position_buffer));
    context.buffer_data_with_i32(Gl::ARRAY_BUFFER, Vec2::SIZE * MAX_POINTS, Gl::STATIC_DRAW);
    let v_position = match context.get_attrib_location(&program, "v_position") {
        -1 => Err("unable to get location for v_position"),
        p => Ok(p as u32),
    }?;
    context.vertex_attrib_pointer_with_i32(v_position, 2, Gl::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(v_position);

    let v_color_buffer = context
        .create_buffer()
        .ok_or("failed to create v_color buffer")?;
    context.bind_buffer(Gl::ARRAY_BUFFER, Some(&v_color_buffer));
    context.buffer_data_with_i32(Gl::ARRAY_BUFFER, Vec4::SIZE * MAX_POINTS, Gl::STATIC_DRAW);
    let v_color = match context.get_attrib_location(&program, "v_color") {
        -1 => Err("unable to get location for v_color"),
        p => Ok(p as u32),
    }?;
    context.vertex_attrib_pointer_with_i32(v_color, 4, Gl::FLOAT, false, 0, 0);
    context.enable_vertex_attrib_array(v_color);

    context.clear_color(0.5, 0.5, 0.5, 1.0);

    let points = Rc::new(RefCell::new(0));

    let canvas = Rc::new(RefCell::new(canvas));
    let context = Rc::new(RefCell::new(context));

    let drawing = Rc::new(RefCell::new(false));
    // MouseUp
    {
        let drawing = drawing.clone();
        let canvas = canvas.clone();
        let stop_drawing = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            *drawing.borrow_mut() = false;
        }) as Box<dyn FnMut(_)>);
        canvas
            .borrow()
            .set_onmouseup(Some(stop_drawing.as_ref().unchecked_ref()));
        stop_drawing.forget();
    }
    // MouseDown
    {
        let drawing = drawing.clone();
        let canvas = canvas.clone();
        let start_drawing = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            *drawing.borrow_mut() = true;
        }) as Box<dyn FnMut(_)>);
        canvas
            .borrow()
            .set_onmousedown(Some(start_drawing.as_ref().unchecked_ref()));
        start_drawing.forget();
    }
    // MouseMove
    {
        let context = context.clone();
        let points = points.clone();
        let drawing = drawing.clone();
        let canvas_ref = canvas.clone();
        let add_point = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if !*drawing.borrow() {
                return;
            }
            let context = context.borrow();
            let mut points = points.borrow_mut();
            let canvas = canvas_ref.borrow();

            context.bind_buffer(Gl::ARRAY_BUFFER, Some(&v_position_buffer));
            let t = Vec2(
                -1.0 + (2.0 * event.offset_x() as f32) / canvas.width() as f32,
                -1.0 + (2.0 * (canvas.height() as f32 - event.offset_y() as f32))
                    / canvas.height() as f32,
            );
            context.buffer_sub_data_with_i32_and_array_buffer_view(
                Gl::ARRAY_BUFFER,
                Vec2::SIZE * (*points % MAX_POINTS),
                &Vec2::flatten(&[t]),
            );

            context.bind_buffer(Gl::ARRAY_BUFFER, Some(&v_color_buffer));
            context.buffer_sub_data_with_i32_and_array_buffer_view(
                Gl::ARRAY_BUFFER,
                Vec4::SIZE * (*points % MAX_POINTS),
                &Vec4::flatten(&[COLORS[(*points as usize % COLORS.len())]]),
            );

            *points += 1;
        }) as Box<dyn FnMut(_)>);
        canvas
            .borrow()
            .set_onmousemove(Some(add_point.as_ref().unchecked_ref()));
        add_point.forget();
    }

    utils::render_loop(move || {
        context.borrow().clear(Gl::COLOR_BUFFER_BIT);
        context
            .borrow()
            .draw_arrays(Gl::POINTS, 0, points.borrow().min(MAX_POINTS));
    });

    Ok(())
}
