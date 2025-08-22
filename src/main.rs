mod aabb;
mod bvh;
mod camera;
mod color;
mod hit;
mod interval;
mod material;
mod ray;
mod sphere;
mod texture;
mod utils;
mod vec3;

use camera::Camera;
use color::Color;
use hit::Hittables;
use material::{Dielectric, Lambertian, Metal};
use sphere::Sphere;
use std::sync::Arc;
use utils::random_f64;
use vec3::Vec3;

use crate::{bvh::BVHNode, texture::CheckerTexture, utils::random_range_f64};

fn main() {
    let mut world = Hittables::default();

    let checker = Arc::new(CheckerTexture::new_solid(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(checker)),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Vec3::new(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                match choose_mat {
                    choose_mat if choose_mat < 0.8 => {
                        let albedo = Color::random() * Color::random();
                        let mat = Arc::new(Lambertian::new_color(albedo));
                        let center2 = center + Vec3::new(0.0, random_range_f64(0.0, 0.5), 0.0);
                        world.add(Arc::new(Sphere::new_moving(center, center2, 0.2, mat)));
                    }
                    choose_mat if choose_mat < 0.95 => {
                        let albedo = Color::random_range(0.5, 1.0);
                        let fuzz = random_range_f64(0.0, 0.5);
                        let mat = Arc::new(Metal::new(albedo, fuzz));
                        world.add(Arc::new(Sphere::new(center, 0.2, mat)));
                    }
                    _ => {
                        let mat = Arc::new(Dielectric::new(1.5));
                        world.add(Arc::new(Sphere::new(center, 0.2, mat)));
                    }
                }
            }
        }
    }

    let material_1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));
    let material_2 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));
    let material_3 = Arc::new(Lambertian::new_color(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    let bvh = BVHNode::from(world);
    let world = Hittables::from(Arc::new(bvh));

    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 10;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Vec3::new(13.0, 2.0, 3.0);
    cam.lookat = Vec3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;
    cam.render(&world);
}
