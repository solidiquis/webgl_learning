use crate::gl_facade::{
    shader::{Shader, ShaderKind},
    mesh::{builder::MeshBuilder, Mesh},
    utils,
};
use js_sys::{Float32Array, JsString};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::{WebGlBuffer, WebGlRenderingContext as GL};

mod shaders;
mod vertices;


#[wasm_bindgen]
pub struct Cube {
    mesh: Mesh,
    positions_vbo: WebGlBuffer,
    colors_vbo: WebGlBuffer,
}

type Result<T> = std::result::Result<T, JsValue>;

#[wasm_bindgen]
impl Cube {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: JsString) -> Result<Self> {
        let gl_context = utils::init_gl_rendering_context(&canvas_id)?;
        
        let vertex_shader = Shader::new(ShaderKind::Vertex, shaders::VERTEX_SHADER);
        let fragment_shader = Shader::new(ShaderKind::Fragment, shaders::FRAGMENT_SHADER);

        let mesh = MeshBuilder::new(gl_context)
            .compile_shaders(vertex_shader, fragment_shader)?
            .link_program()?
            .finalize();

        let gl = mesh.use_gl_context();

        let positions_data = Float32Array::from(vertices::VERTICES.as_slice()).buffer();

        let positions_vbo = gl
            .create_buffer()
            .and_then(|vbo| {
                gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vbo));
                gl.buffer_data_with_opt_array_buffer(GL::ARRAY_BUFFER, Some(&positions_data), GL::STATIC_DRAW);
                Some(vbo)
            })
            .ok_or_else(|| JsValue::from_str("Failed to create positions VBO."))?;

        let colors_data = Float32Array::from(vertices::V_COLORS.as_slice()).buffer();

        let colors_vbo = gl
            .create_buffer()
            .and_then(|vbo| {
                gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vbo));
                gl.buffer_data_with_opt_array_buffer(GL::ARRAY_BUFFER, Some(&colors_data), GL::STATIC_DRAW);
                Some(vbo)
            })
            .ok_or_else(|| JsValue::from_str("Failed to create colors VBO."))?;

        Ok(Self { mesh, positions_vbo, colors_vbo })
    }
}
