# The Wavefront .obj format

> _A what?!_

The [Wavefront .obj] file format is a format that describes 3D data in a very simple manner. It is
a text format that has been around the graphics and 3D industry for a while now. Open and widely
adopted by the graphics community, it’s often criticized for its lack of modern features.
Nevertheless, it is a very popular format, simple to parse and contains enough information for a lot
of applications. Applications such as Z-Brush, Blender or 3Ds Max have full support for such a
format so it’s a perfect match for our community concerns.

Because we do not care about the actual format definition nor implementing a parser, we will use the
[wavefront_obj] crate for that purpose:

```toml
wavefront_obj = "10"
```

That format is basically divided into several parts (non-exhaustive):

- Materials definitions. We are currently not interested by those and we will just ignore them.
- Vertex definitions. Vertices are spread out by attributes and each vertex attribute is declared
  on one single line. Currently, we are interested in:
  - 3D positions, with an optional fourth argument.
  - UV mapping coordinates. Those are used for texturing. More on that later.
  - Normals. Normals are very important to perform lighting computations, vertex displacement, etc.
- Face definitions. A face is just a list of numbers indexing the previously declared vertices to
  form either _triangles_ or _quads_.
- Object definitions. Faces and vertices can be gathered in named objects, but we do not care about
  that so far.

So what we will want to do here is to load a `.obj` object and actually display it with
[luminance].

[luminance]: https://crates.io/crates/luminance
[Wavefront .obj]: https://en.wikipedia.org/wiki/Wavefront_.obj_file
[wavefront_obj]: https://crates.io/crates/wavefront_obj
