use ogl33::*;
use std::env;
use std::ffi::CString;
use std::fs;

const VERT_SHADER: &str = r#"#version 420 core
const vec2 quadVertices[4] = { vec2(-1.0, -1.0), vec2(1.0, -1.0), vec2(-1.0, 1.0), vec2(1.0, 1.0) };
void main()
{
    gl_Position = vec4(quadVertices[gl_VertexID], 0.0, 1.0);
} "#;
pub fn get_fragment_shader() -> String{ 
    let contents = fs::read_to_string("src/shader.fs.glsl")
        .expect("Should have been able to read the file");
    return contents;
}

pub unsafe fn make_shader() -> u32{
    let vertex_shader = glCreateShader(GL_VERTEX_SHADER);
    assert_ne!(vertex_shader, 0);
    glShaderSource(
      vertex_shader,
      1,
      &(VERT_SHADER.as_bytes().as_ptr().cast()),
      &(VERT_SHADER.len().try_into().unwrap()),
    );
    glCompileShader(vertex_shader);
    let mut success = 0;
    glGetShaderiv(vertex_shader, GL_COMPILE_STATUS, &mut success);
    if success == 0 {
      let mut v: Vec<u8> = Vec::with_capacity(1024);
      let mut log_len = 0_i32;
      glGetShaderInfoLog(
        vertex_shader,
        1024,
        &mut log_len,
        v.as_mut_ptr().cast(),
      );
      v.set_len(log_len.try_into().unwrap());
      panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
    }
    
    let FRAG_SHADER = get_fragment_shader();
    let fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
    assert_ne!(fragment_shader, 0);
    glShaderSource(
      fragment_shader,
      1,
      &(FRAG_SHADER.as_bytes().as_ptr().cast()),
      &(FRAG_SHADER.len().try_into().unwrap()),
    );
    glCompileShader(fragment_shader);
    let mut success = 0;
    glGetShaderiv(fragment_shader, GL_COMPILE_STATUS, &mut success);
    if success == 0 {
      let mut v: Vec<u8> = Vec::with_capacity(1024);
      let mut log_len = 0_i32;
      glGetShaderInfoLog(
        fragment_shader,
        1024,
        &mut log_len,
        v.as_mut_ptr().cast(),
      );
      v.set_len(log_len.try_into().unwrap());
      panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
    }

    let shader_program = glCreateProgram();
    assert_ne!(shader_program, 0);
    glAttachShader(shader_program, vertex_shader);
    glAttachShader(shader_program, fragment_shader);
    glLinkProgram(shader_program);
    let mut success = 0;
    glGetProgramiv(shader_program, GL_LINK_STATUS, &mut success);
    if success == 0 {
      let mut v: Vec<u8> = Vec::with_capacity(1024);
      let mut log_len = 0_i32;
      glGetProgramInfoLog(
        shader_program,
        1024,
        &mut log_len,
        v.as_mut_ptr().cast(),
      );
      v.set_len(log_len.try_into().unwrap());
      panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
    }
    glDeleteShader(vertex_shader);
    glDeleteShader(fragment_shader);

    glUseProgram(shader_program);
    shader_program
}

