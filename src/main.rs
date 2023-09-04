fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {

            let r = f64::from(i) / f64::from(image_width - 1);
            let g = f64::from(j) / f64::from(image_height - 1);
            let b = 0f64;

            let ir = (255.99 * r) as i64;
            let ig = (255.99 * g) as i64;
            let ib = (255.99 * b) as i64;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    eprintln!("\rDone                           ");
}
