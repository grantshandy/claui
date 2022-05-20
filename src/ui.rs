use eframe::egui::{
    Button, CentralPanel, Checkbox, CollapsingHeader, Context, Grid, RichText, ScrollArea,
    TextEdit, Ui, Visuals,
};

use crate::{misc::capitalize, Claui};

impl eframe::App for Claui {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        self.update_buffer();
        self.update_thread_state();

        ctx.set_visuals(Visuals::dark());

        CentralPanel::default().show(ctx, |ui| {
            self.add_title(ui);
            self.add_options(ui);
            self.add_actions_bar(ui);
            self.add_results(ui);
        });

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());

        // We do constant repainting while its running in order to show the output at correct timing.
        if self.is_running {
            ctx.request_repaint();
        }
    }
}

impl Claui {
    fn add_options(&mut self, ui: &mut Ui) {
        if self.args.len() > 0 {
            ui.separator();

            Grid::new("options")
                .num_columns(3)
                .min_col_width(ui.available_width() / 4.0)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Key");
                    ui.label("Value");
                    ui.label("Description");
                    ui.end_row();

                    for arg in self.args.iter() {
                        ui.label(capitalize(&arg.name));

                        if arg.takes_value {
                            ui.add_enabled(
                                !self.is_running,
                                TextEdit::singleline(
                                    &mut self.ui_arg_state.get_mut(&arg.name.clone()).unwrap().1,
                                )
                                .hint_text(arg.default_value.as_ref().unwrap_or(&"".to_string())),
                            );
                        } else {
                            ui.add_enabled(
                                !self.is_running,
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

    fn add_title(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading(&self.app_info.name);
            if let Some(version) = &self.app_info.ver {
                ui.label(RichText::new(version));
            }
        });

        if let Some(about) = &self.app_info.about {
            ui.label(about);
        }

        if self.app_info.long_about.is_some()
            || self.app_info.author.is_some()
            || self.app_info.ver.is_some()
        {
            CollapsingHeader::new("Info").show(ui, |ui| {
                if let Some(long_about) = &self.app_info.long_about {
                    ui.label(format!("Description: {}", long_about));
                    ui.add_space(3.0);
                }

                if let Some(author) = &self.app_info.author {
                    ui.label(format!("Author: {}", author));
                    ui.add_space(3.0);
                }

                if let Some(version) = &self.app_info.ver {
                    ui.label(format!("Version: {}", version));
                }
            });
        }

        ui.add_space(4.5);
    }

    fn add_actions_bar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui
                .add_enabled(!self.is_running, Button::new("Run"))
                .clicked()
            {
                self.run();
            };

            if ui.add_enabled(!self.is_running, Button::new("Clear")).clicked() {
                self.buffer = String::new();
            }

            if self.is_running {
                ui.label("Running...");
            }
        });
    }

    fn add_results(&mut self, ui: &mut Ui) {
        ui.separator();

        ScrollArea::new([true, true])
            .stick_to_bottom()
            .show(ui, |ui| {
                ui.add_sized(
                    ui.available_size(),
                    TextEdit::multiline(&mut self.buffer.as_str())
                        .code_editor()
                        .cursor_at_end(true),
                );
            });
    }
}
