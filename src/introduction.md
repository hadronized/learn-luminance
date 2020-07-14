# Learn luminance

You’re here because you want to learn luminance. Here is the chapter summary.

## Plan of the book

The book is written in chapters. It is highly recommended to read them in order, as they will
reference code introduced in previous chapters. Also, each chapter has a dedicated Rust project
containing the final solution [here](https://github.com/rust-tutorials/learn-luminance/tree/master/examples).
Feel free to consult the code if you would like to build each chapters on your own without having
to write the code.

You can compile all chapters by cloning the repository and building them all at once, or build and
a specific one:

```sh
git clone --depth 1 https://github.com/rust-tutorials/learn-luminance
cd learn-luminance/examples
# build everything…
cargo build --release

# …or run the chapter 3
cargo run --release --bin chapter-3 ~/Downloads/torus.obj
```

## Everything you should know before jumping in

**luminance** is an effort to make graphics rendering simple and elegant. It was originally imagined,
designed and implemented by [@phaazon] in Haskell ([here](https://hackage.haskell.org/package/luminance))
and eventually ported to Rust in 2016. The core concepts remained the same and the crate has been
slowly evolving ever since. At first used only by [@phaazon] for his Rust demoscene productions (
example [here](https://github.com/phaazon/celeri-remoulade) and
[here](https://github.com/phaazon/outline-2017-invitro), using
[spectra](https://crates.io/crates/spectra)) and a bunch of curious peeps, it has started to
gain more visibility among the graphics ecosystem of Rust and bring more people in.

Currently, such an ecosystem is spread into several crates, ideas and people. It is highly
recommended to read the great article about the ecosystem by [@icefoxen],
[here](https://wiki.alopex.li/AGuideToRustGraphicsLibraries2019).

However, **luminance** is a bit different from what it was initially imagined for. People are
looking for an easy-to-use crate, with good abstractions and safe-guards against all the _bad_ and
_dangerous_ graphics API caveats. **luminance** has always been about providing a safe, type-safe
and elegant API (being Haskell-based makes it heavily use type systems, for instance) but it has
now a more accurate place in the ecosystem. Where [gfx-hal] provides you with an experience focused
on down-to-metal performances and an API very similar to [Vulkan]’s, **luminance** provides an API
that is, for sure, a bit less low-level — and hence, yes, it’s likely you will not have the same
performances as with [gfx-hal] (even though no benchmarks have been done so far), and the API is not
[Vulkan]-based — but easier to start with, especially if you don’t already have a background
experience with [OpenGL] or [Vulkan]. Furthermore, the API of Vulkan is great to build low-level
primitives, while the audience of **luminance** is a bit higher-level.

The strengths of **luminance** are:

- Easy to learn: the concepts, based on [OpenGL], are applied to _graphics_, not _general-purpose
  programming on GPU_. Using **luminance** will help you wrap your fingers around what graphics
  programming is about and it will help you to, perhaps, jump to lower abstractions like
  [gfx-hal], if you ever need to.
- Performant: by using Rust and being designed around the concept of good performances,
  **luminance** should allow you to build nice and fast simulations, animations and video games.
  Remember that games you played years ago didn’t have [Vulkan] and were impressive nonetheless.
  It’s unlikely you will get 100% out of your GPU by using **luminance** since it’s built over
  technologies that are not using 100% of your GPU. Unless you need and know exactly why you need
  100% of your GPU, you should be _just fine™_.
- Elegant: the design is heavily based on functional programming concepts such as typeclasses,
  associated types, singleton types, existentials, contravariant resources, procedural macros,
  strong typing, etc. Plus, every bit of possible _stateful_ computations is hidden behind a
  system of smart state, removing the need to worry about side-effects. **luminance** still has
  mutation (unlike its Haskell version) but the Rust type-system and borrow checker allow for
  safe mutations.
- Modern: the whole **luminance** ecosystem tries its best to stay up-to-date with Rust evolutions
  and features. On the same level, the underneath technologies are kept up-to-date and might even
  change if a more modern and more adapted one emerges ([Vulkan] might eventually get adopted but
  this is just an idea for now).
- _Opinionated enough_: a big bet with **luminance** was to make it opinionated, but not too much.
  It needs to be opinionated to allow some design constructs to be possible and optimize
  performance and allow for extra safety. However, it must not be too much to prevent it to become
  a _framework_. **luminance** is a _library_, not a _framework_, meaning that it will adapt to
  how **you** think you should design your software, not the other way around. That is limited to
  the design of **luminance** but you shouldn’t feel too hands-tied.

Some practical information you want to know before learning:

- The [official GitHub repository](https://github.com/phaazon/luminance-rs)
- The [luminance examples](https://github.com/phaazon/luminance-rs/blob/master/luminance-examples/README.md).
  Those are useful when you know what you are looking for and would like to see quickly how to do
  it with luminance, or just to have a rough idea of what’s supported.
- Spotted a bug? A typo? A performance issue? You need a feature that’s not already available? Shoot it
  [here](https://github.com/phaazon/luminance-rs/issues).

[@phaazon]: https://github.com/phaazon
[@icefoxen]: https://github.com/icefoxen
[gfx-hal]: https://crates.io/crates/gfx-hal
[Vulkan]: https://www.khronos.org/vulkan
[Opengl]: https://www.khronos.org/opengl
