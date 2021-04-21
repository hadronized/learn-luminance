## Creating a window and preparing graphics code

The first thing you want to do is to create a window. This is done via a type you can find in
[luminance-glfw]: `GlfwSurface`. A _graphics surface_ is a common term yet a bit opaque describing
a region of memory that accepts getting rendered to.

Creating a new _graphics surface_ relies on several objects you need to decide values for:

- The dimensions of the surface. In our case, we will use a screen resolution of _960×540_ just
  for the sake of the introduction. You create such dimensions with the `WindowDim::Windowed`
  variant. Others are available, like restricted fullscreen or fullscreen.
- The title of the window. Whether it will get displayed depends on your compositor and window
  manager but pretty much all of them display the title in the top-level location of your window
  as a _window decoration property_. Titles are encoded as simple `&str`.
- A set of options to tweak the window and system-related properties. For now, that last part is
  too advanced and we’ll stick to using the defaults. Use `WindowOpt::default()`.

Getting a `GlfwSurface` might fail, so you need to handle failures via the `Result` type.

Once you get your surface, you can start playing with it. How depends on what kind of application
you want to write. For a video game, a simulation, a demo or an animation program, the following
_event-render_ loop is enough. You ask for the event handler to check whether events have occurred
and dequeue them all. Once you’re done, you render a frame, and you loop back.

You quit the application if the window gets closed by the user or if they enter the _escape_ key,
for instance.

```rust
use glfw::{Action, Context as _, Key, WindowEvent};
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};
use std::process::exit;

fn main() {
  // our graphics surface
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
  let mut ctxt = surface.context;
  let events = surface.events_rx;
  let back_buffer = ctxt.back_buffer().expect("back buffer");

  'app: loop {
    // handle events
    ctxt.window.glfw.poll_events();
    for (_, event) in glfw::flush_messages(&events) {
      match event {
        WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => break 'app,
        _ => ()
      }
    }

    // rendering code goes here
    // …

    // swap buffer chains
    ctxt.window.swap_buffers();
  }
}
```

`ctxt.window.glfw.poll_events()` allows you to have a look whether you need to dequeue events.
If no event is present in the event queue, that function exits immediately instead of blocking for
an event. That allows you to keep maintaining a constant frame rate.

`ctxt.window.swap_buffers()` takes the graphics _back_ buffer linked to your application and
swaps it with the _front_ buffer, which is the one exposed on your screen. That technique is
commonly referred to as [double buffering]. With [luminance], all the renders must eventually end
up in the _back_ buffer so that they get swapped at the end of the main loop. It’s important to
notice — and you will see in future tutorials — that you don’t necessarily have to make your
renders _directly_ into the back buffer. More on that later… ;)

[luminance]: https://crates.io/crates/luminance
[luminance-glfw]: https://crates.io/crates/luminance-glfw
[double buffering]: https://en.wikipedia.org/wiki/Multiple_buffering
