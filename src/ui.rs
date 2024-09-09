use eframe::egui;
use eframe::egui::{
    Button, CentralPanel, Checkbox, CollapsingHeader, Context, Grid, RichText, ScrollArea,
    TextEdit, Ui, Visuals,
};


use crate::Claui;

impl eframe::App for Claui {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if self.font_config.is_some() && !self.font_loaded {
            self.load_font(ctx);
            self.font_loaded = true;
        }
        self.update_buffer();
        self.update_thread_state();

        ctx.set_visuals(Visuals::dark());

        CentralPanel::default().show(ctx, |ui| {
            self.add_title(ui);
            self.add_options(ui);
            self.add_actions_bar(ui);
            self.add_results(ui);
        });

        // We do constant repainting while its running in order to show the output at correct timing.
        if self.func_handle.is_some() {
            ctx.request_repaint();
        }
    }
}

impl Claui {
    fn add_title(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading(&self.app_info.name);
            if let Some(version) = &self.app_info.version {
                ui.label(RichText::new(version));
            }
        });

        if let Some(about) = &self.app_info.about {
            ui.label(about);
        }

        if self.app_info.long_about.is_some()
            || self.app_info.author.is_some()
            || self.app_info.version.is_some()
        {
            CollapsingHeader::new("Info").show(ui, |ui| {
                if let Some(long_about) = &self.app_info.long_about {
                    ui.label(format!("Description: {long_about}"));
                    ui.add_space(3.0);
                }

                if let Some(author) = &self.app_info.author {
                    ui.label(format!("Author: {author}"));
                    ui.add_space(3.0);
                }

                if let Some(version) = &self.app_info.version {
                    ui.label(format!("Version: {version}"));
                }
            });
        }

        ui.add_space(4.5);
    }

    fn add_options(&mut self, ui: &mut Ui) {
        if !self.args.is_empty() {
            ui.separator();

            Grid::new("options")
                .num_columns(3)
                .min_col_width(ui.available_width() / 3.0)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Key");
                    ui.label("Value");
                    ui.label("Description");
                    ui.end_row();

                    for arg in self.args.iter() {
                        ui.label(&arg.display_name);

                        if arg.takes_value {
                            ui.add_enabled(
                                self.func_handle.is_none(),
                                TextEdit::singleline(
                                    &mut self.ui_arg_state.get_mut(&arg.name.clone()).unwrap().1,
                                )
                                .hint_text(arg.default_value.as_ref().unwrap_or(&"".to_string())),
                            );
                        } else {
                            ui.add_enabled(
                                self.func_handle.is_none(),
                                Checkbox::new(
                                    &mut self.ui_arg_state.get_mut(&arg.name.clone()).unwrap().0,
                                    "",
                                ),
                            );
                        }

                        if let Some(desc) = &arg.desc {
                            ui.label(desc);
                        }

                        ui.end_row();
                    }
                });

            ui.separator();
        }
    }

    fn add_actions_bar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui
                .add_enabled(self.func_handle.is_none(), Button::new("Run"))
                .clicked()
            {
                self.run();
            };

            if ui
                .add_enabled(self.func_handle.is_none(), Button::new("Clear"))
                .clicked()
            {
                self.buffer = String::new();
            }

            if self.func_handle.is_some() {
                ui.label("Running...");
            }
        });
    }

    fn add_results(&mut self, ui: &mut Ui) {
        ui.separator();

        ScrollArea::new([true, true])
            .stick_to_bottom(true)
            .show(ui, |ui| {
                ui.add_sized(
                    ui.available_size(),
                    TextEdit::multiline(&mut self.buffer.as_str())
                        .code_editor()
                        .cursor_at_end(true),
                )
            });
    }
    pub fn load_font(&self, ctx: &Context) {
        if let Some(font_config) = self.font_config.clone() {
            let ref font_path = font_config.font_file;
            if let Ok(font_data) = std::fs::read(font_path) {
                let mut fonts = egui::FontDefinitions::default();
                fonts.font_data.insert(
                    "custom_font".to_owned(),
                    egui::FontData::from_owned(font_data),
                );
                fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap().insert(0, "custom_font".to_owned());
                fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap().insert(0, "custom_font".to_owned());
                ctx.set_fonts(fonts);
            } else {
                eprintln!("Failed to load font file: {}", font_path);
            }
        }

    }
}
