#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use eframe::egui::Ui;
use guitar_chords_egui_v1::chords::possible_chords;
use guitar_chords_egui_v1::guitar::Guitar;
use guitar_chords_egui_v1::notes::*;
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
    selected_tab: GuitarChordsTabs,
}

impl Default for GuitarChordsApp {
    fn default() -> Self {
        Self {
            guitar: Guitar::guitar_6_string_standard(),
            frets_selected: vec![None; 6],
            selected_tab: GuitarChordsTabs::ChordIdentifier,
        }
    }
}

impl eframe::App for GuitarChordsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Guitar Chords");
            ui.separator();
            self.tabs(ui);
            ui.separator();

            match self.selected_tab {
                GuitarChordsTabs::ChordIdentifier => self.chord_identifier(ui),
                GuitarChordsTabs::ChordFinder => self.chord_finder(ui),
                GuitarChordsTabs::CustomizeGuitar => self.customize_guitar(ui),
            }
        });
    }
}

impl GuitarChordsApp {
    fn tabs(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let chord_identifier_tab = ui.selectable_label(
                self.selected_tab == GuitarChordsTabs::ChordIdentifier,
                "Chord Identifier",
            );
            if chord_identifier_tab.clicked() {
                self.selected_tab = GuitarChordsTabs::ChordIdentifier;
            }

            ui.separator();

            let chord_finder_tab =
                ui.selectable_label(self.selected_tab == GuitarChordsTabs::ChordFinder, "Chord Finder");
            if chord_finder_tab.clicked() {
                self.selected_tab = GuitarChordsTabs::ChordFinder;
            }

            ui.separator();

            let customize_guitar_tab = ui.selectable_label(
                self.selected_tab == GuitarChordsTabs::CustomizeGuitar,
                "Customize Guitar",
            );
            if customize_guitar_tab.clicked() {
                self.selected_tab = GuitarChordsTabs::CustomizeGuitar;
            }
        });
    }

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

    fn chord_finder(&mut self, ui: &mut Ui) {
        ui.label("TODO: Chord finder");
        // TODO
    }

    fn customize_guitar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let guitar_6_string_standard_button = ui.selectable_label(
                self.guitar == Guitar::guitar_6_string_standard(),
                "guitar 6 string standard",
            );
            if guitar_6_string_standard_button.clicked() && self.guitar != Guitar::guitar_6_string_standard() {
                self.guitar = Guitar::guitar_6_string_standard();
                self.frets_selected = vec![None; 6];
            }

            let bass_4_string_standard = ui.selectable_label(
                self.guitar == Guitar::bass_4_string_standard(),
                "bass 4 string standard",
            );
            if bass_4_string_standard.clicked() && self.guitar != Guitar::bass_4_string_standard() {
                self.guitar = Guitar::bass_4_string_standard();
                self.frets_selected = vec![None; 4];
            }

            let bass_5_string_standard_button = ui.selectable_label(
                self.guitar == Guitar::bass_5_string_standard(),
                "bass 5 string standard",
            );
            if bass_5_string_standard_button.clicked() && self.guitar != Guitar::bass_5_string_standard() {
                self.guitar = Guitar::bass_5_string_standard();
                self.frets_selected = vec![None; 5];
            }
        });

        ui.separator();

        for string in 0..self.guitar.guitar_strings.len() {
            if string >= self.guitar.guitar_strings.len() {
                break;
            }
            ui.horizontal(|ui| {
                ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);

                if ui.button("x").clicked() {
                    self.guitar.guitar_strings.remove(string);
                    self.frets_selected.remove(string);
                    return;
                }

                if ui.button("-").clicked() {
                    let s = self.guitar.guitar_strings[string].tuning;
                    self.guitar.guitar_strings[string].tuning = notes_sub(s, 1);
                }

                let tuning = &mut self.guitar.guitar_strings[string].tuning;

                egui::ComboBox::from_id_salt(string)
                    .selected_text(&note_name(*tuning))
                    .width(40.0)
                    .show_ui(ui, |ui| {
                        for note in 0..12 {
                            ui.selectable_value(tuning, note, note_name(note));
                        }
                    });

                if ui.button("+").clicked() {
                    let s = self.guitar.guitar_strings[string].tuning;
                    self.guitar.guitar_strings[string].tuning = notes_add(s, 1);
                }

                for fret in 0..=12 {
                    ui.separator();
                    ui.label(note_button_label(self.guitar.fret_to_note(string, fret)));
                }
            });
        }

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Add string (fifth)").clicked() {
                self.guitar.add_string(note_fifth, E);
                self.frets_selected.push(None);
            }

            if ui.button("Add string (fourth)").clicked() {
                self.guitar.add_string(note_fourth, A);
                self.frets_selected.push(None);
            }
        });
    }
}

#[derive(PartialEq)]
enum GuitarChordsTabs {
    ChordIdentifier,
    ChordFinder,
    CustomizeGuitar,
}
