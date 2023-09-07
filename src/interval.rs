pub struct Interval {
	pub min: f64,
	pub max: f64
}

impl Interval {
	pub fn empty() -> Interval {
		Interval {
			min: f64::MAX,
			max: f64::MIN
		}
	}

	pub fn universe() -> Interval {
		Interval {
			min: f64::MIN,
			max: f64::MAX
		}
	}

	pub fn contains(&self, x: f64) -> bool {
		self.min <= x && x <= self.max
	}

	pub fn surround(&self, x: f64) -> bool {
		self.min < x && x < self.max
	}

	pub fn clamp(&self, x: f64) -> f64 {
		if x > self.max {
			return self.max;
		}
		if x < self.min {
			return self.min;
		}

		x
	}
}