pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;

use crate::hittable::Hittable;
use crate::color::Color;
use crate::vec3::{Vec3, Point3};
use crate::ray::Ray;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    if let Some(record) = world.hit(ray, 0., f64::MAX) {
        return 0.5 * (record.normal + Color::new(1., 1., 1.));
    }

    let unit_direction = ray.direction().unit();
    let a = 0.5 * (unit_direction.y() + 1.);

    (1. - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.)
}

fn main() {

    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let image_height = f64::max(f64::from(image_width) / aspect_ratio, 1.) as i32;

    let mut world = HittableList::new();
    world.push(Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)));
    world.push(Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)));

    let focal_length = 1.;
    let viewport_height = 2.;
    let viewport_width = viewport_height * (f64::from(image_width) / f64::from(image_height));
    let camera_center = Point3::new(0., 0., 0.);

    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., -viewport_height, 0.);

    let pixel_delta_u = viewport_u / f64::from(image_width);
    let pixel_delta_v = viewport_v / f64::from(image_height);

    let viewport_upper_left = camera_center - viewport_u / 2. - viewport_v / 2. - Vec3::new(0., 0., focal_length);
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel00_loc + (pixel_delta_v * f64::from(j)) + (pixel_delta_u * f64::from(i));
            let ray_direction = pixel_center - camera_center;

            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray, &world);
            pixel_color.write();
        }
    }

    eprintln!("\rDone                           ");
}
