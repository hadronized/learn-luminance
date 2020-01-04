# The shaders

Again, we need to define our shaders. But in our case, we are going to need to write _slightly_
more complicated shaders. See, a 3D viewer requires objects to be _projected_ onto one’s screen. In
the previous chapters, we just rendered a triangle in 2D. Here we are talking 3D. Going from a three
dimensional world to a screen (i.e. 2D) requires some operations to do.

## Projection matrix

Going from a 3D space to a 2D space always implies losing information on the fly in the process.
This is really easy to picture by taking an easier example. Imagine someone moving around Earth.
They have a position, including an altitude. Now imagine you want to _project_ their positions onto
the up axis, giving the altitude at which those persons are. One very easy way to do that is to
simply _drop_ all the other components of their positions and just retain their “up” component. For
instance, if we have a position as:

```
position = (x, y, z)
```

And we say that `y` is the component of the up-axis, we can define our projection as such:

```
project_altitude((_, y, _)) = y
```

Going from a 3D space to a screen can be done in _several_ ways. The way done above was _3D -> 1D_
but here, _3D -> 2D_ can be achieved by several ideas. For instance, an _orthogonal projection_ is
a kind of projection that preserves parallels. A _perspective projection_ is a projection that
implies distorting the vertices so that the _field of view_ is respected. You are typically used to
that kind of projection when playing a video game or watching a movie, for instance. Or just taking
a picture!

## The perspective matrix

Any way, no more theory talk. In order to use a perspective matrix and project our loaded object,
we will be using the [cgmath] crate. Many exist that can do the job but I really like the simplicity
and raw speed of [cgmath].

```toml
cgmath = "0.17"
```

We will be using several symbols from it:

```rust
use cgmath::{perspective, EuclideanSpace, Matrix4, Point3, Rad, Vector3};
```

Don’t get scared about the heavy-math symbol names. What you must know, however, is this:

- In graphics applications, we use [linear algebra] _a lot_. You don’t have to know everything by
  heart, obviously, but having a linear algebra background will highly help for sure.
- From linear algebra, we’re mostly interested into several concepts:
  - Vector spaces. The most important one. You should know how you are supposed to add vectors and
    how to scale them by a given scalar number and how to compute the sine of the angle between two
    vectors. More prerequisites will come but for today that’s enough about vector spaces.
  - Matrices. Unlike the movie, they’re not entertainment and can encode lots of other math
    concepts, among linear maps and manipulating vector spaces in a compact and powerful way. We
    use them for combining translations, scaling, rotations, [shearing], etc.
  - Quaternions. Scary name for a cute structure. Quaternions are 4-components numbers that can
    represent a lot of things. In our case, we like to use them to represent arbitrary rotations of
    φ angle (often expressed in radians) around a given unit axis (a unit axis is a 3D vector that
    has been [normalized]).

I know, I know, that’s a lot of new concepts completely unrelated to [luminance]. But you need them
and, trust me, it’s not that hard.

Back to our code now.

```rust
const FOVY: Rad<f32> = Rad(std::f32::consts::PI / 2.);
const Z_NEAR: f32 = 0.1;
const Z_FAR: f32 = 10.;

// …

// in the main_loop function, before the actual loop

let projection = perspective(
  FOVY,
  surface.width() as f32 / surface.height() as f32,
  Z_NEAR,
  Z_FAR,
);
```

This defines a projection matrix with a _field of view_ set to _π ÷ 2_ (wich represents a field of
view of 90°), with an _aspect ratio_ defines by the division between the width and height of the
framebuffer we’re rendering into and with two special parameters, `Z_NEAR` and `Z_FAR`. Those are
_clipping_ parameters defining a _frustrum_ object. Everything outside the frustrum won’t be
visible when asking to render.

![](https://i0.wp.com/www.lighthouse3d.com/wp-content/uploads/2011/04/vf.gif?w=405)

> Image taken from [here](http://www.lighthouse3d.com/tutorials/view-frustum-culling).

## The view matrix

A single projection matrix _projects_ 3D coordinates on 2D coordinates with a given perspective but
the 3D coordinate system is still left unchanged. What it means is that projecting only will place
you in a situation where you’re still at the origin (i.e. `(0, 0, 0)`) looking in the _forward_
direction. What the forward vector is depends on which canonical system you decide to use, but in
our case, it’s a [right-handed system]. The _X_ unit axis goes from left to right of your screen;
the _Y_ unit axis goes from down to up your screen and the _Z_ axis (also called _depth_) goes from
_inside your screen_ towards your face.

Because we will just place the loaded object at the origin, we will want to slightly offset our
position and view direction so that we can actually see the object. Again, [cgmath] provides us with
the right _transformation matrix_ for this.

```rust
let view = Matrix4::<f32>::look_at(Point3::new(2., 2., 2.), Point3::origin(), Vector3::unit_y());
```

We will then be at the 3D point `(2, 2, 2)` and will look at the origin, the _Y_ unit axis being
considered the _up_ axis.

## Declaring the matrices in the shader

Next step is to actually use those matrices. In order to do so, we need to do two things:

- Use the matrices in the shader code.
- Tell [luminance] about the existence of those matrices in the shaders so that we can update them.

### GLSL matrices

Let’s start from the _vertex shader_ we used in the previous chapter.

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

In our case, we don’t have vertex colors anymore and we will likely just set a constant color to
start with. Plus, now the point is in 3D, not 2D.

```glsl
in vec3 position; // 3D point!

void main() {
  gl_Position = vec4(position, 1.);
}
```

The problem is: it’s easy for the GPU to render 2D coordinates because there’s nothing to project:
your screen is already using a 2D space coordinate system. However, how should we handle our 3D
vertices?

You already have the answer: the _projection_ matrix. That matrix will just turn your 3D vertices
into 2D vertices. In order to use it, you need to declare a _shader uniform variable_. Uniforms are
special variables which values are set by the application before or after a render command. When
a render command is issued, it is not possible to change the value anymore until the render command
has finished. You can picture those uniform values as _constants over a draw call_. Some graphics
APIs call those _constant buffers_ for that reason.

Let’s add one we’ll call `projection`. You declare them at global scope, just next to `in` and
`out` declarations. They can be declared in any shader stage that needs to manipulate them.

```glsl
uniform mat4 projection;
```

It’s as simple as that. In our case, `projection` is a 4×4 matrix that will be supplied by the
application.

In order to be projected, a point must be multiplied by a matrix. Let’s just project our 3D point
then:

```glsl
gl_Position = projection * vec4(position, 1.);
```

All of this might be a bit weird or awkward at first because it’s highly linked to how vector spaces
and linear algebra work but you’ll have to trust me on this (or read some linear algebra theory!).

Now let’s add the _view_ matrix, allowing us to slightly offset the camera in the scene:

```glsl
uniform mat4 view;
```

Simple, right?

```glsl
gl_Position = projection * view * vec4(position, 1.);
```

So, some explications here. We multiply `view` and `position` first because `view` is a matrix that
_transforms_ a 3D point into another 3D point. You can picture that as a _basis transformation_.
Then we want to project that transformed point, so we multiply by the `projection` at the very left.

That’s all for the GLSL code. Nothing else to do: our 3D point is now projected onto our screen by
having gone through two basis change:

- A _3D -> 3D_ linear transformation to _move the camera around_.
- A _3D -> 2D_ projection to introduce perspective.

### The uniform interface

In order for your application to actually update and send those matrices to the GPU, you need to
declare them and change the way the shader [`Program`] works. A special concept must be used here:
[uniform interfaces].

A uniform interface is a typed contract between the GLSL code compiled and linked in a shader
[`Program`] and what you are supposed to with it. This powerful abstraction allows you to specify,
via a type, a set of _variables_ that are available in the GLSL code and that you can get access to
as soon as your [`Program`] gets shading things. The way you do this is a multi-step yet simple
process:

1. You define a `struct` that holds the GPU variables. Those are called [`Uniform`].
2. You implement the [`UniformInterface`] trait for that type. If you don’t want to get too much
  into the details of that trait, you can use [luminance-derive] to implement the trait very easily.
  That’s what we are going to do.
3. You set your [`Program`] type variable setting its _uniform interface_ to the type you just
  created.
4. When you use a [`ShadingGate`] to shade objects, you have access to your _uniform interface_ and
  can then update GPU variables there.

Why cannot you create and handle the uniform interface by yourself? The thing is: that concept
_must_ be [contravariant], because allowing you to handle such objects around while they depend on
the current GPU context and state would be highly unsafe.

Let’s create such a type.

```rust
#[derive(Debug, UniformInterface)]
struct ShaderInterface {
  #[uniform(unbound)]
  projection: Uniform<M44>,
  #[uniform(unbound)]
  view: Uniform<M44>,
}
```

Sooo… as you can see, we define a regular `struct` but _derive_ `UniformInterface`. Deriving the
trait unlocks several annotations you can use via the `#[uniform(..)]` syntax. Here, we’ll talk
about two:

- `#[uniform(unbound)]`: that annotation tells luminance that **if** the GPU variable this uniform
  variable refers to is _inactive_ or _inexistent_, no error will be generated. Instead, a special
  _unbound_ uniform variable will be emitted. So an _unbound_ `Uniform<M44>` means that if the GPU
  variable named after that uniform is _inactive_ or _inexistent_, the resulting `Uniform<M44>` will
  silently do nothing when you will try to update it. This is a feature you want when you’re
  developing or debugging but you should disable that on an end-user application or if you don’t
  care about errors for a given variable.
- `#[uniform(name = "foo")]`: rename the field. By default, the GLSL uniform variable will match the
  name of the `struct` field you define. You can change the mapping with that simple annotation.
- You can of course mix annotations; e.g. `#[uniform(name = "t", unbound)]` is an _unbound_ uniform
  variable mapped to a `uniform float t` in the GLSL code.

All we have to do now is to change the type of our [`Program`] to use the uniform interface and
we’re done.

```rust
let program: Program<VertexSemantics, (), ShaderInterface> =
  Program::from_strings(None, VS_STR, None, FS_STR)
    .unwrap()
    .ignore_warnings();
```

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
