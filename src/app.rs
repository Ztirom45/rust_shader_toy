use std::{ffi::CString, time::Instant};

use beryllium::{init::InitFlags, video::GlWindow, *};

use ogl33::*;
use crate::shader::*;

//Warning: left_button and right button updates are not implemented yet
pub struct MouseState{
  pub x:i32,
  pub y:i32,
  pub left_button:bool,
  pub right_button:bool,
}

pub struct App{
  pub sdl:Sdl,
  pub win:GlWindow,
  pub shader_program:u32,
  pub start_time:Instant,
  pub is_running:bool,
  pub i_mouse:MouseState,
}

 
type Vertex = [f32; 3];
const VERTICES: [Vertex; 3] =
    [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];
    


impl App{
  pub fn new() -> Self{
    let sdl = Sdl::init(InitFlags::EVERYTHING);

    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();
    #[cfg(target_os = "macos")]
    {
    sdl
      .set_gl_context_flags(video::GlContextFlags::FORWARD_COMPATIBLE)
      .unwrap();
    }

    let win_args = video::CreateWinArgs {
        title: "test",
        width: 800,
        height: 600,
        allow_high_dpi: true,
        borderless: false,
        resizable: true,
      };

    let win = sdl
      .create_gl_window(win_args)
      .expect("couldn't make a window and context");
    /*unsafe{  
      ogl33::glEnable(ogl33::GL_TEXTURE_2D);
    }*/
    //creat mesh
    let shader_program:u32;
    unsafe{ 
      load_gl_with(|f_name| win.get_proc_address(f_name.cast()));

      glClearColor(0.2, 0.3, 0.3, 1.0);

      let mut vao = 0;
      glGenVertexArrays(1, &mut vao);
      assert_ne!(vao, 0);
      glBindVertexArray(vao);
      glEnableVertexAttribArray(0);
      shader_program = make_shader();
    }
    //let shader = Shader::new(&win);
    return Self{
      sdl,
      win,
      shader_program,
      start_time:Instant::now(),
      is_running:true,
      i_mouse:MouseState { x: 0, y: 0, left_button: false, right_button: false }
    };
  }

  pub  fn update_uniforms(&mut self){
        //update iResolution
        
        let i_resolution = self.win.get_window_size();
        unsafe{ 
        load_gl_with(|f_name| self.win.get_proc_address(f_name as *const u8));
        glViewport(0, 0, i_resolution.0*2, i_resolution.1*2);
        
        let i_resolution_location = glGetUniformLocation(
        self.shader_program,
        CString::new("iResolution").unwrap().as_ptr() as *const i8
        );
	if i_resolution_location>=0{
		glUniform2f(i_resolution_location,i_resolution.0 as f32,i_resolution.1 as f32);
	}else{
		println!("error: couldn't find iResolution uniform. maybe not used\n");
	}
        
        //update iTime
        let i_time:f32 = self.start_time.elapsed().as_secs_f32();
        let i_time_location = glGetUniformLocation(
        self.shader_program,
        CString::new("iTime").unwrap().as_ptr() as *const i8
        );
        
	if i_time_location>=0{
		glUniform1f(i_time_location,i_time);
	}else{
		println!("error: couldn't find iTime uniform. maybe unused\n");
	}


        //update iMouse
        let i_mouse_location = glGetUniformLocation(
        self.shader_program,
        CString::new("iMouse").unwrap().as_ptr() as *const i8
        );
        
	if i_mouse_location>=0{
		glUniform4f(
                  i_mouse_location,
                  self.i_mouse.x as f32,
                  self.i_mouse.y as f32,
                  self.i_mouse.left_button as i32 as f32,
                  self.i_mouse.right_button as i32 as f32
                );
	}else{
		println!("error: couldn't find iMouse uniform. maybe unused\n");
	}
        }

   
  }
  pub fn pre_draw(&mut self){
    unsafe {
      load_gl_with(|f_name| self.win.get_proc_address(f_name as *const u8));
      //glClearColor(0.2, 0.3, 0.3, 1.0);
      glClear(GL_COLOR_BUFFER_BIT);
    }

  }/*
  pub unsafe fn  print_cstring(&self,data: *const u8){//just for debugging realy buggy don't use
    println!("{}",std::str::from_utf8_unchecked(std::slice::from_raw_parts(data, 40)));
  }
  pub fn get_info(&self){//just for debugging realy buggy don't use
    unsafe{
      load_gl_with(|f_name| self.win.get_proc_address(f_name as *const u8));
      self.print_cstring(glGetString(GL_VENDOR));
      self.print_cstring(glGetString(GL_RENDERER));
      self.print_cstring(glGetString(GL_VERSION));
      self.print_cstring(glGetString(GL_SHADING_LANGUAGE_VERSION));
    }		
  }*/
  pub fn draw(&mut self){
    unsafe{
      glDrawArrays(GL_TRIANGLE_STRIP, 0, 4);
    }
  }

  pub fn handle_events(&mut self){
      while let Some(event) = self.sdl.poll_events() {
        match event {
            (events::Event::Quit, _) => self.is_running = false,
            (events::Event::MouseMotion{
              win_id: _, 
              mouse_id: _, 
              button_state: _,
              x_win,
              y_win, 
              x_delta: _, 
              y_delta: _ }, _) => {
                self.i_mouse.x = x_win;
                self.i_mouse.y = y_win;
              
              },
            _ => (),
        }
      }

      //let x = events.mouse_state().x1();
  }

  pub fn update(&mut self){ 
    self.handle_events();
    self.update_uniforms();

    self.pre_draw();
    self.draw();
    self.win.swap_window();
  }

  pub fn run(&mut self){
    while self.is_running{
      self.update();
    }
  self.clear(); 
  }
  pub fn clear(&mut self){
    //self.shader.clear(&self.win);
  }
}
