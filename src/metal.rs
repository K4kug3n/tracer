use crate::material::Material;
use crate::ray::Ray;
use crate::hit_record::HitRecord;
use crate::color::Color;

pub struct Metal {
	albedo: Color
}

impl Material for Metal {
	fn scatter(&self, ray: &Ray, rec: &HitRecord) -> (Ray, Color) {
		let reflection = ray.direction().unit().reflect(&rec.normal);

		(Ray::new(rec.point, reflection), self.albedo)
	}
}

impl Metal {
	pub fn new(albedo: Color) -> Metal {
		Metal { 
			albedo
		}
	}
}