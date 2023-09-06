use std::vec::Vec;

use crate::hittable::{Hittable, HitRecord};

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
	fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
		let mut record = None;
		let mut closest_so_far = t_max;

		for object in self.objects.iter() {
			if let Some(new_rec) = object.hit(ray, t_min, closest_so_far) {
				closest_so_far = new_rec.t;
				record = Some(new_rec);
			}
		}

		record
	}
}