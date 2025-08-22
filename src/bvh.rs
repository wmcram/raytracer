use crate::aabb::AABB;
use crate::hit::{Hit, Hittables};
use crate::interval::Interval;
use crate::utils::random_range_int;
use std::cmp::Ordering;
use std::sync::Arc;

pub struct BVHNode {
    left: Arc<dyn Hit>,
    right: Arc<dyn Hit>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(objects: &mut Vec<Arc<dyn Hit>>, start: usize, end: usize) -> Self {
        let mut bbox = AABB::EMPTY;
        for i in start..end {
            bbox = AABB::from((bbox, objects[i].bounding_box()));
        }
        let axis = bbox.longest_axis();
        let comparator = |a: &Arc<dyn Hit>, b: &Arc<dyn Hit>| BVHNode::box_compare(a, b, axis);

        let object_span = end - start;

        let left: Arc<dyn Hit>;
        let right: Arc<dyn Hit>;
        match object_span {
            1 => {
                left = objects[start].clone();
                right = objects[start].clone();
            }
            2 => {
                left = objects[start].clone();
                right = objects[start + 1].clone();
            }
            _ => {
                objects[start..end].sort_by(comparator);
                let mid = start + object_span / 2;
                left = Arc::new(BVHNode::new(objects, start, mid));
                right = Arc::new(BVHNode::new(objects, mid, end));
            }
        }

        Self {
            left: left.clone(),
            right: right.clone(),
            bbox,
        }
    }

    fn box_compare(a: &Arc<dyn Hit>, b: &Arc<dyn Hit>, axis_index: usize) -> Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = b.bounding_box().axis_interval(axis_index);
        if a_axis_interval.min < b_axis_interval.min {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl From<Hittables> for BVHNode {
    fn from(mut value: Hittables) -> Self {
        let size = value.objects.len();
        Self::new(&mut value.objects, 0, size)
    }
}

impl Hit for BVHNode {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
        rec: &mut crate::hit::HitRecord,
    ) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }
        let hit_left = self.left.hit(r, ray_t, rec);
        let hit_right = self.right.hit(
            r,
            Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max }),
            rec,
        );
        return hit_left || hit_right;
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
