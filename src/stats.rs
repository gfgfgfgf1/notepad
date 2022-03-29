use std::{cell::RefCell, collections::HashMap, rc::Rc};

use eframe::egui;

use eframe::egui::text_edit::CursorRange;
use egui::*;
use plot::{Bar, BarChart, Legend, Plot};

use crate::utils::is_approx_integer;

#[derive(Default)]
pub struct StatsWindows {
    pub show_word_length: bool,
    pub show_char_count: bool,
}

#[derive(Default)]
pub struct Stats {
    pub windows: StatsWindows,

    pub word_length: HashMap<usize, usize>,
    pub chars_count: Rc<RefCell<Vec<(char, usize)>>>,
    pub last_cursor: CursorRange,
}

impl Stats {
    pub fn show(&mut self, ctx: &Context) {
        self.word_length_ui(ctx);
        self.char_count_ui(ctx);
    }

    fn word_length_ui(&mut self, ctx: &Context) {
        Window::new("Word length distribution")
            .open(&mut self.windows.show_word_length)
            .show(ctx, |ui| {
                let chart = BarChart::new(
                    self.word_length
                        .iter()
                        .map(|(&size, &number)| Bar::new(size as f64, number as f64).width(1.0))
                        .collect(),
                )
                .color(Color32::LIGHT_BLUE)
                .name("Word length");

                Plot::new("Normal Distribution Demo")
                    .legend(Legend::default())
                    .show(ui, |plot_ui| plot_ui.bar_chart(chart))
                    .response
            });
    }

    fn char_count_ui(&mut self, ctx: &Context) {
        Window::new("Character distribution")
            .open(&mut self.windows.show_char_count)
            .show(ctx, |ui| {
                let chart = BarChart::new(
                    self.chars_count
                        .borrow()
                        .iter()
                        .enumerate()
                        .map(|(i, (_, number))| Bar::new(i as f64, *number as f64).width(1.0))
                        .collect(),
                )
                .color(Color32::LIGHT_BLUE)
                .name("Character count");

                Plot::new("Normal Distribution Demo")
                    .legend(Legend::default())
                    .x_axis_formatter({
                        let chars_count = self.chars_count.clone();
                        move |val, _range| {
                            if is_approx_integer(val)
                                && val >= 0.0
                                && val < chars_count.borrow().len() as f64
                            {
                                let c = chars_count.borrow()[val as usize].0;
                                if c.is_alphanumeric() {
                                    String::from(c)
                                } else {
                                    format!("Code: '{}'", c as usize)
                                }
                            } else {
                                String::new()
                            }
                        }
                    })
                    .show(ui, |plot_ui| plot_ui.bar_chart(chart))
                    .response
            });
    }

    pub fn compute_chars_count(&mut self, content_buffer: &str) {
        let mut hashmap = HashMap::new();
        for char in content_buffer.chars() {
            *hashmap.entry(char).or_insert(0) += 1;
        }
        let mut chars_count: Vec<(char, usize)> = hashmap.into_iter().collect();
        chars_count.sort_by_key(|(_, i)| *i);
        chars_count.reverse();
        *self.chars_count.borrow_mut() = chars_count;
    }

    pub fn compute_word_length(&mut self, content_buffer: &str) {
        self.word_length.clear();
        for word in content_buffer.split_whitespace() {
            *self.word_length.entry(word.len()).or_insert(0) += 1;
        }
    }
}
