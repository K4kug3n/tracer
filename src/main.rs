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

use crate::vec3::Vec3;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::lambertian::Lambertian;
use crate::color::Color;

fn main() {
    let red = Lambertian::new(Color::new(1., 0.5, 0.2));
    let green = Lambertian::new(Color::new(0.3, 1., 0.2));

    let mut world = HittableList::new();
    world.push(Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5, Box::new(red))));
    world.push(Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100., Box::new(green))));

    let mut camera = Camera::new();

    camera.aspect_ratio = 16. / 9.;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;

    camera.render(&world);
}
