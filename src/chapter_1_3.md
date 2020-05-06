## Changing the background color

We’re getting to the interesting things. Before trying to render cool rotating cubes, we need to
understand how graphics rendering works in [luminance].

[luminance] has a special way of encoding renders. Instead of giving you all the GPU power right
away, it constrains you to pretty much none. That seems insane but you will eventually recognize
that such a way of doing is actually pretty useful. What it means is that by default, you cannot
do anything and you are required to build up types to unlock new GPU features.

To render something, you need several resources. The first resource is a _frame buffer_. A frame
buffer, encoded with [`Framebuffer`], is a special GPU resource that holds _frames_, i.e. the
pixel storage for renders!

> So… is the _back_ buffer you told us earlier some kind of [`Framebuffer`]?

It’s not _some_ kind: it **is** a [`Framebuffer`]. And guess what: you can access it via the
[`Surface::back_buffer`] method.

So, let’s make our first cool render and make a color-varying background!  First, you will need to
import one symbol from [luminance]: [`GraphicsContext`], which is a trait that allows you to run
[luminance] code and talk to the GPU. We will be using pipelines for our rendering. A graphics
pipeline is just a strongly typed description of what a GPU should do in order to render _things_
into a [`Framebuffer`]. You can picture pipelines as [AST]s in which each node represents a given
resource sharing and leaves are actual renders.

> More on [pipelines here](https://docs.rs/luminance/latest/luminance/index.html#understanding-the-pipeline-architecture).

We will also use [`Instant`], from the standard library, to
handle low-precision yet sufficient time points.

```rust
use luminance::context::GraphicsContext as _;
use luminance::pipeline::PipelineState;
use std::time::Instant;
```

> We don’t need, so far, to have access to the [`GraphicsContext`] symbol — we just need the
> implementors.

Now, let’s get our _back_ buffer.

```rust
fn main_loop(mut surface: GlfwSurface) {
  let start_t = Instant::now();
  let back_buffer = surface.back_buffer().unwrap();

  'app: loop {
    // …
```

As you can see, getting the _back_ buffer is piece of cake. Now let’s handle that color.

```rust
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
```

That’s already a lot of code to discuss. `surface.pipeline_builder()` gets a lightweight object
that you can use to create _graphics pipelines_ — its type is [`Builder`]. You can get that type
once and for all and keep it around if you want to but in our case, since we’re only going to create
a single pipeline, we’ll just chain everything.

Then, the [`Builder::pipeline`] function, applied to the [`Builder`] object, creates a graphics
pipeline. In our case, we don’t want to render anything, we just want to modify the _back_ buffer background
color. That is done with the arguments you pass to [`Builder::pipeline`]. The first one is the
frame buffer to render to. In our case, it’s our _back_ buffer.

The second argument is the _pipeline state_ to use when running our pipeline. Everytime you perform a
render into a frame buffer, you _need_ to provide such an object, which contains the color to use when
clearing the framebuffer’s color buffers. It is possible to tell [luminance] not to clear color buffers
but this is off topic.

The third and last argument is a _closure_ you need to pass. That closure will be called as soon as
the frame buffer is ready to receive a render. All this code is fully _synchronous_ though, so
lifetimes are enforced. In our case, since we’re not interested into making any actual render,
we just pass a closure that does nothing. More on its two arguments later.

You should obtain a window with a varying color, such as the following screenshot.

![](./imgs/hello-world.png)

The complete code is:

```rust
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
```

[luminance]: https://crates.io/crates/luminance
[luminance-glfw]: https://crates.io/crates/luminance-glfw
[cargo-watch]: https://crates.io/crates/cargo-watch
[double buffering]: https://en.wikipedia.org/wiki/Multiple_buffering
[`Surface::poll_events`]: https://docs.rs/luminance-windowing/latest/luminance_windowing/trait.Surface.html#tymethod.poll_events
[`Surface::swap_buffers`]: https://docs.rs/luminance-windowing/latest/luminance_windowing/trait.Surface.html#tymethod.swap_buffers
[`Framebuffer`]: https://docs.rs/luminance/latest/luminance/framebuffer/struct.Framebuffer.html
[`Surface::back_buffer`]: https://docs.rs/luminance-windowing/latest/luminance_windowing/trait.Surface.html#method.back_buffer
[`GraphicsContext`]: https://docs.rs/luminance/latest/luminance/context/trait.GraphicsContext.html
[`Instant`]: https://doc.rust-lang.org/std/time/struct.Instant.html
[`Builder`]: https://docs.rs/luminance/latest/luminance/pipeline/struct.Builder.html
[`Builder::pipeline`]: https://docs.rs/luminance/latest/luminance/pipeline/struct.Builder.html#method.pipeline
[AST]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
