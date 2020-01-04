# Loading an .obj object

This is not really the purpose of this book but you actually need the following code in order to get the rest.
So here it is. Refer to the [wavefront_obj] crate for further details. The idea here is that we define our
own `Obj` type with our own representation of what an object is. We then use [wavefront_obj] to
load one object and convert it to our representation. Simple.

All the code you read so far introduced new concepts from [wavefront_obj] that will not be explained
because this is not the right place in this current book. Nevertheless, no new concept was introduced
regarding [luminance].

> Note: we also use the [try-guard] crate to convert boolean expression to _try_ values.

```rust
use std::fs::File;
use std::io::Read as _;
use std::path::Path;
use luminance::tess::{Tess, TessBuilder, TessError};
use luminance::context::GraphicsContext;
use try_guard::verify;
use wavefront_obj::obj;

// …

#[derive(Debug)]
struct Obj {
  vertices: Vec<Vertex>,
  indices: Vec<VertexIndex>,
}

impl Obj {
  fn to_tess<C>(self, ctx: &mut C) -> Result<Tess, TessError>
  where
    C: GraphicsContext,
  {
    TessBuilder::new(ctx)
      .set_mode(Mode::Triangle)
      .add_vertices(self.vertices)
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
[luminance-derive]: https://crates.io/crates/luminance-derive
[`Vertex`]: https://docs.rs/luminance/latest/luminance/vertex/trait.Vertex.html
[`Semantics`]: https://docs.rs/luminance/latest/luminance/vertex/trait.Semantics.html
[`Tess`]: https://docs.rs/luminance/latest/luminance/tess/struct.Tess.html
[`TessBuilder`]: https://docs.rs/luminance/latest/luminance/tess/struct.TessBuilder.html
[`Mode`]: https://docs.rs/luminance/latest/luminance/tess/enum.Mode.html
[`Pipeline`]: https://docs.rs/luminance/latest/luminance/pipeline/struct.Pipeline.html
[`ShadingGate`]: https://docs.rs/luminance/latest/luminance/pipeline/struct.ShadingGate.html
[`ShadingGate::shade`]: https://docs.rs/luminance/latest/luminance/pipeline/struct.ShadingGate.html#method.shade
[`VertexShader`]: https://docs.rs/luminance/latest/luminance/shader/stage/enum.Type.html#variant.VertexShader
[`FragmentShader`]: https://docs.rs/luminance/latest/luminance/shader/stage/enum.Type.html#variant.FragmentShader
[`Program`]: https://docs.rs/luminance/latest/luminance/shader/program/struct.Program.html
[`RenderGate`]: https://docs.rs/luminance/latest/luminance/pipeline/struct.RenderGate.html
[`TessGate`]: https://docs.rs/luminance/latest/luminance/pipeline/struct.TessGate.html
[Wavefront .obj]: https://en.wikipedia.org/wiki/Wavefront_.obj_file
[wavefront_obj]: https://crates.io/crates/wavefront_obj
[cgmath]: https://crates.io/crates/cgmath
[linear algebra]: https://en.wikipedia.org/wiki/Linear_algebra
[shearing]: https://en.wikipedia.org/wiki/Shear_matrix
[normalized]: http://mathworld.wolfram.com/NormalizedVector.html
[right-handed system]: https://en.wikipedia.org/wiki/Right-hand_rule
[uniform interfaces]: https://docs.rs/luminance/latest/luminance/shader/program/trait.UniformInterface.html
[`Uniform`]: https://docs.rs/luminance/latest/luminance/shader/program/struct.Uniform.html
[`Uniform::update`]: https://docs.rs/luminance/latest/luminance/shader/program/struct.Uniform.html#method.update
[`UniformInterface`]: https://docs.rs/luminance/latest/luminance/shader/program/trait.UniformInterface.html
[contravariant]: https://en.wikipedia.org/wiki/Functor#Covariance_and_contravariance
[`ProgramInterface`]: https://docs.rs/luminance/latest/luminance/shader/program/struct.ProgramInterface.html
[`M44`]: https://docs.rs/luminance/latest/luminance/linear/type.M44.html
[Phong]: https://en.wikipedia.org/wiki/Phong_shading
[try-guard]: https://crates.io/crates/try-guard
