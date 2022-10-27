use crate::gl_facade::{
    shader::{Shader, ShaderKind},
    mesh::{builder::MeshBuilder, Mesh},
    utils,
};
use js_sys::{Float32Array, JsString, Number, Uint16Array};
use nalgebra_glm as glm;
use std::convert::TryInto;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlCanvasElement, WebGlRenderingContext as GL, WebGlProgram};

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

        let gl_context = mesh.get_gl_context();
        let program = mesh.get_program();

        gl_context.enable(GL::DEPTH_TEST);
        gl_context.depth_func(GL::LEQUAL);

        /*
         * Positions VBO
         * */
        let position_attr_index = gl_context.get_attrib_location(program, "position") as u32;
        let positions_data = Float32Array::from(vertices::VERTICES.as_slice()).buffer();
        let positions_vbo = gl_context
            .create_buffer()
            .ok_or_else(|| JsValue::from_str("Failed to create positions VBO."))?;
        gl_context.bind_buffer(GL::ARRAY_BUFFER, Some(&positions_vbo));
        gl_context.buffer_data_with_opt_array_buffer(GL::ARRAY_BUFFER, Some(&positions_data), GL::STATIC_DRAW);
        gl_context.vertex_attrib_pointer_with_i32(position_attr_index, 3, GL::FLOAT, false, 0, 0);
        gl_context.enable_vertex_attrib_array(position_attr_index);
            
        /*
         * IBO
         * */
        let index_data = Uint16Array::from(vertices::INDICES.as_slice()).buffer();
        let ibo = gl_context
            .create_buffer()
            .ok_or_else(|| JsValue::from_str("Failed to create IBO."))?;
        gl_context.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&ibo));
        gl_context.buffer_data_with_opt_array_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&index_data), GL::STATIC_DRAW);
            
        /*
         * Colors VBO
         * */
        let color_attr_index = gl_context.get_attrib_location(program, "color") as u32;
        let colors_data = Float32Array::from(vertices::V_COLORS.as_slice()).buffer();
        let colors_vbo = gl_context
            .create_buffer()
            .ok_or_else(|| JsValue::from_str("Failed to create colors VBO."))?;
        gl_context.bind_buffer(GL::ARRAY_BUFFER, Some(&colors_vbo));
        gl_context.buffer_data_with_opt_array_buffer(GL::ARRAY_BUFFER, Some(&colors_data), GL::STATIC_DRAW);
        gl_context.vertex_attrib_pointer_with_i32(color_attr_index, 4, GL::FLOAT, false, 0, 0);
        gl_context.enable_vertex_attrib_array(color_attr_index);

        Ok(Self { mesh })
    }

    pub fn render(&self, canvas_width: Number, canvas_height: Number, x_rotation: Number, y_rotation: Number) -> Result<()> {
        let gl = self.get_gl_context();

        let canvas_w = canvas_width.as_f64().map(|w| (w as i64) as i32).unwrap();
        let canvas_h = canvas_height.as_f64().map(|w| (w as i64) as i32).unwrap();
        let y_rot = y_rotation.as_f64().map(|w| (w as f32) * (std::f32::consts::PI / 180.0)).unwrap();
        let x_rot = x_rotation.as_f64().map(|w| (w as f32) * (std::f32::consts::PI / 180.0)).unwrap();

        let x_axis = glm::vec3(0.0, 1.0, 0.0);
        let y_axis = glm::vec3(1.0, 0.0, 0.0);
       
        gl.viewport(0, 0, canvas_w, canvas_h);
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear_depth(1.0);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let program = self.use_program();

        let identity_matrix = glm::TMat4::identity();
        let y_rot_matrix = glm::rotate(&identity_matrix, y_rot, &y_axis);
        let x_rot_matrix = glm::rotate(&identity_matrix, x_rot, &x_axis);
        let model_matrix = y_rot_matrix * x_rot_matrix;

        let m_uniform = gl.get_uniform_location(program, "m")
            .ok_or_else(|| JsValue::from_str("Failed to get 'm' uniform location."))?;

        gl.uniform_matrix4fv_with_f32_array(
            Some(&m_uniform),
            false,
            model_matrix.as_slice()
        );

        let cam_position = glm::vec3(0.0, 0.0, 5.0);
        let cam_target = glm::vec3(0.0, 0.0, -1.0);
        let cam_up = glm::vec3(0.0, 1.0, 0.0);
        let view_matrix = glm::look_at(&cam_position, &cam_target, &cam_up);

        let v_uniform = gl.get_uniform_location(program, "v")
            .ok_or_else(|| JsValue::from_str("Failed to get 'v' uniform location."))?;

        gl.uniform_matrix4fv_with_f32_array(
            Some(&v_uniform),
            false,
            view_matrix.as_slice()
        );

        let aspect_ratio = canvas_w as f32 / canvas_h as f32;
        let fov = std::f32::consts::PI / 4.0;
        let near = 0.1;
        let far = 100.0;
        let perspective_matrix = glm::perspective(aspect_ratio, fov, near, far);

        let p_uniform = gl.get_uniform_location(program, "p")
            .ok_or_else(|| JsValue::from_str("Failed to get 'p' uniform location."))?;

        gl.uniform_matrix4fv_with_f32_array(
            Some(&p_uniform),
            false,
            perspective_matrix.as_slice()
        );

        gl.draw_elements_with_i32(
            GL::TRIANGLES,
            vertices::INDICES.len().try_into().unwrap(),
            GL::UNSIGNED_SHORT,
            0
        );

        Ok(())
    }

    fn get_gl_context(&self) -> &GL {
        self.mesh.get_gl_context()
    }

    fn use_program(&self) -> &WebGlProgram {
        let program = &self.mesh.get_program();
        self.mesh.get_gl_context().use_program(Some(program));
        program
    }
}
