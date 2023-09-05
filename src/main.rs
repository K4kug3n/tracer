pub mod vec3;
pub mod color;

use color::Color;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            
            let color = Color::new(f64::from(i) / f64::from(image_width - 1), f64::from(j) / f64::from(image_height - 1), 0.);
            color.write();
        }
    }

    eprintln!("\rDone                           ");
}
