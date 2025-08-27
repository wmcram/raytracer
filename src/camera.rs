use std::io;

use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::color::{Color, write_color};
use crate::hit::{Hit, HitRecord};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utils::random_f64;
use crate::vec3::{Vec3, cross, unit_vector};

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: usize,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f64,
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pub background: Color,

    image_height: usize,
    center: Vec3,
    pixel00_loc: Vec3,
    // pixel gaps
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    // normalization factor for samples
    pixel_samples_scale: f64,
    // camera frame basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

// builder fns
impl Camera {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn with_aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn with_image_width(mut self, image_width: usize) -> Self {
        self.image_width = image_width;
        self
    }

    pub fn with_samples_per_pixel(mut self, samples_per_pixel: u32) -> Self {
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn with_max_depth(mut self, max_depth: u32) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn with_vfov(mut self, vfov: f64) -> Self {
        self.vfov = vfov;
        self
    }

    pub fn with_lookfrom(mut self, lookfrom: Vec3) -> Self {
        self.lookfrom = lookfrom;
        self
    }

    pub fn with_lookat(mut self, lookat: Vec3) -> Self {
        self.lookat = lookat;
        self
    }

    pub fn with_vup(mut self, vup: Vec3) -> Self {
        self.vup = vup;
        self
    }

    pub fn with_defocus_angle(mut self, defocus_angle: f64) -> Self {
        self.defocus_angle = defocus_angle;
        self
    }

    pub fn with_focus_dist(mut self, focus_dist: f64) -> Self {
        self.focus_dist = focus_dist;
        self
    }

    pub fn with_background(mut self, background: Color) -> Self {
        self.background = background;
        self
    }

    pub fn build(mut self) -> Self {
        self.initialize();
        self
    }
}

impl Camera {
    pub fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
        self.center = self.lookfrom;

        let theta = f64::to_radians(self.vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = unit_vector(self.lookfrom - self.lookat);
        self.u = unit_vector(cross(self.vup, self.w));
        self.v = cross(self.w, self.u);

        let viewport_u = self.u * viewport_width;
        let viewport_v = -self.v * viewport_height;
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;
        let viewport_upper_left =
            self.center - (self.w * self.focus_dist) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        let defocus_radius = self.focus_dist * f64::tan(f64::to_radians(self.defocus_angle / 2.0));
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    // Outputs the image (in PPM format) to standard output.
    pub fn render(&mut self, world: &dyn Hit) {
        self.initialize();
        let total = self.image_width * self.image_height;

        eprintln!("Rendering...");
        print!("P3\n{} {}\n255\n", self.image_width, self.image_height);
        let pixels: Vec<Color> = (0..total)
            .into_par_iter()
            .progress_count(total as u64)
            .map(|k| {
                let j = k / self.image_width;
                let i = k % self.image_width;
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }
                pixel_color * self.pixel_samples_scale
            })
            .collect();
        let mut out = io::BufWriter::new(io::stdout().lock());
        for color in pixels {
            write_color(&mut out, &color);
        }
        eprint!("\rDone.              \n");
    }

    // Determines the color that the camera sees along this ray. This function calls itself
    // recursively up to limit `depth` to account for reflection/refraction.
    fn ray_color(&self, r: &Ray, depth: u32, world: &dyn Hit) -> Color {
        if depth <= 0 {
            return Color::default();
        }

        let mut rec = HitRecord::default();

        if !world.hit(
            r,
            Interval {
                min: 0.001,
                max: f64::INFINITY,
            },
            &mut rec,
        ) {
            return self.background;
        }

        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        let color_from_emission = rec.mat.emitted(rec.u, rec.v, rec.p);
        if !rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return color_from_emission;
        }
        let color_from_scatter = attenuation * self.ray_color(&scattered, depth - 1, world);
        color_from_emission + color_from_scatter
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (i as f64 + offset.x()))
            + (self.pixel_delta_v * (j as f64 + offset.y()));
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_f64();
        Ray::new(ray_origin, ray_direction).with_time(ray_time)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        return self.center + (self.defocus_disk_u * p[0]) + (self.defocus_disk_v * p[1]);
    }
}
