use crate::open_file::OpenFile;
use crate::stats::Stats;
use crate::utils::show_error_message;
use eframe::{egui, epi};
use egui::*;

#[derive(Default)]
pub struct Notepad {
    open_file: OpenFile,
    stats: Stats,
}

// Misc
impl Notepad {
    fn update_title(&mut self, frame: &epi::Frame) {
        if self.open_file.changed {
            frame.set_window_title(&format!("*{}", self.open_file.name))
        } else {
            frame.set_window_title(&self.open_file.name)
        }
    }

    fn handle_hotkeys(&mut self, ctx: &Context) {
        let mut input = ctx.input_mut();
        if input.consume_key(Modifiers::CTRL, Key::S) && self.can_save() {
            self.action_save()
        }
    }

    fn can_save(&self) -> bool {
        self.open_file.changed && self.open_file.path.is_some()
    }
}

// Actions
impl Notepad {
    // File
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

    // Stats
    fn action_show_word_length_stats(&mut self) {
        self.stats
            .compute_word_length(&self.open_file.content_buffer);
        self.stats.windows.show_word_length = true;
    }

    fn action_show_char_count_stats(&mut self) {
        self.stats
            .compute_chars_count(&self.open_file.content_buffer);
        self.stats.windows.show_char_count = true;
    }
}

// Ui
impl Notepad {
    fn text_editor_ui(&mut self, ui: &mut Ui) {
        ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let rows =
                    (ui.available_height() / ui.fonts().row_height(&FontId::default())) as usize;
                let text_output = TextEdit::multiline(&mut self.open_file.content_buffer)
                    .frame(false)
                    .desired_width(f32::INFINITY)
                    .desired_rows(rows)
                    .show(ui);
                self.stats.last_cursor = text_output.cursor_range.unwrap_or_default();
                if text_output.response.changed() {
                    self.open_file.changed = true;
                }
            });
    }

    fn top_menu_ui(&mut self, ui: &mut Ui) {
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
            });
            ui.menu_button("Stats", |ui| {
                if ui.button("Word length distribution").clicked() {
                    self.action_show_word_length_stats();
                    ui.close_menu();
                }
                if ui.button("Character count distribution").clicked() {
                    self.action_show_char_count_stats();
                    ui.close_menu();
                }
            });
        });
    }

    fn info_panel_ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let cursor_row = self.stats.last_cursor.primary.pcursor.paragraph + 1;
            let cursor_col = self.stats.last_cursor.primary.pcursor.offset + 1;
            // let chars_num = self.open_file.content_buffer.chars().count();
            let chars_num = self.open_file.content_buffer.len();
            let lines_num = self.open_file.content_buffer.lines().count();
            let words_num = self.open_file.content_buffer.split_whitespace().count();
            ui.label(format!("Row: {cursor_row}"));
            ui.separator();
            ui.label(format!("Column: {cursor_col}"));
            ui.separator();
            ui.label(format!("Characters: {chars_num}"));
            ui.separator();
            ui.label(format!("Words: {words_num}"));
            ui.separator();
            ui.label(format!("Lines: {lines_num}"));
        });
    }
}

// epi::App
impl epi::App for Notepad {
    fn setup(&mut self, ctx: &Context, frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
        ctx.set_pixels_per_point(1.2);
        frame.set_window_title(&self.open_file.name);
    }

    fn update(&mut self, ctx: &Context, frame: &epi::Frame) {
        self.update_title(frame);
        self.handle_hotkeys(ctx);
        self.stats.show(ctx);

        TopBottomPanel::top("top_menu").show(ctx, |ui| self.top_menu_ui(ui));
        TopBottomPanel::bottom("info_panel").show(ctx, |ui| self.info_panel_ui(ui));
        CentralPanel::default().show(ctx, |ui| self.text_editor_ui(ui));
    }

    fn name(&self) -> &str {
        "Notepad"
    }
}
