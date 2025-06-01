#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
slint::include_modules!();
use image;
use rfd::FileDialog;
use slint::{Image as SlintImage, SharedString};
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

mod effects;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let window = AppWindow::new()?;
    let ui = Arc::new(window);

    {
        let ui_copy = ui.clone();

        ui.on_open(move || {
            if let Some(path) = FileDialog::new().pick_file() {
                ui_copy.set_imagePath(SharedString::from(path.to_str().unwrap()));
                ui_copy.set_image(SlintImage::load_from_path(&path).unwrap());
            }
        });
    }

    {
        let ui_copy = ui.clone();
        let ui_weak = ui.as_weak();

        ui.on_dither(move |dither_level, brightness_level, contrast_level| {
            let image_path = ui_copy.get_imagePath();

            if !image_path.is_empty() {
                let image = image::open(image_path).unwrap();
                let value = ui_weak.clone();
                thread::spawn(move || {
                    let contrast_level = contrast_level as f32;
                    let output =
                        effects::dither(image, dither_level, brightness_level, contrast_level)
                            .unwrap();

                    let rgba_image = output.to_rgba8();
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
                            //let image_og = SlintImage::from_rgba8_premultiplied(pixel_buffer_og);
                            //ui.set_image2(image_og);
                        }
                    })
                    .unwrap();
                });
            }
        });
    }

    {
        let ui_copy = ui.clone();

        ui.on_export(move |dither_level, brightness_level, contrast_level| {
            println!("Exporting");
            let image_str = ui_copy.get_imagePath().to_string();

            if !image_str.is_empty() {
                let image = image::open(image_str).unwrap();

                let image_path = ui_copy.get_imagePath().to_string();
                thread::spawn(move || {
                    let contrast_level = contrast_level as f32;
                    let output =
                        effects::dither(image, dither_level, brightness_level, contrast_level)
                            .unwrap();

                    let image_path = PathBuf::from(image_path);
                    if let Some(name) = image_path.file_stem() {
                        println!("Name: {}", name.to_string_lossy());

                        if let Some(path) = FileDialog::new().pick_folder() {
                            println!("Path: {}", path.to_string_lossy());
                            let path = path.join(format!("{}_dither.png", name.to_string_lossy()));
                            output.save(&path).unwrap();
                        }
                    }
                });
            }
        });
    }

    ui.run()?;
    Ok(())
}
