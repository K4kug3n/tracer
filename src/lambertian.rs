use crate::material::Material;
use crate::ray::Ray;
use crate::hit_record::HitRecord;
use crate::color::Color;
use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Lambertian {
	albedo: Color
}

impl Material for Lambertian {
	fn scatter(&self, _: &Ray, rec: &HitRecord) -> (Ray, Color) {
		let mut direction = rec.normal + Vec3::random_unit();
		if direction.near_zero() {
			direction = rec.normal;
		}

		(Ray::new(rec.point, direction), self.albedo)
	}
}

impl Lambertian {
	pub fn new(albedo: Color) -> Lambertian {
		Lambertian { 
			albedo 
		}
	}
}