use crate::gl_facade::{
    shader::{Shader, ShaderKind},
    mesh::{builder::MeshBuilder, Mesh},
    utils,
};
use js_sys::{Float32Array, JsString};
use std::convert::TryInto;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlCanvasElement, WebGlRenderingContext as GL};

mod shaders;
mod vertices;


#[wasm_bindgen]
pub struct Cube {
    mesh: Mesh
}

type Result<T> = std::result::Result<T, JsValue>;

#[wasm_bindgen]
impl Cube {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: JsString) -> Result<Cube> {
        let gl = utils::init_gl_rendering_context(&canvas_id)?;
        
        let vertex_shader = Shader::new(ShaderKind::Vertex, shaders::VERTEX_SHADER);
        let fragment_shader = Shader::new(ShaderKind::Fragment, shaders::FRAGMENT_SHADER);

        let mesh = MeshBuilder::new(gl)
            .compile_shaders(vertex_shader, fragment_shader)?
            .link_program()?
            .finalize();

        let gl_context = mesh.use_gl_context();
        let program = mesh.use_program();

        let positions_data = Float32Array::from(vertices::VERTICES.as_slice()).buffer();

        gl_context
            .create_buffer()
            .and_then(|vbo| {
                gl_context.bind_buffer(GL::ARRAY_BUFFER, Some(&vbo));
                gl_context.buffer_data_with_opt_array_buffer(GL::ARRAY_BUFFER, Some(&positions_data), GL::STATIC_DRAW);
                let position_attr_index = gl_context.get_attrib_location(program, "position") as u32;
                gl_context.vertex_attrib_pointer_with_i32(position_attr_index, 3, GL::FLOAT, false, 0, 0);
                gl_context.enable_vertex_attrib_array(position_attr_index);
                Some(vbo)
            })
            .ok_or_else(|| JsValue::from_str("Failed to create positions VBO."))?;

        let colors_data = Float32Array::from(vertices::V_COLORS.as_slice()).buffer();

        gl_context
            .create_buffer()
            .and_then(|vbo| {
                gl_context.bind_buffer(GL::ARRAY_BUFFER, Some(&vbo));
                gl_context.buffer_data_with_opt_array_buffer(GL::ARRAY_BUFFER, Some(&colors_data), GL::STATIC_DRAW);
                let color_attr_index = gl_context.get_attrib_location(program, "color") as u32;
                gl_context.vertex_attrib_pointer_with_i32(color_attr_index, 3, GL::FLOAT, false, 0, 0);
                gl_context.enable_vertex_attrib_array(color_attr_index);
                Some(vbo)
            })
            .ok_or_else(|| JsValue::from_str("Failed to create colors VBO."))?;

        Ok(Self { mesh })
    }

    pub fn render(&self) -> Result<()> {
        let gl = self.use_gl_context();
       
        let object = gl.canvas().ok_or(JsValue::from_str("Failed to get canvas from WebGlRenderingContext"))?;
        let canvas = object.dyn_into::<HtmlCanvasElement>()?;

        gl.viewport(0, 0, canvas.width().try_into().unwrap(), canvas.height().try_into().unwrap());
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear_depth(1.0);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        self.use_program();

        gl.draw_arrays(GL::LINES, 0, 24);

        Ok(())
    }

    fn use_gl_context(&self) -> &GL {
        self.mesh.use_gl_context()
    }

    fn use_program(&self) {
        self.mesh.use_gl_context().use_program(Some(&self.mesh.use_program()));
    }
}
