use crate::vec3::Vec3;
use crate::interval::Interval;

pub type Color = Vec3;

impl Color {
	pub fn write(&self, samples_per_pixel: i8) {
            let scale = 1. / f64::from(samples_per_pixel);

            let mut r = self.x();
            let mut g = self.y();
            let mut b = self.z();

            r *= scale;
            g *= scale;
            b *= scale;

            let interval = Interval{ min: 0., max: 0.99 }; 
            let ir = (255.99 * interval.clamp(r)) as i64;
            let ig = (255.99 * interval.clamp(g)) as i64;
            let ib = (255.99 * interval.clamp(b)) as i64;

            println!("{} {} {}", ir, ig, ib);
	}
}