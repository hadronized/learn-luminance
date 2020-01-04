use luminance::context::GraphicsContext as _;
use luminance::pipeline::PipelineState;
use luminance_glfw::{Action, GlfwSurface, Key, Surface as _, WindowDim, WindowEvent, WindowOpt};
use std::process::exit;
use std::time::Instant;

fn main() {
  let surface = GlfwSurface::new(
    WindowDim::Windowed(960, 540),
    "Hello, world!",
    WindowOpt::default(),
  );

  match surface {
    Ok(surface) => {
      eprintln!("graphics surface created");
      main_loop(surface);
    }

    Err(e) => {
      eprintln!("cannot create graphics surface:\n{}", e);
      exit(1);
    }
  }
}

fn main_loop(mut surface: GlfwSurface) {
  let start_t = Instant::now();
  let back_buffer = surface.back_buffer().unwrap();

  'app: loop {
    // handle events
    for event in surface.poll_events() {
      match event {
        WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => break 'app,
        _ => (),
      }
    }

    // rendering code goes here
    // get the current time and create a color based on the time
    let t = start_t.elapsed().as_millis() as f32 * 1e-3;
    let color = [t.cos(), t.sin(), 0.5, 1.];

    surface.pipeline_builder().pipeline(
      &back_buffer,
      &PipelineState::default().set_clear_color(color),
      |_, _| (),
    );

    // swap buffer chains
    surface.swap_buffers();
  }
}
