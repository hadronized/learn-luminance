use glfw::{Action, Context as _, Key, WindowEvent};
use luminance::context::GraphicsContext as _;
use luminance::pipeline::PipelineState;
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};
use std::process::exit;
use std::time::Instant;

fn main() {
  let dim = WindowDim::Windowed {
    width: 960,
    height: 540,
  };
  let surface = GlfwSurface::new_gl33("Hello, world!", WindowOpt::default().set_dim(dim));

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

fn main_loop(surface: GlfwSurface) {
  let start_t = Instant::now();
  let mut ctxt = surface.context;
  let events = surface.events_rx;
  let back_buffer = ctxt.back_buffer().expect("back buffer");

  'app: loop {
    // handle events
    ctxt.window.glfw.poll_events();
    for (_, event) in glfw::flush_messages(&events) {
      match event {
        WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => break 'app,
        _ => (),
      }
    }

    // rendering code goes here
    // get the current time and create a color based on the time
    let t = start_t.elapsed().as_millis() as f32 * 1e-3;
    let color = [t.cos(), t.sin(), 0.5, 1.];

    let render = ctxt
      .new_pipeline_gate()
      .pipeline(
        &back_buffer,
        &PipelineState::default().set_clear_color(color),
        |_, _| Ok(()),
      )
      .assume();

    // swap buffer chains
    if render.is_ok() {
      ctxt.window.swap_buffers();
    } else {
      break 'app;
    }
  }
}
