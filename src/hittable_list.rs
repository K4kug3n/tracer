use std::vec::Vec;

use crate::hittable::Hittable;
use crate::hit_record::HitRecord;
use crate::interval::Interval;

pub struct HittableList {
	objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
	pub fn new() -> HittableList {
		HittableList { 
			objects: Vec::new()
		}
	}

	pub fn push(&mut self, object: Box<dyn Hittable>) {
		self.objects.push(object);
	}

	pub fn clear(&mut self) {
		self.objects.clear();
	}
}

impl Hittable for HittableList {
	fn hit(&self, ray: &crate::ray::Ray, t: &Interval) -> Option<HitRecord> {
		let mut record = None;
		let mut closest_so_far = t.max;

		for object in self.objects.iter() {
			if let Some(new_rec) = object.hit(ray, &Interval{ min: t.min, max: closest_so_far }) {
				closest_so_far = new_rec.t;
				record = Some(new_rec);
			}
		}

		record
	}
}