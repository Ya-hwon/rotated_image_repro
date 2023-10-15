use std::f32::consts::FRAC_PI_2;
use eframe::{
    egui::{self, Image, ImageButton},
    epaint::Vec2,
};

fn main() {
    let mut rotation_angle = 0f32;

    eframe::run_simple_native(
        "rotated image button test",
        eframe::NativeOptions::default(),
        move |ctx, _frame| {
            egui_extras::install_image_loaders(&ctx);
            egui::CentralPanel::default().show(ctx, |ui| {
                let image = {
                    let mut image = Image::new(egui::include_image!("../img.jpeg"));
                    if rotation_angle != 0f32 {
                        image = image.rotate(rotation_angle, Vec2::splat(0.5));
                    }
                    image
                };
                let img_btn = ui.add(ImageButton::new(image));
                if img_btn.clicked() {
                    rotation_angle += FRAC_PI_2;
                }
                if img_btn.secondary_clicked() {
                    rotation_angle -= FRAC_PI_2;
                }
            });
        },
    )
    .unwrap();
}
