#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use eframe::egui::Ui;
use guitar_chords_egui_v1::chords::possible_chords;
use guitar_chords_egui_v1::guitar::Guitar;
use guitar_chords_egui_v1::notes::{note_button_label, note_name};
use std::collections::HashSet;

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1080.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Guitar Chords",
        options,
        Box::new(|_cc| Ok(Box::<GuitarChordsApp>::default())),
    )
}

struct GuitarChordsApp {
    guitar: Guitar,
    frets_selected: Vec<Option<u8>>,
}

impl Default for GuitarChordsApp {
    fn default() -> Self {
        Self {
            guitar: Guitar::standard_6_string(),
            frets_selected: vec![None; 6],
        }
    }
}

impl GuitarChordsApp {
    fn chord_identifier(&mut self, ui: &mut Ui) {
        for (string, selected_fret) in self.frets_selected.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);

                for fret in 0..=24 {
                    if fret > 0 {
                        ui.separator();
                    }
                    let sl = ui.selectable_label(
                        selected_fret.is_some_and(|v| v == fret),
                        note_button_label(self.guitar.fret_to_note(string, fret)),
                    );
                    if sl.clicked() {
                        if selected_fret.is_some_and(|v| v == fret) {
                            *selected_fret = None;
                        } else {
                            *selected_fret = Some(fret);
                        }
                    }
                }
            });
        }

        ui.separator();

        let notes = self
            .frets_selected
            .iter()
            .enumerate()
            .filter(|(_, f)| f.is_some())
            .map(|(s, f)| self.guitar.fret_to_note(s, f.unwrap()))
            .collect::<HashSet<_>>();
        let mut notes = notes.into_iter().collect::<Vec<_>>();
        notes.sort_unstable();
        let notes_display = notes.iter().map(|&n| note_name(n)).collect::<Vec<_>>();
        ui.label(format!("Selected notes: {notes_display:?}"));
        // TODO make the chord clickable, leading to chord finder tab
        ui.label(format!("Selected chord: {:#?}", possible_chords(&notes)));
    }
}

impl eframe::App for GuitarChordsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Guitar Chords");
            ui.separator();

            // TODO add chord finder in another tab
            // TODO support custom tunnings
            self.chord_identifier(ui);
        });
    }
}
