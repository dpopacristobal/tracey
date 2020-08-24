use rand::Rng;
use std::cmp::Ordering;
use std::sync::Arc;

use super::{BoundingBox, Hit, HitRecord, World};
use crate::linalg::ray::Ray;

// Could we replace this and just use generics with traits?
fn b_box_compare(hittable_a: Arc<dyn Hit>, hittable_b: Arc<dyn Hit>, axis: i32) -> Ordering {
    // TODO(dnlpc): Add some actual error-handling.
    let box_a = hittable_a.bounding_box(0.0, 0.0).unwrap();
    let box_b = hittable_b.bounding_box(0.0, 0.0).unwrap();

    box_a.min_corner()[axis as usize]
        .partial_cmp(&box_b.min_corner()[axis as usize])
        .unwrap()
}

fn b_box_compare_x(hittable_a: Arc<dyn Hit>, hittable_b: Arc<dyn Hit>) -> Ordering {
    b_box_compare(hittable_a, hittable_b, 0)
}

fn b_box_compare_y(hittable_a: Arc<dyn Hit>, hittable_b: Arc<dyn Hit>) -> Ordering {
    b_box_compare(hittable_a, hittable_b, 1)
}

fn b_box_compare_z(hittable_a: Arc<dyn Hit>, hittable_b: Arc<dyn Hit>) -> Ordering {
    b_box_compare(hittable_a, hittable_b, 2)
}

pub struct BvhNode {
    left_child_node: Arc<dyn Hit>,
    right_child_node: Arc<dyn Hit>,
    b_box: BoundingBox,
}

impl BvhNode {
    pub fn new(objects: &mut [Arc<dyn Hit>], time_0: f64, time_1: f64) -> Self {
        let mut rng = rand::thread_rng();
        let axis = rng.gen_range(0, 3);

        let comparator = match axis {
            0 => b_box_compare_x,
            1 => b_box_compare_y,
            2 => b_box_compare_z,
            _ => panic!("Non-existent axis."),
        };

        let object_num = objects.len();
        let (left_child_node, right_child_node) = match object_num {
            1 => (objects[0].clone(), objects[0].clone()),
            2 => {
                if comparator(objects[0].clone(), objects[1].clone()) == Ordering::Less {
                    (objects[0].clone(), objects[1].clone())
                } else {
                    (objects[1].clone(), objects[0].clone())
                }
            }
            _ => {
                objects.sort_unstable_by(|hittable_a, hittable_b| {
                    comparator(hittable_a.clone(), hittable_b.clone())
                });
                let mid = object_num / 2;
                let (mut slice_left, mut slice_right) = objects.split_at_mut(mid);
                (
                    Arc::new(Self::new(&mut slice_left, time_0, time_1)) as Arc<dyn Hit>,
                    Arc::new(Self::new(&mut slice_right, time_0, time_1)) as Arc<dyn Hit>,
                )
            }
        };

        let left_b_box = left_child_node.bounding_box(time_0, time_1).unwrap();
        let right_b_box = right_child_node.bounding_box(time_0, time_1).unwrap();

        let b_box = left_b_box.combine(&right_b_box);

        Self {
            left_child_node,
            right_child_node,
            b_box,
        }
    }

    pub fn from_world(world: &mut World, time_0: f64, time_1: f64) -> Self {
        Self::new(world.objects_mut().as_mut_slice(), time_0, time_1)
    }
}

impl Hit for BvhNode {
    fn hit(&self, ray: Ray, t_min: f64, mut t_max: f64) -> Option<HitRecord> {
        self.b_box.hit(ray, t_min, t_max)?;

        let hit_left_opt = self.left_child_node.hit(ray, t_min, t_max);
        if let Some(hit_left) = hit_left_opt.as_ref() {
            t_max = hit_left.t;
        }

        let hit_right_opt = self.right_child_node.hit(ray, t_min, t_max);

        if hit_right_opt.is_some() {
            hit_right_opt
        } else if hit_left_opt.is_some() {
            hit_left_opt
        } else {
            None
        }
    }

    fn bounding_box(&self, _t_min: f64, _t_max: f64) -> Option<BoundingBox> {
        Some(self.b_box.clone())
    }
}
