use crate::ray::Ray;
use crate::interval::Interval;
use crate::hit_record::HitRecord;


pub trait Hittable {
	fn hit(&self, ray: &Ray, t: &Interval) -> Option<HitRecord>;
}