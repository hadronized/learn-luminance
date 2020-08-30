# Loading an .obj object

This is not really the purpose of this book but you actually need the following code in order to get the rest.
So here it is. Refer to the [wavefront_obj] crate for further details. The idea here is that we define our
own `Obj` type with our own representation of what an object is. We then use [wavefront_obj] to
load one object and convert it to our representation. Simple.

> Note: we also use the [try-guard] crate to convert boolean expressions to _try_ values.

However, before going any further, we are going to need to introduce a new crate: [luminance-front].

## luminance-front

The examples you saw so far are simple enough they don’t require you to explicitly put type
ascriptions about the scarce resources you use. However, as soon as you need to pass them around,
you are going to hit a problem.

[luminance] — the crate — is a backend agnostic graphics API. It means that it doesn’t know which
backend it will be executed with. That information is selected by a _platform_ crate — in our case,
[luminance-glfw], which in its turn depends on a _backend crate_ — [luminance-gl] here. A type
such as [`Tess`] gets its first type variable, `B`, replaced by the backend type. You didn’t notice
that because the examples from the previous chapters didn’t require passing such types around, but
now we are going to need to annotate functions’ signatures with [luminance] types, and then, with
backend types.

You have three options:

1. You can decide to bring in [luminance-gl] in your `Cargo.toml` and use the backend type that is
  exported from there — a.k.a. `GL33` in our case. This will allow you to use it where backend
  types are expected, but this will prevent you from using anything else but OpenGL 3.3. It ties
  your code to a backend implementation.
2. You could continue declaring and passing the `B` type variable. That is a sane option if you are
  writing a luminance crate or a middleware library, but it’s going to lead to not very comfortable
  types to read. For instance, if you want to be able to call the `TessBuilder::set_vertices`
  method, the list of constaints, types and bounds you need to add (from
  `luminance::backend::tess`) is likely to discourage you.
3. You could use [luminance-front]. The goal of this crate is to select a backend type at
  compile-time and provide type aliases to remove the need to annotate functions and types with
  backend types. This is a much more comfortable situation and it scales / adapts to the
  compilation targets and feature gates you set in your `Cargo.toml`.

We are going to use [luminance-front] to demonstrate how easy the crate is. Basically, it will
re-export all the types from [luminance], replacing the `B` type variables (where it appears) by
the right backend type. The backend type, found at `luminance_front::Backend`, can be used if you
still need to constrain some code (typical with `C: GraphicsContext`).

Also, in order to make things more coherent and convenient, [luminance-front] re-exports symbols
which don’t have the `B` type variables (it simply forwards them), so that you can remove
`luminance` from your `use` statements and make them more uniform.

> Note: keep in mind that if you use [luminance-derive], as it depends on [luminance], you will
> still need to have to include [luminance] in your `Cargo.toml`.

Add this to your `Cargo.toml`:

```toml
luminance-front = "0.3"
```

## Loading Wavefront objects

Loading a .obj is not really part of this book, but we’ll provide the code so that you don’t have
to struggle too much. With [luminance-front], you will notice that the platform crate is still up
to us to decide. So if we want to be able to work for any platform crates, we need to constrain
the platform with the backend type [luminance-front] would have selected for us. We can do that
with `luminance_front::Backend`. If `C` is the type of the platform (in our case it’s `GlfwSurface`
but it’s a good practice to be able to adapt to any), then the following is required to perform
[luminance] operations:

```rust
where C: GraphicsContext<Backend = Backend>
```

Then, loading is just a matter of following [wavefront_obj]’s API:

```rust
use luminance_front::context::GraphicsContext;
use luminance_front::tess::{Tess, TessError};
use luminance_front::Backend;
use std::fs::File;
use std::io::Read as _;
use std::path::Path;
use try_guard::verify;
use wavefront_obj::obj;

// …

#[derive(Debug)]
struct Obj {
  vertices: Vec<Vertex>,
  indices: Vec<VertexIndex>,
}

impl Obj {
  fn to_tess<C>(
    self,
    surface: &mut C,
  ) -> Result<Tess<Vertex, VertexIndex, (), Interleaved>, TessError>
  where
    C: GraphicsContext<Backend = Backend>,
  {
    surface
      .new_tess()
      .set_mode(Mode::Triangle)
      .set_vertices(self.vertices)
      .set_indices(self.indices)
      .build()
  }

  fn load<P>(path: P) -> Result<Self, String>
  where
    P: AsRef<Path>,
  {
    let file_content = {
      let mut file = File::open(path).map_err(|e| format!("cannot open file: {}", e))?;
      let mut content = String::new();
      file.read_to_string(&mut content).unwrap();
      content
    };
    let obj_set = obj::parse(file_content).map_err(|e| format!("cannot parse: {:?}", e))?;
    let objects = obj_set.objects;

    verify!(objects.len() == 1).ok_or("expecting a single object".to_owned())?;

    let object = objects.into_iter().next().unwrap();

    verify!(object.geometry.len() == 1).ok_or("expecting a single geometry".to_owned())?;

    let geometry = object.geometry.into_iter().next().unwrap();

    println!("loading {}", object.name);
    println!("{} vertices", object.vertices.len());
    println!("{} shapes", geometry.shapes.len());

    // build up vertices; for this to work, we remove duplicated vertices by putting them in a
    // map associating the vertex with its ID
    let mut vertex_cache: HashMap<obj::VTNIndex, VertexIndex> = HashMap::new();
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut indices: Vec<VertexIndex> = Vec::new();

    for shape in geometry.shapes {
      if let obj::Primitive::Triangle(a, b, c) = shape.primitive {
        for key in &[a, b, c] {
          if let Some(vertex_index) = vertex_cache.get(key) {
            indices.push(*vertex_index);
          } else {
            let p = object.vertices[key.0];
            let position = VertexPosition::new([p.x as f32, p.y as f32, p.z as f32]);
            let vertex = Vertex { position };
            let vertex_index = vertices.len() as VertexIndex;

            vertex_cache.insert(*key, vertex_index);
            vertices.push(vertex);
            indices.push(vertex_index);
          }
        }
      } else {
        return Err("unsupported non-triangle shape".to_owned());
      }
    }

    Ok(Obj { vertices, indices })
  }
}
```

Basically, calling `Obj::load(path)` here will get us a `Result<Obj, String>`. We can then just
convert it to a [`Tess`] for [luminance] to process.

[luminance]: https://crates.io/crates/luminance
[luminance-front]: https://crates.io/crates/luminance-front
[luminance-gl]: https://crates.io/crates/luminance-gl
[luminance-glfw]: https://crates.io/crates/luminance-glfw
[`Tess`]: https://docs.rs/luminance/latest/luminance/tess/struct.Tess.html
[wavefront_obj]: https://crates.io/crates/wavefront_obj
[try-guard]: https://crates.io/crates/try-guard
