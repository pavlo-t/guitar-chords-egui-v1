#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use cpal::traits::StreamTrait;
use eframe::egui;
use eframe::egui::Ui;
use guitar_chords_egui_v1::audio::{
    play_frequency_sawtooth, play_frequency_sine_plus_harmonics, play_frequency_sine_wave,
    playe_frequency_karplus_strong,
};
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
    audio_streams: Vec<cpal::Stream>,
}

impl Default for GuitarChordsApp {
    fn default() -> Self {
        Self {
            guitar: Guitar::guitar_6_string_standard(),
            frets_selected: vec![None; 6],
            selected_tab: GuitarChordsTabs::ChordIdentifier,
            audio_streams: vec![],
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
                GuitarChordsTabs::AudioPlayback => self.audio_playback(ui),
                GuitarChordsTabs::ChordFinder => self.chord_finder(ui),
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

            let audio_playback =
                ui.selectable_label(self.selected_tab == GuitarChordsTabs::AudioPlayback, "Audio Playback");
            if audio_playback.clicked() {
                self.selected_tab = GuitarChordsTabs::AudioPlayback;
            }

            ui.separator();

            let chord_finder_tab =
                ui.selectable_label(self.selected_tab == GuitarChordsTabs::ChordFinder, "Chord Finder");
            if chord_finder_tab.clicked() {
                self.selected_tab = GuitarChordsTabs::ChordFinder;
            }
        });
    }

    fn chord_identifier(&mut self, ui: &mut Ui) {
        self.pick_guitar(ui);
        ui.separator();

        for string in 0..self.guitar.guitar_strings.len() {
            if string >= self.guitar.guitar_strings.len() {
                break;
            }
            ui.horizontal(|ui| {
                ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);

                if ui.button("X").clicked() {
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

                for fret in 0..=15 {
                    if fret > 0 {
                        ui.separator();
                    }
                    let selected_fret = self.frets_selected.get_mut(string).unwrap();
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

    fn audio_playback(&mut self, ui: &mut Ui) {
        ui.label("Note playback:");

        // TODO calculate the frequencies, A3 is 220hz:
        //  To calculate the frequency of a musical note, you can use a formula that relates it to a
        //  reference frequency (usually A4 at 440 Hz) and the number of semitones away from that reference.
        //  The formula is: f = 2^(n/12) * 440, where 'f' is the frequency you want to calculate,
        //  and 'n' is the number of semitones above (positive) or below (negative) the reference note A4.
        let notes = vec![
            ("C4", 261.63),
            ("C#4", 277.18),
            ("D4", 293.66),
            ("D#4", 311.13),
            ("E4", 329.63),
            ("F4", 349.23),
            ("F#4", 369.99),
            ("G4", 392.00),
            ("G#4", 415.30),
            ("A4", 440.00),
            ("A#4", 466.16),
            ("B4", 493.88),
        ];

        // TODO common volume

        ui.label("Sine waves:");
        ui.horizontal(|ui| {
            for (note, freq) in &notes {
                if ui.button("Play ".to_string() + note).clicked() {
                    self.audio_streams.push(play_frequency_sine_wave(*freq));
                }
            }
        });

        ui.label("Sine waves plus harmonics:");
        ui.horizontal(|ui| {
            for (note, freq) in &notes {
                if ui.button("Play ".to_string() + note).clicked() {
                    self.audio_streams.push(play_frequency_sine_plus_harmonics(*freq));
                }
            }
        });

        ui.label("Sawtooth:");
        ui.horizontal(|ui| {
            for (note, freq) in &notes {
                if ui.button("Play ".to_string() + note).clicked() {
                    self.audio_streams.push(play_frequency_sawtooth(*freq));
                }
            }
        });

        ui.label("Karplus-Strong:");
        ui.horizontal(|ui| {
            for (note, freq) in &notes {
                if ui.button("Play ".to_string() + note).clicked() {
                    self.audio_streams.push(playe_frequency_karplus_strong(*freq));
                }
            }
        });

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Stop all").clicked() {
                self.audio_streams.clear();
            }

            if ui.button("Pause all").clicked() {
                for stream in self.audio_streams.iter_mut() {
                    stream.pause().expect("pause failed");
                }
            }

            if ui.button("Un-Pause all").clicked() {
                for stream in self.audio_streams.iter_mut() {
                    stream.play().expect("play failed");
                }
            }
        });
        // TODO
    }

    fn chord_finder(&mut self, ui: &mut Ui) {
        ui.label("TODO: Chord finder");
        // TODO
    }

    fn pick_guitar(&mut self, ui: &mut Ui) {
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
    }
}

#[derive(PartialEq)]
enum GuitarChordsTabs {
    ChordIdentifier,
    AudioPlayback,
    ChordFinder,
}
