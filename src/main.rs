mod notepad;
mod open_file;
mod stats;
mod utils;

fn main() {
    let app = notepad::Notepad::default();
    eframe::run_native(Box::new(app), eframe::epi::NativeOptions::default());
}
