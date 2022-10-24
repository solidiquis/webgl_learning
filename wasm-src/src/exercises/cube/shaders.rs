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
precision mediump float;
varying vec3 vcolor;

void main() {
    gl_FragColor = vec4(vcolor, 1.0);
}
"#;
