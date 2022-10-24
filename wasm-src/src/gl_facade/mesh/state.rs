use web_sys::{WebGlProgram, WebGlShader};

pub trait MeshState {}

pub struct Initialized;

pub struct ShadersCompiled {
    pub vertex_shader: WebGlShader,
    pub fragment_shader: WebGlShader
}

pub struct ProgramLinked {
    pub program: WebGlProgram
}

impl MeshState for Initialized {}
impl MeshState for ShadersCompiled {}
impl MeshState for ProgramLinked {}
