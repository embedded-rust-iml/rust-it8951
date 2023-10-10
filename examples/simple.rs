use image::{EncodableLayout, GrayImage};
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
    println!("System Info: {:?}", system_info);
    let display_width = system_info.width;
    let display_height = system_info.height;

    let img = image::open("kitten.jpg")?;
    let grayscale_image = img.grayscale();
    let width = grayscale_image.width();
    let height = grayscale_image.height();

    let img_raw = image::open("puppy.png")?.to_luma8().as_bytes().to_vec();
    let img2 = image::DynamicImage::from(
        GrayImage::from_raw(display_width, display_height, img_raw).unwrap(),
    );

    // 0 INIT: works - whole screen blanks
    // 1 DU:
    // 2: GC16: partial update, greyscale
    // 3: GL16
    // 4: GLR16
    // 5: GLD16
    // 6: DU4: 4 gray times
    // 7: A2: 2 bit pictures

    println!("Display puppy data");
    it8951.load_region(&img2, 0, 0)?;
    it8951.display_region(0, 0, display_width, display_height, Mode::GC16)?;

    println!("Sleep 2 seconds");
    thread::sleep(Duration::from_millis(2000));

    println!("Load kitten data");
    it8951.load_region(&grayscale_image, 0, 0)?;

    println!("Power off device");
    it8951.set_power(false)?;

    println!("Power on device, sleep 100ms");
    it8951.set_power(true)?;

    thread::sleep(Duration::from_millis(100));

    println!("Display kitten data");
    it8951.display_region(0, 0, width, height, Mode::GC16)?;

    println!("End");

    Ok(())
}
