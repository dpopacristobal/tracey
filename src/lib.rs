#![allow(dead_code)]

/*
List of things to do:

- Split the render.rs file into something that makes more sense.
- Make a proper benchmark scene that combines a good number of samples per pixel.
- Clean up the modules like camera, which should be its own module.
- Start using the BVH algorithm to improve timings further.
- Check other micro-optimizations that can be done.
- Do another round of clean-up.
- Start adding extra features.
*/

extern crate image;
extern crate rand;
extern crate rayon;

pub mod camera;
pub mod hittables;
pub mod linalg;
pub mod materials;
pub mod render;
