# TinyRenderer-rs

A Rust learning implementation of the TinyRenderer software rendering pipeline.

Original TinyRenderer:
- https://haqr.eu/tinyrenderer/

## Current Progress

This project currently renders an OBJ model with a basic flat-shading pipeline.

Implemented:
- Generic vector math (`Vec2`, `Vec3`) with core operations
- Line rasterization (Bresenham-style)
- Triangle rasterization with barycentric coordinates
- Z-buffer based hidden surface removal (`smaller z` wins)
- OBJ parsing for `v` and triangle `f` (supports tokens like `v/vt/vn`, currently uses only vertex index)
- Flat Lambert lighting (face normal + directional light)
- Grayscale color output in RGBA (`R=G=B`)
- Unit tests for model parsing, line/triangle rasterization, and Lambert math helpers

## Current Limitations

- `main` currently writes PPM files (`output/output.ppm`, `output/depth_output.ppm`)
- Full OBJ support is not finished yet (`vt`, `vn`, non-triangle faces, negative indices, etc.)
- Shading is flat per-face only (no Gouraud/Phong interpolation yet)

## Run

```bash
cargo run
```

```bash
cargo test
```