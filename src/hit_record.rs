use crate::vec3::{Vec3, Point3};
use crate::ray::Ray;
use crate::material::Material;

pub struct HitRecord<'a> {
	pub point: Point3,
	pub normal: Vec3,
	pub t: f64,
	pub front_face: bool,
	pub material: &'a dyn Material
}

impl HitRecord<'_> {
	pub fn new(point: Point3, t: f64, material: &dyn Material) -> HitRecord {
		HitRecord {
			point,
			normal: Vec3::new(0., 0., 0.),
			t,
			front_face: false,
			material
		}
	}

	pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
		self.front_face = ray.direction().dot(*outward_normal) < 0.;
		self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
	}
}