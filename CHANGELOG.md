# CHANGELOG

## future release

- `static_path` was removed.
  `cargo-geng` now copies `assets` folder instead of `static`, placing it next to executable
  (before contents of `static` folder were placed near executable)
  New intended usage is `run_dir().join("assets")` instead of old `static_dir()`
- `AABB` renamed to `Aabb2`
- `Vec2`/3/4 renamed to `vec2`, its now a lowercase type same as in GLSL.
  It is also now a tuple struct, which means you can construct it with `let v = vec2(x, y)`
  as well as pattern match with `let vec2(x, y) = v`.
  They also implement `Deref`/`DerefMut` into types with `x`/`y`/`z`/`w` fields so you can still use `v.x` etc
- batbox now supposedly fully documented

## 0.10.0

- `use batbox::prelude::*` instead of `use batbox::*`
