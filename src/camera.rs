use crate::color::{Color, write_color};
use crate::hit::{Hit, HitRecord};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utils::random_f64;
use crate::vec3::{Vec3, unit_vector};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: usize,
    pub samples_per_pixel: u32,
    pub max_depth: u32,

    image_height: usize,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: usize, samples_per_pixel: u32) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as usize;
        let image_height = if image_height < 1 { 1 } else { image_height };
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;
        let center = Vec3::new(0.0, 0.0, 0.0);
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth: 50,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixel_samples_scale,
        }
    }

    pub fn render(&self, world: &dyn Hit) {
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {}    ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, self.max_depth, world);
                }
                let pixel_color = pixel_color * self.pixel_samples_scale;
                write_color(&mut std::io::stdout(), &pixel_color);
            }
        }
        eprint!("\rDone.              \n");
    }

    fn ray_color(r: &Ray, depth: u32, world: &dyn Hit) -> Color {
        if depth <= 0 {
            return Color::default();
        }

        let mut rec = HitRecord::default();
        if world.hit(
            r,
            Interval {
                min: 0.001,
                max: f64::INFINITY,
            },
            &mut rec,
        ) {
            let mut scattered = Ray::default();
            let mut attentuation = Color::default();
            if rec.mat.scatter(r, &rec, &mut attentuation, &mut scattered) {
                return attentuation * Self::ray_color(&scattered, depth - 1, world);
            }
            return Color::default();
        }
        let unit_direction = unit_vector(&r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        return Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a;
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (i as f64 + offset.x()))
            + (self.pixel_delta_v * (j as f64 + offset.y()));
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(1.0, 100, 10)
    }
}
