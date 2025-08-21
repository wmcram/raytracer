mod camera;
mod color;
mod hit;
mod interval;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use hit::Hittables;
use sphere::Sphere;
use std::rc::Rc;
use vec3::Vec3;

fn main() {
    let mut world = Hittables::default();
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let cam = Camera::new(16.0 / 9.0, 400);
    cam.render(&world);
}
