use crate::vec3::{Vec3, Point3};
use crate::ray::Ray;

pub struct HitRecord {
	pub point: Point3,
	pub normal: Vec3,
	pub t: f64,
	pub front_face: bool
}

impl HitRecord {
	pub fn new(point: Point3, t: f64) -> HitRecord {
		HitRecord {
			point,
			normal: Vec3::new(0., 0., 0.),
			t,
			front_face: false
		}
	}

	pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
		self.front_face = ray.direction().dot(*outward_normal) < 0.;
		self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
	}
}

pub trait Hittable {
	fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}