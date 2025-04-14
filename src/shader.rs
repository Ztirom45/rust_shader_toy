use beryllium::video::GlWindow;
use ogl33::*;
const VERT_SHADER: &str = r#"#version 420 core
const vec2 quadVertices[4] = { vec2(-1.0, -1.0), vec2(1.0, -1.0), vec2(-1.0, 1.0), vec2(1.0, 1.0) };
void main()
{
    gl_Position = vec4(quadVertices[gl_VertexID], 0.0, 1.0);
} "#;

const FRAG_SHADER: &str = r#"#version 430 core
  out vec4 final_color;

  void main() {
    final_color = vec4(1.0, 1.0, 1.0, 1.0);
  }
"#;

pub struct Shader{
  pub vertex_shader:u32,
  pub fragment_shader:u32,
  pub shader_program:u32,
}

impl Shader{
  pub fn new(win:&GlWindow) -> Self{

    let vertex_shader:u32;
    let fragment_shader:u32;
    let shader_program:u32;
    unsafe { 
      load_gl_with(|f_name| win.get_proc_address(f_name as *const u8));
      //create vertex shader
      vertex_shader = glCreateShader(GL_VERTEX_SHADER);
      assert_ne!(vertex_shader, 0);
      
      glShaderSource(
        vertex_shader,
        1,
        &(VERT_SHADER.as_bytes().as_ptr().cast()),
        &(VERT_SHADER.len().try_into().unwrap()),
      );

      //compile vertex shader
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

      //create fragment shader
      fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
      assert_ne!(fragment_shader, 0);

      glShaderSource(
        fragment_shader,
        1,
        &(FRAG_SHADER.as_bytes().as_ptr().cast()),
        &(FRAG_SHADER.len().try_into().unwrap()),
      );

      //compile fragment shader shader
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

      //link shader 
      shader_program = glCreateProgram();
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
    }

    return Self{
      vertex_shader, 
      fragment_shader, 
      shader_program
    };
  }
  pub fn clear(&mut self,win:&GlWindow){
    unsafe { 
      load_gl_with(|f_name| win.get_proc_address(f_name as *const u8));
      glDeleteShader(self.vertex_shader);
      glDeleteShader(self.fragment_shader);
    }
  }
}


