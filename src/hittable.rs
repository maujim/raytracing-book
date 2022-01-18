use crate::util::{Point, Vector};
use crate::{Material, Ray};
use std::marker::{Send, Sync};
use std::sync::Arc;

pub struct HitRecord {
    pub point: Point,
    pub normal: Vector,
    pub material: Arc<dyn Material>,
    t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        point: Point,
        outward_normal: &Vector,
        material: Arc<dyn Material>,
        t: f64,
        ray: &Ray,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;

        let mut normal = *outward_normal;
        if !front_face {
            normal *= -1.0;
        };

        Self {
            point,
            normal,
            material,
            t,
            front_face,
        }
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    items: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
        }
    }

    pub fn add(&mut self, item: Arc<dyn Hittable>) {
        self.items.push(item);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut result: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for item in &self.items {
            if let Some(hit_record) = item.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                result = Some(hit_record);
            }
        }

        result
    }
}
