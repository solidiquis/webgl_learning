use std::fmt::{self, Display};
use web_sys::WebGlRenderingContext as GL;

pub struct Shader<'a> {
    kind: ShaderKind,
    src: &'a str
}

impl<'a> Shader<'a> {
    pub fn new(kind: ShaderKind, src: &'a str) -> Self {
        Self { kind, src }
    }

    pub fn use_src(&self) -> &'a str {
        &self.src
    }

    pub fn as_u32(&self) -> u32 {
        self.kind.as_u32()
    }

    pub fn kind_as_string(&self) -> String {
        format!("{}", self.kind)
    }
}

#[derive(Debug)]
pub enum ShaderKind {
    Vertex,
    Fragment
}

impl ShaderKind {
    fn as_u32(&self) -> u32 {
        match self {
            Self::Vertex => GL::VERTEX_SHADER,
            Self::Fragment => GL::FRAGMENT_SHADER
        }
    }
}

impl Display for ShaderKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShaderKind::Vertex => write!(f, "vertex shader"),
            ShaderKind::Fragment => write!(f, "fragment shader")
        }
    }
}
