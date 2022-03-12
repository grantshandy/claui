use eframe::{
    egui::{Button, CentralPanel, Checkbox, Context, RichText, ScrollArea, TextEdit, Ui, CollapsingHeader},
    epi,
};

use crate::Clui;

impl epi::App for Clui {
    fn name(&self) -> &str {
        self.app_info.name.as_str()
    }

    fn update(&mut self, ctx: &Context, frame: &epi::Frame) {
        self.update_buffer();
        self.update_thread_state();

        CentralPanel::default().show(ctx, |ui| {
            self.add_title(ui);
            self.add_options(ui);
            self.add_actions_bar(ui);
            self.add_results(ui);
        });

        // Resize the native window to be just the size we need it to be:
        frame.set_window_size(ctx.used_size());
        frame.request_repaint();
    }
}

impl Clui {
    fn add_options(&mut self, ui: &mut Ui) {
        if self.args.len() > 0 {
            ui.separator();

            for arg in self.args.iter() {
                ui.columns(3, |columns| {
                    columns[0].label(&arg.name);

                    if arg.takes_value {
                        columns[1].add_enabled(
                            !self.is_running,
                            TextEdit::singleline(
                                &mut self.ui_arg_state.get_mut(&arg.name.clone()).unwrap().1,
                            ).hint_text(arg.default_value.as_ref().unwrap_or(&"".to_string())),
                        );
                    } else {
                        columns[1].add_enabled(
                            !self.is_running,
                            Checkbox::new(
                                &mut self.ui_arg_state.get_mut(&arg.name.clone()).unwrap().0,
                                "",
                            ),
                        );
                    }

                    if let Some(desc) = &arg.desc {
                        columns[2].label(desc);
                    }
                });
            }

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

        if let Some(long_about) = &self.app_info.long_about {
            CollapsingHeader::new("Description").show(ui, |ui| {
                ui.add_space(2.0);
                ui.label(long_about);
                if let Some(author) = &self.app_info.author {
                    ui.add_space(3.0);
                    ui.label(format!("Author: {}", author));  
                }
                ui.add_space(2.0);
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
