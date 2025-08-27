mod aabb;
mod bvh;
mod camera;
mod color;
mod hit;
mod interval;
mod material;
mod perlin;
mod quad;
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

use crate::{
    bvh::BVHNode,
    hit::{Rotated, Translated},
    material::DiffuseLight,
    quad::{Quad, make_box},
    texture::{CheckerTexture, NoiseTexture},
    utils::random_range_f64,
};

fn main() {
    cornell_box();
}

fn cornell_box() {
    let mut world = Hittables::default();

    let red = Arc::new(Lambertian::new_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_color(Color::new(15.0, 15.0, 15.0)));

    // Make walls and light
    world.add(Arc::new(Quad::new(
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    // Make boxes
    let box1 = make_box(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = Arc::new(Rotated::new(box1, 15.0));
    let box1 = Arc::new(Translated::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let box2 = make_box(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        white,
    );
    let box2 = Arc::new(Rotated::new(box2, -18.0));
    let box2 = Arc::new(Translated::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);

    let mut cam = Camera::builder()
        .with_aspect_ratio(1.0)
        .with_image_width(600)
        .with_samples_per_pixel(200)
        .with_max_depth(50)
        .with_background(Color::default())
        .with_vfov(40.0)
        .with_lookfrom(Vec3::new(278.0, 278.0, -800.0))
        .with_lookat(Vec3::new(278.0, 278.0, 0.0))
        .with_vup(Vec3::new(0.0, 1.0, 0.0))
        .with_defocus_angle(0.0)
        .with_focus_dist(10.0)
        .build();

    cam.render(&world);
}

fn simple_light() {
    let mut world = Hittables::default();
    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(pertext)),
    )));

    let difflight = Arc::new(DiffuseLight::new_color(Color::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(Quad::new(
        Vec3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 7.0, 0.0),
        2.0,
        difflight,
    )));

    let mut cam = Camera::builder()
        .with_aspect_ratio(16.0 / 9.0)
        .with_image_width(400)
        .with_samples_per_pixel(100)
        .with_max_depth(50)
        .with_vfov(20.0)
        .with_lookfrom(Vec3::new(26.0, 3.0, 6.0))
        .with_lookat(Vec3::new(0.0, 2.0, 0.0))
        .with_vup(Vec3::new(0.0, 1.0, 0.0))
        .with_defocus_angle(0.0)
        .with_focus_dist(10.0)
        .with_background(Color::default())
        .build();
    cam.render(&world);
}

fn quads() {
    let mut world = Hittables::default();
    let quad_color = Arc::new(Lambertian::new_color(Color::random()));

    world.add(Arc::new(Quad::new(
        Vec3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        quad_color.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        quad_color.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Vec3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        quad_color.clone(),
    )));

    let mut cam = Camera::builder()
        .with_aspect_ratio(1.0)
        .with_image_width(400)
        .with_samples_per_pixel(100)
        .with_max_depth(50)
        .with_vfov(80.0)
        .with_lookfrom(Vec3::new(0.0, 0.0, 9.0))
        .with_lookat(Vec3::new(0.0, 0.0, 0.0))
        .with_vup(Vec3::new(0.0, 1.0, 0.0))
        .with_defocus_angle(0.0)
        .with_focus_dist(10.0)
        .with_background(Color::new(0.75, 0.1, 0.75))
        .build();
    cam.render(&world);
}

fn perlin_spheres() {
    let mut world = Hittables::default();
    let perlin = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(perlin.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(perlin.clone())),
    )));

    let mut cam = Camera::builder()
        .with_aspect_ratio(16.0 / 9.0)
        .with_image_width(400)
        .with_samples_per_pixel(100)
        .with_max_depth(50)
        .with_vfov(20.0)
        .with_lookfrom(Vec3::new(13.0, 2.0, 3.0))
        .with_lookat(Vec3::new(0.0, 0.0, 0.0))
        .with_vup(Vec3::new(0.0, 1.0, 0.0))
        .with_defocus_angle(0.0)
        .with_focus_dist(10.0)
        .with_background(Color::new(0.75, 0.1, 0.75))
        .build();

    cam.render(&world);
}

fn checkered_spheres() {
    let mut world = Hittables::default();
    let checker = Arc::new(CheckerTexture::new_solid(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker)),
    )));

    let mut cam = Camera::builder()
        .with_aspect_ratio(16.0 / 9.0)
        .with_image_width(400)
        .with_samples_per_pixel(100)
        .with_max_depth(50)
        .with_vfov(20.0)
        .with_lookfrom(Vec3::new(13.0, 2.0, 3.0))
        .with_lookat(Vec3::new(0.0, 0.0, 0.0))
        .with_vup(Vec3::new(0.0, 1.0, 0.0))
        .with_defocus_angle(0.0)
        .with_focus_dist(10.0)
        .with_background(Color::new(0.75, 0.1, 0.75))
        .build();

    cam.render(&world);
}

fn bouncing_spheres() {
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

    let mut cam = Camera::builder()
        .with_aspect_ratio(16.0 / 9.0)
        .with_image_width(600)
        .with_samples_per_pixel(5)
        .with_max_depth(50)
        .with_vfov(20.0)
        .with_lookfrom(Vec3::new(13.0, 2.0, 3.0))
        .with_lookat(Vec3::new(0.0, 0.0, 0.0))
        .with_vup(Vec3::new(0.0, 1.0, 0.0))
        .with_defocus_angle(0.6)
        .with_focus_dist(10.0)
        .with_background(Color::new(1.0, 1.0, 1.0))
        .build();

    cam.render(&world);
}
