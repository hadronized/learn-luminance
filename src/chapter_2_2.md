# Shady triangle

[luminance] is not a framework nor a video game engine. By default, it comes with zero data set.
Hence, there is nothing _per se_ that provides _materials_ or that kind of concept. You will have
to craft them yourself and that is what we are going to do in this section.

Remember our pipeline creation from the first chapter? We needed to provide a closure taking two
arguments. It’s time to explain you what those arguments are for. Let’s take the pipeline
definition again:

```rust
surface.new_pipeline_gate().pipeline(
  &back_buffer,
  &PipelineState::default().set_clear_color(color),
  |_, _| (),
);
```

And rewrite it by using the arguments:

```rust
surface.new_pipeline_gate().pipeline(
  &back_buffer,
  &PipelineState::default().set_clear_color(color),
  |pipeline, mut shd_gate| (),
  // …
});
```

The `pipeline` argument here represents a [`Pipeline`] and `shd_gate` a [`ShadingGate`].

### The graphics pipeline

The [`Pipeline`] object you’re given represents a _graphics pipeline_. It allows you to notify
the GPU about scarce resources you’re about to use or perform specific tasks related to such
resources. That is pretty advanced so we will just ignore that object and will leave it to `_`
for now.

More on [`Pipeline`] in a future chapter.

### The shading gate

[`ShadingGate`] represents a way to _shade_ things. Shading means, like if you had a paper and color
pens, filling in shapes with colors. [luminance] works the same way. However, you have no pen and no
default algorithm to fill your triangle. You need to instruct [luminance] how to. And to do that,
you need a small digression in the world of [GLSL] and shaders.

A _shader stage_ is a piece of code that runs on a GPU. Its inputs and how frequently it will be
called heavily depend on its kind. The following table gives a better understanding.

> Here, frequency doesn’t refer to a frequency in time, but a frequency in GPU resources. You will
> get what it means below.

| Shader stage type                | Mandatory? | What it’s for                                                    | Inputs                | Running frequency                                                                                                              |
| -------------------------------- | ---------- | ---------------------------------------------------------------- | --------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| [`VertexShader`]                 | Yes.       | Transforming vertices at the beginning of the pipeline.          | Vertex attributes.    | Once for every vertices comprised in our [`Tess`].                                                                             |
| [`TessellationControlShader`]    | No.        | Determines how much a _primitive patch_ must be tessellated.     | Abstract patches.     | At least once for every _abstract patches_ flowing from the [`VertexShader`].                                                  |
| [`TessellationEvaluationShader`] | No.        | Transform tessellated patches.                                   | Abstract patches.     | At least once for every _abstract patches_ flowing from the _tessellator_ that has followed the [`TessellationControlShader`]. |
| [`GeometryShader`]               | No.        | Map, filter, add and transform _primitives_.                     | Primitive attributes. | Once for each primitive patch flowing out from either the [`VertexShader`] or [`TessellationEvaluationShader`].                |
| [`FragmentShader`]               | Yes.       | Transform _rasterized_ fragments into _render target output(s)_. | Rasterized fragment.  | Once for each fragments rasterized from the previous stages.                                                                   |

Basically, you need to provide some valid [GLSL] code for, at least, the [`VertexShader`] and the
[`FragmentShader`].

> As cool as they are, we will not dig (for this chapter) into the other types of shader stages.

Assembling _shader stages_ yields a _shader program_, which type is [`Program`]. Such an object
can then be used with our [`ShadingGate`] to shade our triangle! What we need to do here is to
write:

- A [`VertexShader`] in [GLSL] that will simply forward the vertex attributes to the next stage
  so that this information is available later.
- A [`FragmentShader`] that will read this information and output a single color for each pixel
  of the ~screen~ frame buffer.

### The vertex shader

> If you don’t know [GLSL], you’re going to have some hard times but we will try to explain
> everything.

A vertex shader runs for every vertices in your [`Tess`]. In our case, we don’t really want to do
anything useful with the vertices: we just want them to be drawn on our 2D screen. However, we need
to tell the next stage (i.e. [`FragmentShader`]) about what makes a _vertex_. That manual operation
must be written in the [`VertexShader`].

In your project, add the `src/vs.glsl` file and fill it with:

```glsl
// those are our vertex attributes
in vec2 position;
in vec3 color;

// this is the output of the vertex shader (we could have had several ones)
out vec3 v_color;

void main() {
  // simply forward the color
  v_color = color;

  // mandatory; tell the GPU to use the position vertex attribute to put the vertex in space
  gl_Position = vec4(position, 0., 1.);
}
```

### The fragment shader

A fragment shader runs for every _rasterized_ fragments from the render of your [`Tess`]. Basically,
your GPU transforms the geometry, projects it, performs several complex operations we end up with
rasterized data. Rasterized data means to discretize all the vertex attributes. The discretization
process is based on the resolution of the _render target_ — i.e. the frame buffer outputs.
Rasterized data will perform, for instance, interpolation of your vertex attributes so that each
texel (i.e. a frame buffer / texture _pixel_) has its version of the vertex attribute.

A rasterized triangle can be imagined as a collection of thousands of texels representing a
triangle. Each and everyone of them will store a color, in our case, so that it’s easy to render
that on a screen, store in an image, etc.

In your project, add the `src/fs.glsl` file with the following content:

```glsl
// this was the vertex shader output; it’s now our (rasterized and interpolated) input!
in vec3 v_color;

// we will output a single color
out vec3 frag_color;

void main() {
  // KISS
  frag_color = v_color;
}
```

In your `src/main.rs`, add the following lines:

```rust
const VS_STR: &str = include_str!("vs.glsl");
const FS_STR: &str = include_str!("fs.glsl");
```

And we’re good to go. Let’s create a [`Program`]. We’ll first need to import the type.

### The shader program

A _shader program_ is a collection of _shader stages_, connecting them to each other. It’s like
building an actual program by gluing functions to each other. [luminance] has a very opinionated
idea of what a GPU shader program is and should be. That opinion will be explained in a future
chapter. Let’s focus on the simple stuff first.

```rust
use luminance::shader::Program;
```

Then, right before your loop:

```rust
  let mut program = surface
    .new_shader_program::<VertexSemantics, (), ()>()
    .from_strings(VS_STR, None, None, FS_STR)
    .unwrap()
    .ignore_warnings();
```

As you can see, you need to provide the _vertex semantics type_ you defined earlier. That enables
[luminance] to check whether your _shader program_ is compatible with the [`Tess`] you intend to use
it with… at compile-time. Ignore the two `()`, we’ll discuss that later. However, notice the use
of the [`BuiltProgram::ignore_warnings`] method: it gives you the actual [`Program`] by ignoring any
_warnings_ that might have happened while creating the shader program. You can inspect them if you
want to but for the purpose of this example, you will not need to. Also, keep in mind that those
are only warnings that wouldn’t cause your program to behave in a weird way.

> On a general note, [luminance] is heavily type-driven. Familiarize yourself with how you can
> drive behavior with types (the [turbofish syntax], for instance, will be useful).

### The shading node

The next step is to create a new _shading node_ in your graphics pipeline. This is done via the
[`ShadingGate`].

```rust
    let render = surface.new_pipeline_gate().pipeline(
      &back_buffer,
      &PipelineState::default().set_clear_color(color),
      |_, mut shd_gate| {
        shd_gate.shade(&mut program, |_, _, mut rdr_gate| {
          // …
        });
      },
    );
```

You can see we are getting access to a new type of _gate_ here: a [`RenderGate`].

[luminance]: https://crates.io/crates/luminance
[luminance-derive]: https://crates.io/crates/luminance-derive
[`Tess`]: https://docs.rs/luminance/latest/luminance/tess/struct.Tess.html
[`Pipeline`]: https://docs.rs/luminance/latest/luminance/pipeline/struct.Pipeline.html
[`ShadingGate`]: https://docs.rs/luminance/latest/luminance/pipeline/struct.ShadingGate.html
[GLSL]: https://www.khronos.org/opengl/wiki/Core_Language_(GLSL)
[`TessellationControlShader`]: https://docs.rs/luminance/latest/luminance/shader/stage/enum.Type.html#variant.TessellationControlShader
[`TessellationEvaluationShader`]: https://docs.rs/luminance/latest/luminance/shader/stage/enum.Type.html#variant.TessellationEvaluationShader
[`VertexShader`]: https://docs.rs/luminance/latest/luminance/shader/stage/enum.Type.html#variant.VertexShader
[`GeometryShader`]: https://docs.rs/luminance/latest/luminance/shader/stage/enum.Type.html#variant.GeometryShader
[`FragmentShader`]: https://docs.rs/luminance/latest/luminance/shader/stage/enum.Type.html#variant.FragmentShader
[`Program`]: https://docs.rs/luminance/latest/luminance/shader/program/struct.Program.html
[`BuiltProgram::ignore_warnings`]: https://docs.rs/luminance/latest/luminance/shader/program/struct.BuiltProgram.html#method.ignore_warnings
[turbofish syntax]: https://doc.rust-lang.org/1.30.0/book/first-edition/generics.html
[`RenderGate`]: https://docs.rs/luminance/latest/luminance/pipeline/struct.RenderGate.html
