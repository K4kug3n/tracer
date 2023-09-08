use crate::ray::Ray;
use crate::hit_record::HitRecord;
use crate::color::Color;

pub trait Material {
	fn scatter(&self, ray: &Ray, rec: &HitRecord, attenuation: &Color) -> Option<Ray>;
}