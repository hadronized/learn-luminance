# Altering the graphics pipeline

Now that we have a shader program that can accept updating our two matrices, we need to actually
pass the matrices down to the GPU so that it can use them when transforming points in the _vertex
shader_. That is done via the [`ShadingGate`]. When you call the [`ShadingGate::shade`] method,
you must pass a closure that will receive three arguments:

- A [`ProgramInterface`], that gives you access to an interface to update uniforms.
- Your uniform interface.
- A [`RenderGate`], as seen in previous chapters.

The [`ProgramInterface`] argument allows you to directly manipulate the fields in your
`ShaderInterface` — here, `projection` and `view` that you get as second argument. It also allows
other operations on the [`Program`]’s _uniforms_ but it’s currently off topic — feel free to read
its documentation if you’re interested, though.

```rust
    let render = ctxt
      .new_pipeline_gate()
      .pipeline(
        &back_buffer,
        &PipelineState::default().set_clear_color(color),
        |_, mut shd_gate| {
          shd_gate.shade(&mut program, |mut iface, uni, mut rdr_gate| {
            iface.set(&uni.projection, projection.into());
            iface.set(&uni.view, view.into());

            rdr_gate.render(&RenderState::default(), |mut tess_gate| {
              tess_gate.render(&mesh)
            })
          })
        },
      )
      .assume();
```

It’s as simple as that.

> _Why the `.into()` call_?

Because of [cgmath]’s type that must be converted to the right array type of our 4×4 matrix.

If you compile and run with the following
[suzanne.obj](https://phaazon.net/media/uploads/suzanne.obj) file, you should see this:

![](imgs/suzanne_flat.png)

As you can see, we can guess the monkey silhouette but we don’t actually see any details. In order
to fix that, we will need several things:

- Vertex normals. Those are used to make light rays _bounce_ over the surface of the object and then
  compute angles to determine illumination.
- At least one light, to actually “see” the object.

[`ShadingGate`]: https://docs.rs/luminance/latest/luminance/pipeline/struct.ShadingGate.html
[`ShadingGate::shade`]: https://docs.rs/luminance/latest/luminance/pipeline/struct.ShadingGate.html#method.shade
[`Program`]: https://docs.rs/luminance/latest/luminance/shader/program/struct.Program.html
[`RenderGate`]: https://docs.rs/luminance/latest/luminance/pipeline/struct.RenderGate.html
[cgmath]: https://crates.io/crates/cgmath
[`ProgramInterface`]: https://docs.rs/luminance/latest/luminance/shader/program/struct.ProgramInterface.html
