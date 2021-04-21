# What is a triangle?

Everyone knows what a triangle is… but what is a triangle on your GPU? In [luminance], a triangle
can be _represented_ in lots of ways. In our case:

- A triangle has three vertices.
- We decide that each vertex has a position in **2D**, represented by two floating point values on
  32-bit.
- Each vertex has a color, represented as RGB on 8-bit unsigned integers.

The first thing to do is to create… types. You will see throughout this book that [luminance] is
heavily type-oriented. Don’t freak out. [luminance] requires you to define your type in a way it can
acknowledge how vertices’ data are formed. The following, for instance, will not work:

```rust
type Position = (f32, f32);
type RGB = (u8, u8, u8);
type Vertex = (Position, RGB);
```

But the real vertex definition is very, very similar. Hang on.

## Defining your vertex type

In order to define our vertex type, we need to create a `struct` that will implement the [`Vertex`]
trait. That trait requires various information to be provided by the implementor. You don’t have
to worry about those, because a crate exists to automatically implement such a trait:
[luminance-derive].

First thing first: add [luminance-derive] to your project’s `[dependencies]` section:

```toml
luminance-derive = "0.6"
```

Simple. One last thing: when you will use the `Vertex` derive annotation, you will have to provide
a _vertex semantics_ type, implementing the [`Semantics`] trait. Again, you don’t have to implement
such a trait by hand: [luminance-derive] will handle all that for you.

Vertex semantics are a way to tell [luminance] what the relationship between _all_ objects you
intend to create and the way they will be rendered is. In our case, we just only need two
semantics: _vertex positions_ and _vertex colors_. Let’s create our semantics type by using a
proc-macro derive from [luminance-derive]:

```rust
use luminance_derive::Semantics;
```

Our proc-macro derive, yay. Let’s use it:

```rust
#[derive(Copy, Clone, Debug, Semantics)]
pub enum VertexSemantics {
  #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
  Position,
  #[sem(name = "color", repr = "[u8; 3]", wrapper = "VertexRGB")]
  Color,
}
```

> Woah, that’s a lot of new syntax!

Indeed, let’s dig the syntax:

- The `Semantics` derive annotation marks an `enum` as being representative of _vertex semantics_.
  Currently, nothing else than `enum` is supported.
- The type implementing `Semantics` must also implement [`Copy`], [`Clone`] and [`Debug`].
- Each variant of the `enum` represents a distinct _vertex semantics_.
- The syntax `#[sem(..)]` provides several mandatory information:
  - The `name = "position"` annotation gives [luminance] a way to recognize the semantics in
    _shader stages_. You don’t have to worry about what it means so far but keep in mind that
    that identifier must be unique.
  - `repr = "[f32; 2]"` tells which is the underlying expected type of the semantics. That
    constructs a strongly-typed assumption about the semantics. A `"position"` is a 2D `f32`.
    Period. You have a large list of types you can use here but you are limited to the implementors
    of [`VertexAttrib`].
  - The `wrapper = "VertexPosition"` annotation generates a new type called `VertexPosition` and
    in scope in the module you declared the `enum`. That type is one of the only ones which are
    recognized as being usable with the `Semantics` `enum` you just declared — it implements the
    [`HasSemantics`] trait for which `HasSemantics::Sem = VertexSemantics`. This type is also
    equipped with some functions and implementors, such as `new`, `From / Into`, etc.

All of this might be a bit confusing; let’s clarify even further. When you declare an `enum`
annotated with `#[derive(Copy, Clone, Debug, Semantics)]`, [luminance-derive] does automatically
implement [`Semantics`] for you and generates as many types as variants in your `enum`. Those types
represent _vertex attributes_ types you will be able to use to construct types that will correctly
implement the [`Vertex`] trait.

Talking about [`Vertex`], let’s go and define our vertex type. The `Vertex` derive annotation works
on both `struct`s and tuple-`struct`. Import the `Vertex` proc-macro derive first:

```rust
use luminance_derive::{Semantics, Vertex};
```

And then define your `Vertex` type (it’s possible to use the same typename because proc-macro won’t
clash with types):

```rust
#[derive(Vertex)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
  #[allow(dead_code)]
  position: VertexPosition,

  #[allow(dead_code)]
  #[vertex(normalized = "true")]
  color: VertexRGB,
}
```

A new syntax! So:

- The `Vertex` derive annotation marks a `struct` as being a valid _vertex_ type. Currently, only
  `struct` with fields and tuple-`struct` are supported.
- The `#[vertex(sem = "VertexSemantics")]` provides a mapping to a type that represents your
  _vertex semantics_.
- Each field must have a type that implements `HasSemantics<Sem = VertexSemantics>` in that case.
  Don’t forget about the `wrapper` types that got generated with the `Semantics` derive: those
  types are valid as fields’ types here.
- The special `#[vertex(normalized = "true")]` annotation marks a field as being _normalized_.
  Normalized fields make sense when the field is of an integral type, such as `[u8; 3]`, which is
  _unsigned integral_. When trying to fetch normalized vertex attributes, a _vertex stage_ will get
  normalized floating point numbers (lying in `[0.; 1.]`) instead of the typical e.g. `[0; 255]`.

And we are good to go as our vertex type is now live!

> Some notes: the generated wrapper types have some useful methods and implementors. You can easily
> get a list by running `cargo doc --open`.

## Defining a triangle

A triangle is just three points — three vertices. Let’s define them.

```rust
const VERTICES: [Vertex; 3] = [
  Vertex::new(
    VertexPosition::new([-0.5, -0.5]),
    VertexRGB::new([255, 0, 0]),
  ),
  Vertex::new(
    VertexPosition::new([0.5, -0.5]),
    VertexRGB::new([0, 255, 0]),
  ),
  Vertex::new(
    VertexPosition::new([0., 0.5]),
    VertexRGB::new([0, 0, 255])
  ),
];
```

It’s that simple.

## The final part of the recipe: GPU tessellations

> Tessellations?

In [luminance], everything that has a _vertex_ or that _must be rendered_ is done via a
_tessellation_ as described by the [`Tess`] type. GPU tessellations provide information about:

- The nature of the topology of the underlying _vertex mesh_. That is, zero, one or several
  buffers describing the raw topology of the mesh.
- The way vertices are linked to each other. That is done via several ways that are going to be
  explored in this book, but so far, we’ll stick to _primitive modes_, encoded via the [`Mode`]
  type.
- And a lot of cool features you should be impatient to discover, but everything happens to those
  who wait. ;)

Creating a [`Tess`] will upload our vertices to the GPU so that we have an object (i.e. [`Tess`]) to
manipulate and render our triangle. However, one does not simply create a [`Tess`]: we need
[`TessBuilder`], which follows the [builder pattern].

![](./imgs/one-does-not-simply-create-tess-without-builder.jpg)

Let’s see the code to create our [`Tess`] via [`TessBuilder`].

```rust
use luminance::tess::Mode;
```

Then:

```rust
  // at the beginning of main_loop
  let triangle = ctxt
    .new_tess()
    .set_vertices(&VERTICES[..])
    .set_mode(Mode::Triangle)
    .build()
    .unwrap();
```

If you don’t specify the [`Mode`], the [`TessBuilder`] defaults to [`Mode::Point`], which will not
connect your vertices between them and will leave three independent points on your screen. Also,
in a production application, you shouldn’t use `.unwrap()` but instead propagate the error or
treat it.

We have everything we need to represent our triangle on GPU with [luminance]. Let’s go on and see
how we can render it into our frame buffer.

[luminance]: https://crates.io/crates/luminance
[luminance-derive]: https://crates.io/crates/luminance-derive
[`Vertex`]: https://docs.rs/luminance/latest/luminance/vertex/trait.Vertex.html
[`Semantics`]: https://docs.rs/luminance/latest/luminance/vertex/trait.Semantics.html
[`Copy`]: https://doc.rust-lang.org/std/marker/trait.Copy.html
[`Clone`]: https://doc.rust-lang.org/std/clone/trait.Clone.html
[`Debug`]: https://doc.rust-lang.org/std/fmt/trait.Debug.html
[`VertexAttrib`]: https://docs.rs/luminance/latest/luminance/vertex/trait.VertexAttrib.html
[`HasSemantics`]: https://docs.rs/luminance/latest/luminance/vertex/trait.HasSemantics.html
[`Tess`]: https://docs.rs/luminance/latest/luminance/tess/struct.Tess.html
[`TessBuilder`]: https://docs.rs/luminance/latest/luminance/tess/struct.TessBuilder.html
[`Mode`]: https://docs.rs/luminance/latest/luminance/tess/enum.Mode.html
[`Mode::Point`]: https://docs.rs/luminance/latest/luminance/tess/enum.Mode.html#variant.Point
