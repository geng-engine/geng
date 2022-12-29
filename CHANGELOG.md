# CHANGELOG

## future release

- `static_path` was removed.
  `cargo-geng` now copies `assets` folder instead of `static`, placing it next to executable
  (before contents of `static` folder were placed near executable)
  New intended usage is `run_dir().join("assets")` instead of old `static_dir()`

## 0.10.0

- `use batbox::prelude::*` instead of `use batbox::*`
