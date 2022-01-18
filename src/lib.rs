#![warn(clippy::all)]

mod util;
mod hittable;
mod material;
mod ray;
mod shapes;

pub use crate::camera::Camera;
pub use crate::hittable::{HitRecord, Hittable, HittableList};
pub use crate::material::Material;
pub use crate::material::{Dielectric, Lambertian, Metal};
pub use crate::ray::Ray;
pub use crate::shapes::Sphere;
pub use crate::util::*;

use rand::distributions::Uniform;

fn random_point_in_unit_sphere() -> Point {
    let distribution = Uniform::new(-1.0, 1.0);
    let mut rng = rand::thread_rng();

    loop {
        let p = Point::from_distribution(&distribution, &mut rng);

        if p.norm_squared() < 1.0 {
            return p;
        }
    }
}

fn random_unit_vector() -> Point {
    random_point_in_unit_sphere().normalize()
}

fn random_point_in_hemisphere(normal: &Vector) -> Point {
    let p = random_point_in_unit_sphere();

    if p.dot(normal) > 0.0 {
        p
    } else {
        -p
    }
}

fn random_point_in_unit_disk() -> Point {
    let distribution = Uniform::new(-1.0, 1.0);
    let mut rng = rand::thread_rng();

    loop {
        let mut p = Point::from_distribution(&distribution, &mut rng);
        p[2] = 0.0;

        if p.norm_squared() < 1.0 {
            return p;
        }
    }
}

pub fn ray_color(ray: &Ray, world: &HittableList, recursion_depth: usize) -> Color {
    if recursion_depth == 0 {
        // if we exceed the depth, return no light
        Color::from_element(0.0)
    } else if let Some(hit_record) = world.hit(ray, 0.001, f64::INFINITY) {
        hit_record.material.scatter(ray, &hit_record).map_or(
            Color::from_element(0.0),
            |(ref scattered_ray, ref attenuation)| {
                let mut ray = ray_color(scattered_ray, world, recursion_depth - 1);
                ray.component_mul_assign(attenuation);
                ray
            },
        )
    } else {
        // background color
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::from_element(1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

mod camera {
    use crate::random_point_in_unit_disk;
    use crate::util::{Point, Vector};
    use crate::Ray;

    pub struct Camera {
        pub origin: Point,
        lower_left_corner: Point,
        horizontal: Vector,
        vertical: Vector,
        u: Vector,
        v: Vector,
        w: Vector,
        lens_radius: f64,
    }

    impl Camera {
        pub fn new(
            lookfrom: Point,
            lookat: Point,
            vup: Vector,
            vertical_fov: f64,
            aspect_ratio: f64,
            aperture: f64,
            focus_dist: f64,
        ) -> Self {
            let theta = f64::to_radians(vertical_fov);
            let h = (theta / 2.0).tan();
            let viewport_height = 2.0 * h;
            let viewport_width = aspect_ratio * viewport_height;

            let w = (lookfrom - lookat).normalize();
            let u = vup.cross(&w).normalize();
            let v = w.cross(&u);

            let origin = lookfrom;
            let horizontal = focus_dist * viewport_width * u;
            let vertical = focus_dist * viewport_height * v;
            let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

            let lens_radius = aperture / 2.0;

            Self {
                origin,
                lower_left_corner,
                horizontal,
                vertical,
                u,
                v,
                w,
                lens_radius,
            }
        }

        pub fn get_ray(&self, s: f64, t: f64) -> Ray {
            let rd = self.lens_radius * random_point_in_unit_disk();
            let offset = self.u * rd.x + self.v * rd.y;

            Ray::new(
                self.origin + offset,
                self.lower_left_corner + s * self.horizontal + t * self.vertical
                    - self.origin
                    - offset,
            )
        }
    }
}
