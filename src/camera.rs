use crate::ray::Ray;
use crate::hittable::Hittable;
use crate::color::Color;
use crate::interval::Interval;
use crate::vec3::{Point3, Vec3};
use rand::Rng;

pub struct Camera {
	pub samples_per_pixel: i8,
	pub max_depth: i8,
	pub image_width: i32,
	pub aspect_ratio: f64,
	image_height: i32,
	center: Point3,
	pixel00_loc: Point3,
	pixel_delta_u: Vec3,
	pixel_delta_v: Vec3
}

impl Camera {
	pub fn new() -> Camera {
		Camera {
			samples_per_pixel: 10,
			max_depth: 10,
			image_width: 100,
			aspect_ratio: 1.,
			image_height: 100,
			center: Vec3::new(0., 0., 0.),
			pixel00_loc: Point3::new(-1., 1., -1.),
			pixel_delta_u: Vec3::new(0.02, 0., 0.),
			pixel_delta_v: Vec3::new(0., -0.02, 0.)
		}
	}

	pub fn render(&mut self, world: &dyn Hittable) {
		self.initialise();

		println!("P3");
    	println!("{} {}", self.image_width, self.image_height);
    	println!("255");

    	for j in 0..self.image_height {
    	    eprint!("\rScanlines remaining: {} ", self.image_height - j);
    	    for i in 0..self.image_width {
    	        
    	        let mut pixel_color = Color::new(0., 0., 0.);
				for _ in 0..self.samples_per_pixel {
					let ray = self.get_ray(i, j);
					pixel_color += Camera::ray_color(&ray, self.max_depth, world);
				}

    	        pixel_color.write(self.samples_per_pixel);
    	    }
    	}

		eprintln!("\rDone                           ");
	}

	fn initialise(&mut self) {
		self.image_height = f64::max(f64::from(self.image_width) / self.aspect_ratio, 1.) as i32;

		let focal_length = 1.;
    	let viewport_height = 2.;
    	let viewport_width = viewport_height * (f64::from(self.image_width) / f64::from(self.image_height));
    	let camera_center = Point3::new(0., 0., 0.);

    	let viewport_u = Vec3::new(viewport_width, 0., 0.);
    	let viewport_v = Vec3::new(0., -viewport_height, 0.);

    	self.pixel_delta_u = viewport_u / f64::from(self.image_width);
    	self.pixel_delta_v = viewport_v / f64::from(self.image_height);

    	let viewport_upper_left = camera_center - viewport_u / 2. - viewport_v / 2. - Vec3::new(0., 0., focal_length);
    	self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
	}

	fn ray_color(ray: &Ray, depth: i8, world: &dyn Hittable) -> Color {
		if depth == 0 {
			return Color::new(0., 0., 0.);
		}

		if let Some(record) = world.hit(ray, &Interval { min: 0.001, max: f64::MAX }) {
			let direction = Vec3::random_on_hemisphere(&record.normal);

			return 0.5 * Camera::ray_color(&Ray::new(record.point, direction), depth - 1, world);
		}
	
		let unit_direction = ray.direction().unit();
		let a = 0.5 * (unit_direction.y() + 1.);
	
		(1. - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.)
	}

	fn get_ray(&self, i: i32, j: i32) -> Ray {
		let pixel_center = self.pixel00_loc + (self.pixel_delta_v * f64::from(j)) + (self.pixel_delta_u * f64::from(i));
		let pixel_sample = pixel_center + self.pixel_sample_square();

		let ray_direction = pixel_sample - self.center;

    	Ray::new(self.center, ray_direction)
	}

	fn pixel_sample_square(&self) -> Vec3 {
		let mut rng = rand::thread_rng();
		let px = -0.5 + rng.gen::<f64>();
		let py = -0.5 + rng.gen::<f64>();

		px * self.pixel_delta_u + py * self.pixel_delta_v
	}
}