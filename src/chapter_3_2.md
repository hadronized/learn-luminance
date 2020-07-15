# Defining our vertex type

The first thing to do is to define the kind of geometry our renderer will support. In the first
place, we are not interested into supporting texturing nor even lighting, just raw geometry. We
will then only need the position.

```rust
use luminance_derive::{Semantics, Vertex};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Semantics)]
pub enum VertexSemantics {
  #[sem(name = "position", repr = "[f32; 3]", wrapper = "VertexPosition")]
  Position,
}

#[derive(Clone, Copy, Debug, Vertex)]
#[vertex(sem = "VertexSemantics")]
struct Vertex {
  position: VertexPosition,
}
```

> Notice that we now have a 3D point instead of a 2D point.

You should be familiar with that code by now. We will just add this for future use:

```rust
type VertexIndex = u32;
```

> [luminance] supports several kinds of indices. `u32` is more than enough for our current use case
> but you never know.

[luminance]: https://crates.io/crates/luminance
