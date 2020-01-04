use luminance::context::GraphicsContext as _;
use luminance::pipeline::PipelineState;
use luminance::render_state::RenderState;
use luminance::shader::program::Program;
use luminance::tess::{Mode, TessBuilder, TessSliceIndex as _};
use luminance_derive::{Semantics, Vertex};
use luminance_glfw::{Action, GlfwSurface, Key, Surface as _, WindowDim, WindowEvent, WindowOpt};
use std::process::exit;
use std::time::Instant;

#[derive(Copy, Clone, Debug, Semantics)]
pub enum VertexSemantics {
  #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
  Position,
  #[sem(name = "color", repr = "[u8; 3]", wrapper = "VertexRGB")]
  Color,
}

#[derive(Vertex)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
  position: VertexPosition,
  #[vertex(normalized = "true")]
  color: VertexRGB,
}

const VERTICES: [Vertex; 3] = [
  Vertex {
    position: VertexPosition::new([-0.5, -0.5]),
    color: VertexRGB::new([255, 0, 0]),
  },
  Vertex {
    position: VertexPosition::new([0.5, -0.5]),
    color: VertexRGB::new([0, 255, 0]),
  },
  Vertex {
    position: VertexPosition::new([0., 0.5]),
    color: VertexRGB::new([0, 0, 255]),
  },
];

const VS_STR: &str = include_str!("vs.glsl");
const FS_STR: &str = include_str!("fs.glsl");

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

  let triangle = TessBuilder::new(&mut surface)
    .add_vertices(VERTICES)
    .set_mode(Mode::Triangle)
    .build()
    .unwrap();

  let program: Program<VertexSemantics, (), ()> = Program::from_strings(None, VS_STR, None, FS_STR)
    .unwrap()
    .ignore_warnings();

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
      |_, mut shd_gate| {
        shd_gate.shade(&program, |_, mut rdr_gate| {
          rdr_gate.render(&RenderState::default(), |mut tess_gate| {
            tess_gate.render(triangle.slice(..));
          });
        });
      },
    );

    // swap buffer chains
    surface.swap_buffers();
  }
}
