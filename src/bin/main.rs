use raytracer::*;

use rand::distributions::{Distribution, Uniform};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use rayon::prelude::*;

use std::convert::TryInto;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::sync::Arc;

fn main() -> std::io::Result<()> {
    // image
    let image = ImageSettings::from_width(400);
    // let image = ImageSettings::from_width(1920);
    let samples_per_pixel: usize = 50;
    let max_depth = 50;

    // world
    let items_in_scene = 11;
    let world = random_scene(items_in_scene);

    // camera
    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Vector::new(0.0, 1.0, 0.0);

    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        image.aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // rng
    let distribution = Uniform::new(-1.0, 1.0);
    let rng = StdRng::from_entropy();

    // render
    let denominator_u = image.img_width as f64 - 1.0;
    let denominator_v = image.img_height as f64 - 1.0;

    let progress = ProgressBar::new(image.img_height)
        .with_prefix("Rendering")
        .with_style(
            ProgressStyle::default_bar()
                .template("{spinner} {prefix} {percent}% [{elapsed_precise}/{duration_precise}]"),
        );

    let scene: Vec<Vec<Point>> = (0..image.img_height as usize)
        .into_par_iter()
        .rev()
        .map(|j| {
            progress.inc(1);

            let row: Vec<Point> = (0..image.img_width)
                .into_par_iter()
                .map(|i| {
                    let u = (i as f64) / denominator_u;
                    let v = (j as f64) / denominator_v;

                    let mut pixel =
                        (0..samples_per_pixel).fold(Color::from_element(0.0), |acc, _| {
                            let u_extra = distribution.sample(&mut rng.clone()) / denominator_u;
                            let v_extra = distribution.sample(&mut rng.clone()) / denominator_v;

                            let ray = camera.get_ray(u + u_extra, v + v_extra);
                            acc + ray_color(&ray, &world, max_depth)
                        });

                    pixel.apply(|x| {
                        // sqrt() is to gamma-correct for gamma=2.0
                        *x = (*x / samples_per_pixel as f64).sqrt().clamp(0.0, 0.999) * 256_f64;
                    });

                    pixel
                })
                .collect();

            row
        })
        .collect();

    progress.println(format!("{:?}", progress.elapsed()));

    // io
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("image.ppm")?;

    let mut writer =
        BufWriter::with_capacity(image.num_pixels() as usize * samples_per_pixel, file);

    write!(
        writer,
        "P3\n{} {}\n255\n",
        image.img_width, image.img_height
    )?;

    for row in scene {
        for pixel in row {
            writeln!(
                writer,
                "{} {} {}",
                pixel.x as i32, pixel.y as i32, pixel.z as i32
            )?;
        }
    }
    writer.flush()?;

    Ok(())
}

fn random_scene(size: isize) -> HittableList {
    let num_spheres: usize = (4 + (2 * size).pow(2)).try_into().unwrap();
    let mut world = HittableList::with_capacity(num_spheres);

    let ground_material = Arc::new(Lambertian::new(Color::from_element(0.5)));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let distribution = Uniform::new(-1.0, 1.0);
    let metal_albedo_distribution = Uniform::new(0.5, 1.0);
    let mut rng = rand::thread_rng();

    let origin_reference = Point::new(4.0, 0.2, 0.0);

    for a in -size..=size {
        for b in -size..=size {
            let choose_material = distribution.sample(&mut rng);
            let random_x = distribution.sample(&mut rng);
            let random_y = distribution.sample(&mut rng);

            let origin = Point::new(
                (a as f64) + 0.9 * random_x,
                0.2,
                (b as f64) + 0.9 * random_y,
            );

            if (origin - origin_reference).norm() > 0.9 {
                let sphere_material: Arc<dyn Material>;

                if choose_material < 0.8 {
                    // diffuse
                    let albedo = Color::from_distribution(&distribution, &mut rng)
                        .component_mul(&Color::from_distribution(&distribution, &mut rng));

                    sphere_material = Arc::new(Lambertian::new(albedo));
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = Color::from_distribution(&metal_albedo_distribution, &mut rng);
                    let fuzz = rng.gen_range(0.0..0.5);

                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                } else {
                    // glass
                    sphere_material = Arc::new(Dielectric::new(1.5));
                }

                world.add(Arc::new(Sphere::new(origin, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

struct ImageSettings {
    aspect_ratio: f64,
    img_width: u64,
    img_height: u64,
}

impl ImageSettings {
    fn new(img_width: u64, aspect_ratio: f64) -> Self {
        Self {
            aspect_ratio,
            img_width,
            img_height: (img_width as f64 / aspect_ratio) as u64,
        }
    }

    /// Create an image with aspect ratio of 16:9 and given width
    fn from_width(width: u64) -> Self {
        let aspect_ratio = 16.0 / 9.0;
        Self::new(width, aspect_ratio)
    }

    fn num_pixels(&self) -> u64 {
        self.img_width * self.img_height
    }
}
