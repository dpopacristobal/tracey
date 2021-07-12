# tracey

`tracey` is a toy ray tracer that I wrote whilst tinkering with Rust. I had two main goals in mind whilst working on this project: to learn more about programming in Rust (and specifically how Rust compares to C++) and to learn more about rendering, which was adjacent to the geometry modeling work I was doing at Autodesk at the time. The project is based on Peter Shirley's "[Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)" and its [sequels](https://raytracing.github.io/), though it includes some additional features as well.

## Sample Renders

### Classic Cornell Box Scene - 1080x1080 @ 10 rays per pixel

![scene_0](./sample_renders/scene_0.png)

### Nerdier Cornell Box Scene - 1080x1080 @ 10 rays per pixel

![scene_1](./sample_renders/scene_1.png)

### Random Spheres Scene - 1080x720 @ 10 rays per pixel

![scene_2](./sample_renders/scene_2.png)

## Usage

`tracey` can be run as a CLI app thanks to Rust's [structopt](https://github.com/TeXitoi/structopt) library. The CLI allows users to render the sample scenes shown above. Additionally, users can instead input the path to an `.obj` triangle mesh file, which will be rendered inside the Cornell Box scene; note that this will only work if the mesh lies inside the [0, 0, 0] to [555, 555, 555] cube.

Compiling and running `tracey` in this way will require users to have Rust installed. A good [tutorial](https://doc.rust-lang.org/book/ch01-01-installation.html) for installing Rust is available as part of [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html) book.

These are the options provided by the CLI:
```
tracey 0.1.0
A simple CLI to render scenes using tracey.

USAGE:
    tracey.exe [OPTIONS] --rays <rays> --width <width>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --mesh-path <mesh-path>          Path to .obj mesh that will be put into a Cornell Box scene
        --rays <rays>                    Number of rays used per pixel
        --sample-scene <sample-scene>    Sample scene number
        --width <width>                  Width of the output image in pixels
```
## Features

This is a list of the main features that I have implemented in `tracey` thus far:
- placeholder a
- placeholder b
- placeholder c

## Ideas for New Features

This is a list of ideas for new features (or tech-debt clean-up) that could be added to `tracey` (some of them are topics from Peter Shirley's books that were not implemented originally):
- placeholder a
- placeholder b
- placeholder c
