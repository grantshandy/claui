#![windows_subsystem = "windows"]

use clap::Command;
use eframe::{NativeOptions, egui};

// change the filename to whatever is needed
const FONT_BYTES: &[u8] = include_bytes!("方正黑体简体.ttf");

fn main() {
    let app = Command::new("Custom Font Tester");

    claui::run_setup(
        app,
        NativeOptions::default(),
        |cc| {
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "custom_font".to_owned(),
                egui::FontData::from_static(FONT_BYTES),
            );
            fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "custom_font".to_owned());
            fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().insert(0, "custom_font".to_owned());
            cc.egui_ctx.set_fonts(fonts);
        },
        |_| {
            println!("你好世界");
        },
    )
    .unwrap();
}
