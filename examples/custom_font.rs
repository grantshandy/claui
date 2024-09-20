#![windows_subsystem = "windows"]

use clap::Command;
use eframe::{
    egui::{FontData, FontDefinitions, FontFamily},
    NativeOptions,
};

fn main() {
    let app = Command::new("Custom Font Tester");

    claui::run_setup(
        app,
        NativeOptions::default(),
        |cc| {
            let mut fonts = FontDefinitions::default();

            // change font names to whatever is needed
            fonts
                .font_data
                .insert("方正黑体简体".into(), FontData::from_static(include_bytes!("方正黑体简体.ttf")));
            fonts
                .families
                .get_mut(&FontFamily::Monospace)
                .unwrap()
                .insert(0, "方正黑体简体".into());

            cc.egui_ctx.set_fonts(fonts);
        },
        |_| {
            println!("方正黑体简体");
        },
    )
    .unwrap();
}
