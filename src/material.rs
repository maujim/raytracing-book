use crate::util::Vector;
use crate::{random_point_in_unit_sphere, random_unit_vector};
use crate::{Color, HitRecord, Ray};
use rand::Rng;

pub trait Material {
    /// Returns the scattered ray and its attenuation
    fn scatter(&self, input_ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

fn vector_near_zero(vector: &Vector) -> bool {
    let very_small_f64 = 1e-8;

    vector < &Vector::from_element(very_small_f64)
}

impl Material for Lambertian {
    fn scatter(&self, _input_ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit_record.normal + random_unit_vector();

        // handle case where random_unit_vector is very close to -hit_record.normal
        // i.e. scatter_direction is very close to zero
        if vector_near_zero(&scatter_direction) {
            scatter_direction = hit_record.normal;
        };

        let scattered_ray = Ray::new(hit_record.point, scatter_direction);

        Some((scattered_ray, self.albedo))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

fn reflect(vector: &Vector, normal: &Vector) -> Vector {
    vector - 2.0 * vector.dot(normal) * normal
}

impl Material for Metal {
    fn scatter(&self, input_ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected_ray = reflect(&input_ray.direction.normalize(), &hit_record.normal);

        let scattered_ray = Ray::new(
            hit_record.point,
            reflected_ray + self.fuzz * random_point_in_unit_sphere(),
        );

        if scattered_ray.direction.dot(&hit_record.normal) > 0.0 {
            Some((scattered_ray, self.albedo))
        } else {
            None
        }
    }
}

fn refract(vector: &Vector, normal: &Vector, refraction_ratio: f64) -> Vector {
    let cos_theta = (-vector).dot(normal).min(1.0);
    let r_out_perpendicular = refraction_ratio * (vector + cos_theta * normal);
    let r_out_parallel = normal * -1.0 * ((1.0 - r_out_perpendicular.norm_squared()).abs().sqrt());

    r_out_perpendicular + r_out_parallel
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, input_ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let refraction_ratio = ternary!(
            hit_record.front_face,
            1.0 / self.refraction_index,
            self.refraction_index
        );

        let unit_direction = input_ray.direction.normalize();
        let cos_theta = (-unit_direction).dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = (refraction_ratio * sin_theta) > 1.0;
        let has_reflectance = Dielectric::reflectance(cos_theta, refraction_ratio)
            > rand::thread_rng().gen_range(0.0..1.0);

        let scatter_direction = ternary!(
            cannot_refract || has_reflectance,
            reflect(&unit_direction, &hit_record.normal),
            refract(&unit_direction, &hit_record.normal, refraction_ratio)
        );

        let scattered_ray = Ray::new(hit_record.point, scatter_direction);
        let attenuation = Color::from_element(1.0);

        Some((scattered_ray, attenuation))
    }
}
