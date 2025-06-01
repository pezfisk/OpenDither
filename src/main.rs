#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
slint::include_modules!();
use dither_lib::{DitherBuilder, Resize, Rgb};
use image::{DynamicImage, imageops};
use slint::{Image as SlintImage, SharedString};
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let window = AppWindow::new()?;
    let ui = Arc::new(window);

    let image: DynamicImage = image::open("/home/marc/Documents/burning_desire.png").unwrap();
    println!("Image: {}x{}", image.width(), image.height());

    {
        let ui_copy = ui.clone();
        let ui_weak = ui.as_weak();

        ui.on_dither(move |dither_level| {
            let image = image.clone();
            let value = ui_weak.clone();
            thread::spawn(move || {
                let start = Instant::now();
                println!("Starting: {}x{}", image.width(), image.height());

                let rgaa_og_image = image.to_rgba8();
                let width = rgaa_og_image.width();
                let height = rgaa_og_image.height();
                let pixel_data = rgaa_og_image.into_raw();

                let mut pixel_buffer_og =
                    slint::SharedPixelBuffer::<slint::Rgba8Pixel>::new(width, height);
                pixel_buffer_og
                    .make_mut_bytes()
                    .copy_from_slice(&pixel_data);

                let image = imageops::brighten(&image, 30);

                let level = dither_level as u8;
                let sepia = DitherBuilder::new(image)
                    .highlights(Rgb([255, 255, 255]))
                    .shadows(Rgb([0, 0, 0]))
                    .level(level)
                    .generate();

                println!("{}x{}", sepia.width(), sepia.height());

                let duration = start.elapsed().as_millis();
                println!("Before setting image: {} ms", duration);

                let rgba_image = sepia.to_rgba8();
                let width = rgba_image.width();
                let height = rgba_image.height();
                let pixel_data = rgba_image.into_raw();

                let mut pixel_buffer =
                    slint::SharedPixelBuffer::<slint::Rgba8Pixel>::new(width, height);
                pixel_buffer.make_mut_bytes().copy_from_slice(&pixel_data);

                slint::invoke_from_event_loop(move || {
                    if let Some(ui) = value.upgrade() {
                        let imageMod = SlintImage::from_rgba8_premultiplied(pixel_buffer);
                        ui.set_image(imageMod);
                        let image_og = SlintImage::from_rgba8_premultiplied(pixel_buffer_og);
                        ui.set_image2(image_og);
                    }
                })
                .unwrap();

                let duration = start.elapsed().as_millis();
                println!("After setting image: {} ms", duration);
            });
        });
    }

    ui.run()?;
    Ok(())
}
