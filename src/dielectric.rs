use crate::material::Material;
use crate::ray::Ray;
use crate::hit_record::HitRecord;
use crate::color::Color;

use rand::Rng;

pub struct Dielectric {
	refraction_coeff: f64,
}

impl Material for Dielectric {
	fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
		let refraction_ratio = if rec.front_face { 1. / self.refraction_coeff } else { self.refraction_coeff };
		
		let unit_direction = ray.direction().unit();
		let cos_theta = f64::min((-unit_direction).dot(rec.normal), 1.0);
		let sin_theta = f64::sqrt(1. - cos_theta * cos_theta);

		let cannot_refract = (refraction_ratio * sin_theta) > 1.0;
		if cannot_refract || Dielectric::reflectance(cos_theta, self.refraction_coeff) > rand::thread_rng().gen() {
			let reflected = unit_direction.reflect(&rec.normal);

			return Some((Ray::new(rec.point, reflected), Color::new(1., 1., 1.)));
		}
		
		let refracted = unit_direction.refract(&rec.normal, refraction_ratio);

		Some((Ray::new(rec.point, refracted), Color::new(1., 1., 1.)))
	}
}

impl Dielectric {
	pub fn new(refraction_coeff: f64) -> Dielectric {
		Dielectric { 
			refraction_coeff
		}
	}

	fn reflectance(cosine: f64, refraction_coeff: f64) -> f64 {
		let mut r0 = (1. - refraction_coeff) / (1. + refraction_coeff);
		r0 = r0 * r0;

		r0 + (1. - r0) * f64::powf(1. - cosine, 5.)
	}
}