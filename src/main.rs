use beryllium::*;

mod app;
mod shader;
use crate::app::*;

fn main() {
  let mut app:App = App::new();
 
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
  app.clear();
}
