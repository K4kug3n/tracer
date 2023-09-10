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

use crate::vec3::{Vec3, Point3};
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::lambertian::Lambertian;
use crate::metal::Metal;
use crate::dielectric::Dielectric;
use crate::color::Color;

use rand::Rng;

fn main() {
    let mat_ground = Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
   

    let mut world = HittableList::new();
    world.push(Box::new(Sphere::new(Vec3::new(0., -1000., -1.), 1000., mat_ground)));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(f64::from(a) + 0.9 * rng.gen::<f64>(), 0.2, f64::from(b) + 0.9 * rng.gen::<f64>());

            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {

                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    world.push(Box::new(Sphere::new(center, 0.2, Box::new(Lambertian::new(albedo)))));
                }
                else if choose_mat < 0.95 {
                    let albedo = Color::borned_random(0.5, 1.);
                    let fuzz = rng.gen_range((0.)..(0.5));
                    world.push(Box::new(Sphere::new(center, 0.2, Box::new(Metal::new(albedo, fuzz)))));
                }
                else {
                    world.push(Box::new(Sphere::new(center, 0.2, Box::new(Dielectric::new(1.5)))));
                }
            }
        }
    }

    world.push(Box::new(Sphere::new(Point3::new(0., 1., 0.), 1., Box::new(Dielectric::new(1.5)))));
    world.push(Box::new(Sphere::new(Point3::new(-4., 1., 0.), 1., Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))))));
    world.push(Box::new(Sphere::new(Point3::new(4., 1., 0.), 1., Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.)))));

    let mut camera = Camera::new();

    camera.aspect_ratio = 16. / 9.;
    camera.image_width = 1200;
    camera.samples_per_pixel = 500;
    camera.max_depth = 50;

    camera.fov = 20.;
    camera.lookfrom = Point3::new(13., 2., 3.);
    camera.lookat = Point3::new(0., 0., 0.);
    camera.up = Vec3::new(0., 1., 0.);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.;

    camera.render(&world);
}
