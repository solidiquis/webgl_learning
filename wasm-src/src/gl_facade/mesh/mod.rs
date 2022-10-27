use web_sys::{WebGlProgram, WebGlRenderingContext as GL};

pub mod state;
pub mod builder;

pub struct Mesh {
    gl: GL,
    program: WebGlProgram
}

impl Mesh {
    pub fn get_gl_context(&self) -> &GL {
        &self.gl
    }

    pub fn get_program(&self) -> &WebGlProgram {
        &self.program
    }
}
