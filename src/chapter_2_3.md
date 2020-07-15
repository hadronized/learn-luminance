# Rendering our triangle

A [`RenderGate`] allows to create _render nodes_. Such nodes will share [`RenderState`]s for all
lower nodes in the graphics pipeline. We will see what we can do with [`RenderState`] in a future
chapter. Currently, we will just use the default one.

```rust
use luminance::render_state::RenderState;
```

And alter your pipeline:

```rust
    let render = surface.new_pipeline_gate().pipeline(
      &back_buffer,
      &PipelineState::default().set_clear_color(color),
      |_, mut shd_gate| {
        shd_gate.shade(&mut program, |_, _, mut rdr_gate| {
          rdr_gate.render(&RenderState::default(), |mut tess_gate| {
            // …
          });
        });
      },
    );
```

We’re almost there. We’re getting a [`TessGate`], allowing us to render actual tessellations. In
order to do so, we will need to create a [`TessView`] out of our [`Tess`]. That enables to slice
GPU tessellation on the fly for free. In our case, we want the whole thing (the whole triangle).
Some convenient implementors exist, and there’s one allowing to get a `TessView` from a `&Tess`.

> You can also use the [`View`] trait to do such a thing.

Let’s go and finish it.

```rust
let render = surface.new_pipeline_gate().pipeline(
  &back_buffer,
  &PipelineState::default().set_clear_color(color),
  |_, mut shd_gate| {
    shd_gate.shade(&mut program, |_, _, mut rdr_gate| {
      rdr_gate.render(&RenderState::default(), |mut tess_gate| {
        tess_gate.render(&triangle);
      });
    });
  },
);
```

Compile and run the code. You should see something similar to this:

![](imgs/your_first_triangle.png)

The complete code:

```rust
use glfw::{Action, Context as _, Key, WindowEvent};
use luminance::context::GraphicsContext as _;
use luminance::pipeline::PipelineState;
use luminance::render_state::RenderState;
use luminance::tess::Mode;
use luminance_derive::{Semantics, Vertex};
use luminance_glfw::{GlfwSurface, WindowDim, WindowOpt};
use std::process::exit;
use std::time::Instant;

#[derive(Copy, Clone, Debug, Semantics)]
pub enum VertexSemantics {
  #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
  Position,
  #[sem(name = "color", repr = "[u8; 3]", wrapper = "VertexRGB")]
  Color,
}

#[derive(Clone, Copy, Debug, Vertex)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
  #[allow(dead_code)]
  position: VertexPosition,

  #[allow(dead_code)]
  #[vertex(normalized = "true")]
  color: VertexRGB,
}

const VERTICES: [Vertex; 3] = [
  Vertex::new(
    VertexPosition::new([-0.5, -0.5]),
    VertexRGB::new([255, 0, 0]),
  ),
  Vertex::new(
    VertexPosition::new([0.5, -0.5]),
    VertexRGB::new([0, 255, 0]),
  ),
  Vertex::new(VertexPosition::new([0., 0.5]), VertexRGB::new([0, 0, 255])),
];

const VS_STR: &str = include_str!("vs.glsl");
const FS_STR: &str = include_str!("fs.glsl");

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

fn main_loop(mut surface: GlfwSurface) {
  let start_t = Instant::now();

  let triangle = surface
    .new_tess()
    .set_vertices(&VERTICES[..])
    .set_mode(Mode::Triangle)
    .build()
    .unwrap();

  let mut program = surface
    .new_shader_program::<VertexSemantics, (), ()>()
    .from_strings(VS_STR, None, None, FS_STR)
    .unwrap()
    .ignore_warnings();

  'app: loop {
    // handle events
    surface.window.glfw.poll_events();
    for (_, event) in surface.events_rx.try_iter() {
      match event {
        WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => break 'app,

        _ => (),
      }
    }

    let back_buffer = surface.back_buffer().unwrap();

    // rendering code goes here
    // get the current time and create a color based on the time
    let t = start_t.elapsed().as_millis() as f32 * 1e-3;
    let color = [t.cos(), t.sin(), 0.5, 1.];

    let render = surface.new_pipeline_gate().pipeline(
      &back_buffer,
      &PipelineState::default().set_clear_color(color),
      |_, mut shd_gate| {
        shd_gate.shade(&mut program, |_, _, mut rdr_gate| {
          rdr_gate.render(&RenderState::default(), |mut tess_gate| {
            tess_gate.render(&triangle);
          });
        });
      },
    );

    // swap buffer chains
    if render.is_ok() {
      surface.window.swap_buffers();
    } else {
      break 'app;
    }
  }
}
```

[`Tess`]: https://docs.rs/luminance/latest/luminance/tess/struct.Tess.html
[`RenderGate`]: https://docs.rs/luminance/latest/luminance/pipeline/struct.RenderGate.html
[`RenderState`]: https://docs.rs/luminance/latest/luminance/render_state/struct.RenderState.html
[`TessGate`]: https://docs.rs/luminance/latest/luminance/pipeline/struct.TessGate.html
