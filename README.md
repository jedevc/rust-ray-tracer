# Rust raytracer

This is a simple and minimal ray tracer, built in a weekend, using the
raytracing book [here](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

## Running

Build and run the raytracer:

    $ cargo run > image.ppm

Then open the file using any relevant viewer. If you don't have one, you can
convert it to a png using ImageMagick:

    $ convert image.ppm image.png
