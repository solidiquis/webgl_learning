use crate::gl_facade::{
    shader::{Shader, ShaderKind},
    mesh::{Mesh, builder::MeshBuilder},
    utils,
};
use js_sys::{JsString, Float32Array};
use std::convert::TryInto;
use std::mem;
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{HtmlCanvasElement, WebGlRenderingContext as GL, WebGlProgram, WebGlBuffer};

const VERTEX_SHADER: &'static str = r#"
attribute vec4 position;
attribute vec3 color;
varying vec3 fragColor;

void main() {
    gl_Position = position;
    fragColor = color;
}
"#;

const FRAGMENT_SHADER: &'static str = r#"
precision mediump float;
varying vec3 fragColor;

void main() {
    gl_FragColor = vec4(fragColor, 1.0);
}
"#;

const VERTICES: [f32; 15] = [
    // pos             color
    0.0,  0.5,      1.0, 0.0, 0.0,
    -0.5, 0.0,      0.0, 1.0, 0.0,
    0.5,  0.0,      0.0, 0.0, 1.0
];

#[wasm_bindgen]
pub struct Triangle {
    mesh: Mesh,
    vertex_buffer: WebGlBuffer
}

#[wasm_bindgen]
impl Triangle {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: JsString) -> Result<Triangle, JsValue> {
        crate::utils::set_panic_hook();

        let gl_context = utils::init_gl_rendering_context(&canvas_id)?;

        let vertex_shader = Shader::new(ShaderKind::Vertex, VERTEX_SHADER);
        let fragment_shader = Shader::new(ShaderKind::Fragment, FRAGMENT_SHADER);

        let mesh = MeshBuilder::new(gl_context)
            .compile_shaders(vertex_shader, fragment_shader)?
            .link_program()?
            .finalize();

        let gl = mesh.use_gl_context();

        let vertex_buffer = gl.create_buffer().ok_or(JsValue::from_str("Failed to create position buffer."))?;

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));

        let vertex_data = Float32Array::from(VERTICES.as_slice()).buffer();

        gl.buffer_data_with_opt_array_buffer(GL::ARRAY_BUFFER, Some(&vertex_data), GL::STATIC_DRAW);

        Ok(Triangle { mesh, vertex_buffer })
    }

    pub fn render(&self) -> Result<(), JsValue> {
        let gl = self.use_gl_context();

        let object = gl.canvas().ok_or(JsValue::from_str("Failed to get canvas from WebGlRenderingContext"))?;
        let canvas = object.dyn_into::<HtmlCanvasElement>()?;

        gl.viewport(0, 0, canvas.width().try_into().unwrap(), canvas.height().try_into().unwrap());
        gl.clear_color(0.0, 0.0, 0.0, 0.0);
        gl.clear(GL::COLOR_BUFFER_BIT);

        let program = self.use_program();

        gl.use_program(Some(program));

        let vertex_buffer = self.use_vertex_buffer();

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(vertex_buffer));

        let stride = mem::size_of::<f32>() * 5;

        let position_attr_loc = gl.get_attrib_location(program, "position");

        // Position
        gl.vertex_attrib_pointer_with_i32(
            position_attr_loc.try_into().unwrap(),
            2,
            GL::FLOAT,
            false,
            stride.try_into().unwrap(),
            0
        );

        gl.enable_vertex_attrib_array(position_attr_loc.try_into().unwrap());

        let color_attr_loc = gl.get_attrib_location(program, "color");

        let offset = mem::size_of::<f32>() * 2;

        // Color
        gl.vertex_attrib_pointer_with_i32(
            color_attr_loc.try_into().unwrap(),
            3,
            GL::FLOAT,
            false,
            stride.try_into().unwrap(),
            offset.try_into().unwrap()
        );

        gl.enable_vertex_attrib_array(color_attr_loc.try_into().unwrap());

        gl.draw_arrays(GL::TRIANGLES, 0, 3);

        Ok(())
    }

    fn use_vertex_buffer(&self) -> &WebGlBuffer {
        &self.vertex_buffer
    }

    fn use_gl_context(&self) -> &GL {
        self.mesh.use_gl_context()
    }

    fn use_program(&self) -> &WebGlProgram {
        &self.mesh.use_program()
    }
}
