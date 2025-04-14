use beryllium::*;

mod app;
mod shader;
use crate::app::*;

fn main() {
  let mut app:App = App::new();
 
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
  app.clear();
}
