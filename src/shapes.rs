use crate::util::Point;
use crate::{HitRecord, Hittable, Material, Ray};

use std::rc::Rc;

pub struct Sphere {
    pub origin: Point,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(origin: Point, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            origin,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.origin;

        let a = ray.direction.norm_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let mut root = (-half_b - discriminant.sqrt()) / a;

        if root < t_min || t_max < root {
            root = (-half_b + discriminant.sqrt()) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - self.origin) / self.radius;

        let material = Rc::clone(&self.material);

        Some(HitRecord::new(point, &outward_normal, material, root, ray))
    }
}
