use super::super::shader::Shader;
use super::Mesh;
use super::state::{self, MeshState};
use wasm_bindgen::JsValue;
use web_sys::{WebGlShader, WebGlRenderingContext as GL};
pub struct MeshBuilder<S: MeshState> {
    gl: GL,
    state: Box<S>
}

pub type Result<T> = std::result::Result<T, JsValue>;

impl MeshBuilder<state::Initialized> {
    pub fn new(gl: GL) -> MeshBuilder<state::Initialized> {
        Self { gl, state: Box::new(state::Initialized) }
    }

    pub fn compile_shaders(self, vertex_shader: Shader, fragment_shader: Shader) -> Result<MeshBuilder<state::ShadersCompiled>> {
        let gl = self.gl;

        let vertex_shader = Self::compile_shader(&gl, vertex_shader)?;
        let fragment_shader = Self::compile_shader(&gl, fragment_shader)?;

        let state = state::ShadersCompiled { vertex_shader, fragment_shader };

        Ok(MeshBuilder { gl, state: Box::new(state) })
    }

    fn compile_shader(gl: &GL, shader: Shader) -> Result<WebGlShader> {
        let compile_src = |gl_shader| {
            gl.shader_source(&gl_shader, shader.use_src());
            gl.compile_shader(&gl_shader);
            Some(gl_shader)
        };

        let compiled_shader = gl
            .create_shader(shader.as_u32())
            .and_then(compile_src)
            .ok_or(JsValue::from_str("Failed to create vertex shader."))?;

        let success = gl.get_shader_parameter(&compiled_shader, GL::COMPILE_STATUS).as_bool().unwrap_or(false);

        if success { return Ok(compiled_shader) }

        let shader_kind = shader.kind_as_string();

        let err = if let Some(ref info_log) = gl.get_shader_info_log(&compiled_shader) {
            Err(JsValue::from_str(&format!("Failed to compile {shader_kind} with error: {info_log}.")))
        } else {
            Err(JsValue::from_str(&format!("Failed to compile {shader_kind}.")))
        };

        gl.delete_shader(Some(&compiled_shader));

        err
    }
}

impl MeshBuilder<state::ShadersCompiled> {
    pub fn link_program(self) -> Result<MeshBuilder<state::ProgramLinked>> {
        let gl = self.gl;
        let state_object = *self.state;

        let program = gl.create_program().ok_or(JsValue::from_str("Failed to create program."))?;

        let vertex_shader = state_object.vertex_shader;
        let fragment_shader = state_object.fragment_shader;

        gl.attach_shader(&program, &vertex_shader);
        gl.attach_shader(&program, &fragment_shader);

        gl.link_program(&program);

        let success = gl.get_program_parameter(&program, GL::LINK_STATUS).as_bool().unwrap_or(false);

        if success {
            let state = state::ProgramLinked { program };
            return Ok(MeshBuilder { gl, state: Box::new(state) })
        }

        let mut err_msg = "Failed to link program.".to_owned();

        if let Some(ref info_log) = gl.get_program_info_log(&program) {
            err_msg = format!("Failed to link program with error: {info_log}.");
        }

        gl.delete_program(Some(&program));

        Err(JsValue::from_str(&err_msg))
    }
}

impl MeshBuilder<state::ProgramLinked> {
    pub fn finalize(self) -> Mesh {
        let gl = self.gl;
        let state_object = *self.state;
        let program = state_object.program;

        Mesh { gl, program }
    }
}
