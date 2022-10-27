pub const VERTEX_SHADER: &'static str = r#"
attribute vec3 position;
attribute vec4 color;
varying vec4 vcolor;

uniform mat4 m;
uniform mat4 v;
uniform mat4 p;

void main() {
    gl_Position = p * v * m * vec4(position, 1.0);
    vcolor = color;
}
"#;

pub const FRAGMENT_SHADER: &'static str = r#"
precision mediump float;
varying vec4 vcolor;

void main() {
    gl_FragColor = vcolor;
}
"#;
