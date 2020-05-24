use crate::linear_algebra::Vec2;
use crate::utils;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
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

fn get_element(id: &str) -> Option<web_sys::Element> {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id(id))
}

pub fn run(context: Gl) -> Result<(), JsValue> {
    let toggle_btn = get_element("direction-toggle").ok_or("direction-toggle not found")?;
    let toggle_btn = toggle_btn.dyn_into::<web_sys::HtmlButtonElement>()?;

    let speed_slider = get_element("speed-slider").ok_or("speed-slider not found")?;
    let speed_slider = speed_slider.dyn_into::<web_sys::HtmlInputElement>()?;

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

    let direction = Rc::new(RefCell::new(false));
    {
        let direction = direction.clone();
        let toggle_direction = Closure::wrap(Box::new(move || {
            *direction.borrow_mut() ^= true;
        }) as Box<dyn FnMut()>);
        toggle_btn.set_onclick(Some(toggle_direction.as_ref().unchecked_ref()));
        toggle_direction.forget();
    }

    let delay = Rc::new(RefCell::new(50));
    {
        let speed_slider = Rc::new(RefCell::new(speed_slider));
        let speed_slider_ref = speed_slider.clone();
        let delay = delay.clone();
        let change_delay = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            *delay.borrow_mut() = 100 - speed_slider_ref.borrow().value().parse::<i32>().unwrap();
        }) as Box<dyn FnMut(_)>);
        speed_slider
            .borrow()
            .set_oninput(Some(change_delay.as_ref().unchecked_ref()));
        change_delay.forget();
    }

    let mut theta = 0.0f32;

    context.clear_color(1.0, 1.0, 1.0, 1.0);
    utils::render_loop_with_delay(
        move || {
            context.clear(Gl::COLOR_BUFFER_BIT);
            theta += if *direction.borrow() { 0.1 } else { -0.1 };
            context.uniform1f(Some(&theta_loc), theta);
            context.draw_arrays(Gl::TRIANGLE_STRIP, 0, 4);
        },
        delay,
    );

    Ok(())
}
