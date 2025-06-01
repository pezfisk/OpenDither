use dither_lib::{DitherBuilder, Rgb};
use image::{DynamicImage, imageops};
use slint;
use std::time::Instant;

pub fn dither(
    image: DynamicImage,
    dither_level: i32,
    brightness_level: i32,
    contrast_level: f32,
) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    println!(
        "Dither level: {}, brightness level: {}, contrast level: {}",
        dither_level, brightness_level, contrast_level
    );

    let start = Instant::now();
    println!("Starting: {}x{}", image.width(), image.height());

    let contrast_level = contrast_level as f32;
    let image = imageops::brighten(&image, brightness_level);
    let duration = start.elapsed().as_millis();
    println!("After brightening: {} ms", duration);

    let image = imageops::contrast(&image, contrast_level);
    let duration = start.elapsed().as_millis();
    println!("After contrast: {} ms", duration);

    let image: DynamicImage = image.into();
    let duration = start.elapsed().as_millis();
    println!("After into: {} ms", duration);

    let rgaa_og_image = image.to_rgba8();
    let width = rgaa_og_image.width();
    let height = rgaa_og_image.height();
    let pixel_data = rgaa_og_image.into_raw();

    let mut pixel_buffer_og = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::new(width, height);
    pixel_buffer_og
        .make_mut_bytes()
        .copy_from_slice(&pixel_data);

    let duration = start.elapsed().as_millis();
    println!("Before dither: {} ms", duration);

    let level = dither_level as u8;
    let output = DitherBuilder::new(image)
        .highlights(Rgb([255, 255, 255]))
        .shadows(Rgb([0, 0, 0]))
        .level(level)
        .generate();

    let duration = start.elapsed().as_millis();
    println!("After dither: {} ms", duration);
    Ok(output)
}
