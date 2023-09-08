use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Div, Neg};
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
	values : [f64; 3]
}

pub type Point3 = Vec3;

impl Vec3 {
	pub fn new(v1: f64, v2: f64, v3: f64) -> Vec3 {
		Vec3 {
			values: [v1, v2, v3]
		}
	}

	pub fn random() -> Vec3 {
		let mut rng = rand::thread_rng();

		Vec3::new(rng.gen(), rng.gen(), rng.gen())
	}

	pub fn borned_random(min: f64, max: f64) -> Vec3 {
		let mut rng = rand::thread_rng();

		Vec3::new(rng.gen_range(min..max), rng.gen_range(min..max), rng.gen_range(min..max))
	}

	pub fn random_in_unit_sphere() -> Vec3 {
		loop {
			let vec = Vec3::borned_random(-1., 1.);
			if vec.squared_length() < 1. {
				return vec;
			}
		}
	}

	pub fn random_unit() -> Vec3 {
		Vec3::random_in_unit_sphere().unit()
	}

	pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
		let on_unit_sphere = Vec3::random_unit();
		if normal.dot(on_unit_sphere) < 0. {
			return -on_unit_sphere;
		}

		on_unit_sphere
	}

	pub fn x(&self) -> f64 {
		self.values[0]
	} 

	pub fn y(&self) -> f64 {
		self.values[1]
	}

	pub fn z(&self) -> f64 {
		self.values[2]
	}

	pub fn cross(&self, rhs: Vec3) -> Vec3 {
		Vec3::new(
			self.y() * rhs.z() - self.z() * rhs.y(),
			self.z() * rhs.x() - self.x() * rhs.z(),
			self.x() * rhs.y() - self.y() * rhs.x()
		)
	}

	pub fn dot(&self, rhs: Vec3) -> f64 {
		self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
	}

	pub fn length(&self) -> f64 {
		f64::sqrt(self.squared_length())
	}

	pub fn squared_length(&self) -> f64 {
		self.values[0] * self.values[0] + self.values[1] * self.values[1] + self.values[2] * self.values[2]
	}

	pub fn unit(&self) -> Vec3 {
		*self / self.length()
	}

	pub fn near_zero(&self) -> bool {
		let eps = 1e-8;

		(f64::abs(self.x()) < eps) && (f64::abs(self.y()) < eps) && (f64::abs(self.z()) < eps)
	}

	pub fn reflect(&self, normal: &Vec3) -> Vec3 {
		*self - 2. * self.dot(*normal) * *normal
	}
}

impl Add for Vec3 {
	type Output = Vec3;

	fn add(self, rhs: Self) -> Self::Output {
		Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
	}
}

impl AddAssign for Vec3 {
	fn add_assign(&mut self, rhs: Self) {
		*self = *self + rhs
	}
}

impl Sub for Vec3 {
	type Output = Vec3;

	fn sub(self, rhs: Self) -> Self::Output {
		Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
	}
}

impl SubAssign for Vec3 {
	fn sub_assign(&mut self, rhs: Self) {
		*self = *self - rhs
	}
}

impl Mul<f64> for Vec3 {
	type Output = Vec3;

	fn mul(self, scalar: f64) -> Self::Output {
		Vec3::new(self.x() * scalar, self.y() * scalar, self.z() * scalar)
	}
}

impl Mul<Vec3> for f64 {
	type Output = Vec3;

	fn mul(self, vec: Vec3) -> Self::Output {
		vec * self
	}
}

impl Mul for Vec3 {
	type Output = Vec3;
	
	fn mul(self, rhs: Self) -> Self::Output {
		Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
	}
}

impl Div<f64> for Vec3 {
	type Output = Vec3;

	fn div(self, scalar: f64) -> Self::Output {
		Vec3::new(self.x() / scalar, self.y() / scalar, self.z() / scalar)
	}
}

impl Neg for Vec3 {
	type Output = Vec3;

	fn neg(self) -> Self::Output {
		Vec3::new(-self.x(), -self.y(), -self.z())	
	}
}

#[cfg(test)]
mod tests {
	use crate::vec3::Vec3;

    #[test]
	fn add() {
		let lhs = Vec3::new(1., 2., 3.);
		let rhs = Vec3::new(0., 1., 2.);

		let res = lhs + rhs;

		assert_eq!(res.values, [1., 3., 5.]);
	}

	#[test]
	fn add_assign() {
		let mut lhs = Vec3::new(1., 2., 3.);
		let rhs = Vec3::new(0., 1., 2.);

		lhs += rhs;

		assert_eq!(lhs.values, [1., 3., 5.]);
	}

	#[test]
	fn sub() {
		let lhs = Vec3::new(1., 2., 3.);
		let rhs = Vec3::new(0., 1., 2.);

		let res = lhs - rhs;

		assert_eq!(res.values, [1., 1., 1.]);
	}

	#[test]
	fn sub_assign() {
		let mut lhs = Vec3::new(1., 2., 3.);
		let rhs = Vec3::new(0., 1., 2.);

		lhs -= rhs;

		assert_eq!(lhs.values, [1., 1., 1.]);
	}

	#[test]
	fn mul() {
		let vec = Vec3::new(1., 2., 3.);

		let res_1 = vec * 3.;
		let res_2 = 3. * vec;
		let res_3 = vec * vec;

		assert_eq!(res_1.values, [3., 6., 9.]);
		assert_eq!(res_2.values, [3., 6., 9.]);
		assert_eq!(res_3.values, [1., 4., 9.]);
	}

	#[test]
	fn div() {
		let vec = Vec3::new(3., 6., 9.);

		let res = vec / 3.;

		assert_eq!(res.values, [1., 2., 3.]);
	}

	#[test]
	fn neg() {
		let vec = Vec3::new(3., 6., 9.);

		let res = -vec;

		assert_eq!(res.values, [-3., -6., -9.]);
	}

	#[test]
	fn length() {
		let vec = Vec3::new(2., 0., 0.);

		assert_eq!(vec.length(), 2.);
	}

	#[test]
	fn cross() {
		let a = Vec3::new(2., 3., 4.);
		let b = Vec3::new(5., 6., 7.);

		let res = a.cross(b);

		assert_eq!(res.values, [-3., 6., -3.]);
	}

	#[test]
	fn dot() {
		let a = Vec3::new(4., 8., 10.);
		let b = Vec3::new(9., 2., 7.);

		let res = a.dot(b);

		assert_eq!(res, 122.);
	}

}