use beryllium::{init::InitFlags, video::GlWindow, *};
use ogl33::*;

pub struct App{
  pub sdl:Sdl,
  pub win:GlWindow,
}

impl App{
  pub fn new() -> App{
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
        resizable: false,
      };

    let win = sdl
      .create_gl_window(win_args)
      .expect("couldn't make a window and context");
    /*unsafe{  
      glEnable(GL_TEXTURE_2D);
    }*/
    return App{
      sdl,
      win
    };
  }
}
