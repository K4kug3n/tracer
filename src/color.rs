use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
	pub fn write(&self) {
            let ir = (255.99 * self.x()) as i64;
            let ig = (255.99 * self.y()) as i64;
            let ib = (255.99 * self.z()) as i64;

            println!("{} {} {}", ir, ig, ib);
	}
}