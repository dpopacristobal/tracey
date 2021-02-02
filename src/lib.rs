#![allow(dead_code)]

/*
List of things to do:

- Add rectangles.
- Add support for triangle meshes.
- Add a DefaultHittable.
- Replace rhs in function definitions with other.
- Isolate away the part that works on outputting an image.
- Split the render.rs file into something that makes more sense.
- Make a proper benchmark scene that combines a good number of samples per pixel.
- Clean up the modules like camera, which should be its own module.
- Start using the BVH algorithm to improve timings further.
- Check other micro-optimizations that can be done.
- Do another round of clean-up.
- Start adding extra features.
*/

extern crate image;
extern crate obj;
extern crate rand;
extern crate rayon;

pub mod camera;
pub mod hittables;
pub mod linalg;
pub mod load_mesh;
pub mod materials;
pub mod pdfs;
pub mod render;
