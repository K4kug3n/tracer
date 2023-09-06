use crate::vec3::{Vec3, Point3};
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;

pub struct Sphere {
	center: Point3,
	radius: f64
}

impl Hittable for Sphere {
	fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
		let oc = ray.origin() - self.center;

		let a = ray.direction().squared_length();
		let half_b = oc.dot(ray.direction());
		let c = oc.squared_length() - self.radius * self.radius;
	
		let discriminant = half_b * half_b - a * c;
		if discriminant < 0. {
			return None
		}

		let sqrt_discr = f64::sqrt(discriminant);
		let mut root = (-half_b - sqrt_discr) / a;
		if root < t_min || t_max < root {
			root = (-half_b + sqrt_discr) / a;
			if root < t_min || t_max < root {
				return None
			}
		}

		let hit_point = ray.at(root);
		let outward_normal: Vec3 = (hit_point - self.center) / self.radius;
		
		let mut record = HitRecord::new(hit_point, root);
		record.set_face_normal(ray, &outward_normal);

		Some(record)
	}
}

impl Sphere {
	pub fn new(center: Vec3, radius: f64) -> Sphere {
		Sphere {
			center,
			radius
		}
	}
}