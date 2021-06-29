# Crate setup

First thing first: this book is written against this following luminance crate setup:

```toml
luminance = "0.44"
luminance-derive = "0.7"
luminance-front = "0.4"
luminance-glfw = "0.16"
luminance-windowing = "0.10"
```

If you are using a version from crates.io that is more recent that this current book, feel free to
ask via an [issue](https://github.com/phaazon/luminance-rs/issues) to update the book! The goal is
to keep it as updated as possible, but it might happen it lags a bit behind. Sorry about that.

Even though some other crates exist (windowing, for most), we are not going to focus on them and
will stick to the ones above.

## On version updates / migrations

It is possible that you have started reading some documentation / examples / the book about
luminance at a previous version as the one described here. In that case, you will want to
update your luminance dependencies.

Most of the time, you will be fine, but sometimes, you will not correctly update to the latest
version of luminance. This is due to the fact `cargo`, by default when invoking `cargo update`,
will try to satisfy all dependencies in your `Cargo.lock`. If a dependency is already satisfied,
it will not try to recompute it — that is the case for SemVer Ranges. The result of this is some
cryptic typing or traits errors. In such case, you need to tell `cargo` to recompute the
dependencies by taking into account the new upper bounds:

```
cargo update --aggressive
```
