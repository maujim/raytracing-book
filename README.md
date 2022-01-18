# raytracer

An implementation of Peter Shirley's [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) in Rust.

## Running

```sh
cargo run --release
```

This will generate a 1920x1080 image with 100 samples and use at most 50 bounces for each ray. On my machine, with a Ryzen 7 1700 (8 cores/16 threads), this takes 231.82 seconds (3:51 min).
