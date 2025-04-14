use beryllium::*;
use ogl33::*;

mod app;
use crate::app::*;

fn main() {
  let mut app:App = App::new();
  /*let sdl = Sdl::init(init::InitFlags::EVERYTHING);

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
        resizable: false,
  };

  let win = sdl
    .create_gl_window(win_args)
    .expect("couldn't make a window and context");
  */
  /*unsafe{  
    glEnable(GL_TEXTURE_2D);
  }*/
  
 /*
  unsafe {
    let mut vao = 0;
    glGenVertexArrays(1, &mut vao);
    assert_ne!(vao, 0);

    let mut vbo = 0;
    glGenBuffers(1, &mut vbo);
    assert_ne!(vbo, 0);
    glBindBuffer(GL_ARRAY_BUFFER, vbo);
    type Vertex = [f32; 3];
    const VERTICES: [Vertex; 3] =
      [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];
  }*/
  'main_loop: loop {
    // handle events this frame
    while let Some(event) = app.sdl.poll_events() {
        match event {
            (events::Event::Quit, _) => break 'main_loop,
            _ => (),
        }

    }
    app.update();
  }
}
