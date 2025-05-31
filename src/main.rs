#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
slint::include_modules!();
use async_std::task;
use dither_lib::{DitherBuilder, Resize, Rgb};
use image::DynamicImage;
use slint::{Image as SlintImage, SharedString};
use std::sync::Arc;
use std::thread;
use tokio;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let window = AppWindow::new()?;
    let ui = Arc::new(window);

    {
        let ui_copy = ui.clone();
        let ui_weak = ui.as_weak();
        ui.on_dither(move || {
            let value = ui_weak.clone();
            std::thread::spawn(move || {
                let image: DynamicImage =
                    image::open("/home/marc/Documents/burning_desire.png").unwrap();

                println!("{}x{}", image.width(), image.height());

                let sepia = DitherBuilder::new(image)
                    .highlights(Rgb([255, 230, 177]))
                    .shadows(Rgb([25, 25, 112]))
                    .level(2)
                    .generate();

                println!("{}x{}", sepia.width(), sepia.height());

                // Convert to RGBA8 pixel buffer (which is Send)
                let rgba_image = sepia.to_rgba8();
                let width = rgba_image.width();
                let height = rgba_image.height();
                let pixel_data = rgba_image.into_raw();

                // Create SharedPixelBuffer in the background thread
                let mut pixel_buffer = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::new(width, height);
                pixel_buffer.make_mut_bytes().copy_from_slice(&pixel_data);

                // Update UI on the main thread
                slint::invoke_from_event_loop(move || {
                    if let Some(ui) = value.upgrade() {
                        let image = slint::Image::from_rgba8_premultiplied(pixel_buffer);
                        ui.set_image(image);
                    }
                }).unwrap();
            });
        });
    }

    ui.run()?;
    Ok(())
}
