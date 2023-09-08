use crate::material::Material;
use crate::ray::Ray;
use crate::hit_record::HitRecord;
use crate::color::Color;
use crate::vec3::Vec3;

pub struct Metal {
	albedo: Color,
	fuzz: f64
}

impl Material for Metal {
	fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
		let reflection = ray.direction().unit().reflect(&rec.normal);
		let scattered = reflection + Vec3::random_unit() * self.fuzz;
		if scattered.dot(rec.normal) <= 0. {
			return None;
		}

		Some((Ray::new(rec.point, scattered), self.albedo))
	}
}

impl Metal {
	pub fn new(albedo: Color, fuzz: f64) -> Metal {
		Metal { 
			albedo,
			fuzz
		}
	}
}