use image::GenericImageView;
use rust_it8951::{It8951, Mode};
use std::thread;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    println!("Start");
    let mut it8951 = It8951::connect()?;

    let inquiry_result = it8951.inquiry()?;
    println!("vendor: {}", inquiry_result.vendor);
    println!("product: {}", inquiry_result.product);
    println!("revision: {}", inquiry_result.revision);
    thread::sleep(Duration::from_millis(100));
    println!("We are now reading data");
    let system_info = it8951.get_system_info().unwrap();
    println!("width: {}", system_info.width);
    println!("height: {}", system_info.height);
    println!("mode: {}", system_info.mode);
    println!("version: {}", system_info.version);

    println!("Display data");
    let img = image::open("kitten.jpg")?;
    let grayscale_image = img.grayscale();
    let width = grayscale_image.width();
    let height = grayscale_image.height();

    // it8951.update_region(&system_info, &[], 0, 0, 0).unwrap();

    // 0 INIT: works - whole screen blanks
    // 1 DU:
    // 2: GC16: partial update, greyscale
    // 3: GL16
    // 4: GLR16
    // 5: GLD16
    // 6: DU4: 4 gray times
    // 7: A2: 2 bit pictures

    it8951.load_region(&grayscale_image, 0, 0)?;

    it8951.display_region(0, 0, width, height, Mode::GC16)?;
    println!("End");
    Ok(())
}
