use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

use obj::{load_obj, Obj};

use crate::hittables::{BvhNode, Triangle, World};
use crate::linalg::Point3;
use crate::materials::Material;

pub fn load_mesh(mesh_path: &Path, material: Arc<dyn Material>) -> BvhNode {
    let input =
        BufReader::new(File::open(mesh_path).expect("Path to specified .obj file is invalid"));
    let model: Obj = load_obj(input).expect("Failed to parse specified .obj file");

    let mut hittable_list = World::default();

    let tri_num = model.indices.len() / 3;
    for i in 0..tri_num {
        let mut tri_verts = [Point3::default(), Point3::default(), Point3::default()];
        for j in 0..3 {
            let index: usize = model.indices[3 * i + j].into();
            let model_vert = model.vertices[index];
            tri_verts[j] = Point3::new(
                model_vert.position[0].into(),
                model_vert.position[1].into(),
                model_vert.position[2].into(),
            );
        }
        hittable_list.add(Arc::new(Triangle::new(tri_verts, material.clone())));
    }

    BvhNode::from_world(&mut hittable_list, 0.0, 1.0)
}
