pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod interval;
pub mod camera;

use crate::vec3::Vec3;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::camera::Camera;



fn main() {
    let mut world = HittableList::new();
    world.push(Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)));
    world.push(Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)));

    let mut camera = Camera::new();

    camera.aspect_ratio = 16. / 9.;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;

    camera.render(&world);
}
