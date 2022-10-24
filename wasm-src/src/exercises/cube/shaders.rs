pub const VERTEX_SHADER: &'static str = r#"
attribute vec3 position;
attribute vec3 color;
varying vec3 vcolor;

void main() {
    gl_Position = vec4(position, 1.0);
    vcolor = color;
}
"#;

pub const FRAGMENT_SHADER: &'static str = r#"
attribute vec3 position;
varying vec4 vcolor;

void main() {
    gl_FragColor = color;
}
"#;
