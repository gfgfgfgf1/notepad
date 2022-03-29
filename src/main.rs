use eframe::{egui, epi};
use egui::*;

mod open_file;
use open_file::OpenFile;

#[derive(Default)]
struct Notepad {
    open_file: OpenFile,
}

fn show_error_message(title: &str, err: &anyhow::Error) {
    rfd::MessageDialog::new()
        .set_buttons(rfd::MessageButtons::Ok)
        .set_title(title)
        .set_description(&err.to_string())
        .show();
}

impl Notepad {
    fn handle_hotkeys(&mut self, ctx: &Context) {
        let mut input = ctx.input_mut();
        if input.consume_key(Modifiers::CTRL, Key::S) && self.can_save() {
            self.action_save()
        }
    }

    fn can_save(&self) -> bool {
        self.open_file.changed && self.open_file.path.is_some()
    }

    fn action_open(&mut self) {
        let file_path = rfd::FileDialog::new().pick_file();
        if let Some(path) = file_path {
            match OpenFile::open_path(&path) {
                Ok(file) => {
                    self.open_file = file;
                }
                Err(err) => show_error_message("Failed to open file", &err),
            }
        }
    }

    fn action_save(&mut self) {
        if let Err(err) = self.open_file.save() {
            show_error_message("Failed to save file", &err);
        }
    }

    fn action_save_as(&mut self) {
        let path = rfd::FileDialog::new().save_file();
        if path.is_some() {
            self.open_file.path = path;
            if let Err(err) = self.open_file.save() {
                show_error_message("Failed to save file", &err);
            }
        }
    }

    fn show_text_editor(&mut self, ui: &mut Ui) {
        ScrollArea::horizontal()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let text_edit_response = ui.add_sized(
                    ui.available_size(),
                    TextEdit::multiline(&mut self.open_file.content_buffer).frame(false),
                );
                if text_edit_response.changed() {
                    self.open_file.changed = true;
                }
            });
    }

    fn show_top_menu(&mut self, ui: &mut Ui) {
        menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open").clicked() {
                    self.action_open();
                    ui.close_menu();
                }
                if ui
                    .add_enabled(self.can_save(), Button::new("Save"))
                    .clicked()
                {
                    self.action_save();
                    ui.close_menu();
                }
                if ui.button("Save as...").clicked() {
                    self.action_save_as();
                    ui.close_menu()
                }
            })
        });
    }
}

impl epi::App for Notepad {
    fn setup(&mut self, ctx: &Context, frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
        ctx.set_pixels_per_point(1.5);
        frame.set_window_title(&self.open_file.name);
    }

    fn update(&mut self, ctx: &Context, frame: &epi::Frame) {
        if self.open_file.changed {
            frame.set_window_title(&format!("*{}", self.open_file.name))
        } else {
            frame.set_window_title(&self.open_file.name)
        }
        self.handle_hotkeys(ctx);

        TopBottomPanel::top("top_menu").show(ctx, |ui| self.show_top_menu(ui));
        CentralPanel::default().show(ctx, |ui| self.show_text_editor(ui));
    }

    fn name(&self) -> &str {
        "Notepad"
    }
}

fn main() {
    let app = Notepad::default();
    eframe::run_native(Box::new(app), epi::NativeOptions::default());
}
