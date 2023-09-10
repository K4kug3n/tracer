use crate::ray::Ray;
use crate::hittable::Hittable;
use crate::color::Color;
use crate::interval::Interval;
use crate::vec3::{Point3, Vec3};

use rand::Rng;
use std::f64;

pub struct Camera {
	pub samples_per_pixel: i8,
	pub max_depth: i8,
	pub fov: f64,
	pub image_width: i32,
	pub aspect_ratio: f64,
	pub lookfrom: Point3,
	pub lookat: Point3,
	pub up: Vec3,
	pub defocus_angle: f64,
	pub focus_dist: f64,

	image_height: i32,
	center: Point3,
	pixel00_loc: Point3,
	pixel_delta_u: Vec3,
	pixel_delta_v: Vec3,
	u: Vec3,
	v: Vec3,
	w: Vec3,
	defocus_disk_u: Vec3,
	defocus_disk_v: Vec3
}

impl Camera {
	pub fn new() -> Camera {
		Camera {
			samples_per_pixel: 10,
			max_depth: 10,
			fov: 90.,
			image_width: 100,
			aspect_ratio: 1.,
			lookfrom: Point3::new(0., 0., -1.),
			lookat: Point3::new(0., 0., 0.),
			up: Vec3::new(0., 1., 0.),
			defocus_angle: 0.,
			focus_dist: 10.,

			image_height: 100,
			center: Vec3::new(0., 0., 0.),
			pixel00_loc: Point3::new(-1., 1., -1.),
			pixel_delta_u: Vec3::new(0.02, 0., 0.),
			pixel_delta_v: Vec3::new(0., -0.02, 0.),
			u: Vec3::new(0., 0., 0.),
			v: Vec3::new(0., 0., 0.),
			w: Vec3::new(0., 0., 0.),
			defocus_disk_u: Vec3::new(0., 0., 0.),
			defocus_disk_v: Vec3::new(0., 0., 0.)
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

		let theta = self.fov * f64::consts::PI / 180.;
		let h = f64::tan(theta / 2.);
    	let viewport_height = 2. * h * self.focus_dist;
    	let viewport_width = viewport_height * (f64::from(self.image_width) / f64::from(self.image_height));
    	self.center = self.lookfrom;

		self.w = (self.lookfrom - self.lookat).unit();
		self.u = (self.up.cross(self.w)).unit();
		self.v = self.w.cross(self.u);

    	let viewport_u = viewport_width * self.u;
    	let viewport_v = viewport_height * -self.v;

    	self.pixel_delta_u = viewport_u / f64::from(self.image_width);
    	self.pixel_delta_v = viewport_v / f64::from(self.image_height);

    	let viewport_upper_left = self.center - (viewport_u / 2.) - (viewport_v / 2.) - (self.focus_dist * self.w);
    	self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

		let defocus_radius = self.focus_dist * f64::tan((self.defocus_angle / 2.) * f64::consts::PI / 180.);
		self.defocus_disk_u = self.u * defocus_radius;
		self.defocus_disk_v = self.v * defocus_radius;
	}

	fn ray_color(ray: &Ray, depth: i8, world: &dyn Hittable) -> Color {
		if depth == 0 {
			return Color::new(0., 0., 0.);
		}

		if let Some(record) = world.hit(ray, &Interval { min: 0.001, max: f64::MAX }) {
			if let Some((scatter, attenuation)) = record.material.scatter(ray, &record) {
				return attenuation * Camera::ray_color(&scatter, depth - 1, world);
			}

			return Color::new(0., 0., 0.);
		}
	
		let unit_direction = ray.direction().unit();
		let a = 0.5 * (unit_direction.y() + 1.);
	
		(1. - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.)
	}

	fn get_ray(&self, i: i32, j: i32) -> Ray {
		let pixel_center = self.pixel00_loc + (self.pixel_delta_v * f64::from(j)) + (self.pixel_delta_u * f64::from(i));
		let pixel_sample = pixel_center + self.pixel_sample_square();

		let ray_origin = if self.defocus_angle <= 0. { self.center } else { self.defocus_disk_sample() };
		let ray_direction = pixel_sample - ray_origin;

    	Ray::new(ray_origin, ray_direction)
	}

	fn defocus_disk_sample(&self) -> Point3 {
		let p = Vec3::random_in_unit_disk();
		self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
	}

	fn pixel_sample_square(&self) -> Vec3 {
		let mut rng = rand::thread_rng();
		let px = -0.5 + rng.gen::<f64>();
		let py = -0.5 + rng.gen::<f64>();

		px * self.pixel_delta_u + py * self.pixel_delta_v
	}
}