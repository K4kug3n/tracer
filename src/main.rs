pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod interval;
pub mod camera;
pub mod hit_record;
pub mod material;
pub mod lambertian;
pub mod metal;
pub mod dielectric;

use crate::vec3::Vec3;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::lambertian::Lambertian;
use crate::metal::Metal;
use crate::dielectric::Dielectric;
use crate::color::Color;

fn main() {
    let mat_ground = Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.)));
    let mat_center = Box::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let mat_left = Box::new(Dielectric::new(1.5));
    let mat_right = Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.));

    let mut world = HittableList::new();
    world.push(Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100., mat_ground)));
    world.push(Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5, mat_center)));
    world.push(Box::new(Sphere::new(Vec3::new(-1., 0., -1.), 0.5, mat_left)));
    world.push(Box::new(Sphere::new(Vec3::new(1., 0., -1.), 0.5, mat_right)));

    let mut camera = Camera::new();

    camera.aspect_ratio = 16. / 9.;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;

    camera.render(&world);
}
