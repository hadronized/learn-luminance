# Getting started

You’ll need some prerequisites:

| Prerequisite                   | How to get / resolve                            |
| ------------------------------ | ----------------------------------------------- |
| `rustup`                       | [https://rustup.rs](https://rustup.rs)          |
| An up to date `rustc` compiler | `rustup update stable && rustup default stable` |

Let’s create a new _Rust_ project in your `~/dev` directory, for instance. We’ll name the project
`luminance-hello-world`.

```
mkdir ~/dev # choose any other directory that suits you
cd ~/dev
cargo new --bin luminance-first-steps
```

Then we need to add [luminance] as a dependency to our project. Edit your `Cargo.toml` file and
change the `[dependencies]` section according to the following:

```toml
[dependencies]
luminance = "0.40"
luminance-glfw = "0.13"
glfw = "0.39"
```

Some explanations here:

- [luminance] is the core crate and contains everything that abstracts over GPU graphics
  capabilities. You will mostly use that crate to write graphics code.
- [luminance-glfw] is a _platform_ implementation crate for [luminance]. There are several crates
  available for that, depending on the platform you expect to run on. [luminance-glfw] supports a
  lot of platforms so you should be good to start (plus it’s pretty easy). Windowing code allows
  to ask your system to create a window, handle inputs such as keypresses, mouse movements, touch,
  etc. etc.

> Note: since version `0.9`, [luminance-windowing] has lost most of its “implemente” symbols.
> Platform crates such as [luminance-glfw] then don’t re-export the underyling crate’s symbols.
> For that reason, you will have to depend on them explicitely, as we do with [glfw] in this case.

Optional but highly recommended: install [cargo-watch]. That tool allows you to have a `cargo`
loop updating every time a code file in your project changes. You can set it up to re-compile,
re-check, re-test, re-run or even re-doc… Very handy.

```
cargo install cargo-watch
```

You’re now ready to get started.

[luminance]: https://crates.io/crates/luminance
[luminance-glfw]: https://crates.io/crates/luminance-glfw
[luminance-windowing]: https://crates.io/crates/luminance-glfw
[glfw]: https://crates.io/crates/glfw
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
