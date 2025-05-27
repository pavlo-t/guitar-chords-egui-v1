#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use std::collections::HashSet;
use std::sync::LazyLock;

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1080.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native("Guitar Chords", options, Box::new(|_cc| Ok(Box::<MyApp>::default())))
}

struct MyApp {
    strings: Vec<Option<u8>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { strings: vec![None; 6] }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Guitar Chords");

            ui.separator();

            for (string, selected_fret) in self.strings.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);

                    for fret in 0..=24 {
                        if fret > 0 {
                            ui.separator();
                        }
                        let sl = ui.selectable_label(
                            selected_fret.is_some_and(|v| v == fret),
                            note_button_label(fret_to_note(string, fret)),
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
                .strings
                .iter()
                .enumerate()
                .filter(|(_, f)| f.is_some())
                .map(|(s, f)| fret_to_note(s, f.unwrap()))
                .collect::<HashSet<_>>();
            let mut notes = notes.into_iter().collect::<Vec<_>>();
            notes.sort_unstable();
            let notes_display = notes.iter().map(|&n| note_name(n)).collect::<Vec<_>>();
            ui.label(format!("Selected notes: {notes_display:?}"));
            ui.label(format!("Selected chord: {:#?}", possible_chords(&notes)));
        });
    }
}

fn fret_to_note(string: usize, fret: u8) -> u8 {
    let string = match string {
        0 => 4,  // E
        1 => 11, // B
        2 => 7,  // G
        3 => 2,  // D
        4 => 9,  // A
        5 => 4,  // E
        _ => unreachable!(),
    };
    (string + fret) % 12
}

fn note_button_label(note: u8) -> String {
    match note {
        0 => " C\n".to_string(),
        1 => "C#\nDb".to_string(),
        2 => " D\n".to_string(),
        3 => "D#\nEb".to_string(),
        4 => " E\n".to_string(),
        5 => " F\n".to_string(),
        6 => "F#\nGb".to_string(),
        7 => " G\n".to_string(),
        8 => "G#\nAb".to_string(),
        9 => " A\n".to_string(),
        10 => "A#\nBb".to_string(),
        11 => " B\n".to_string(),
        _ => unreachable!(),
    }
}

fn note_name(note: u8) -> String {
    match note {
        0 => "C".to_string(),
        1 => "C#".to_string(),
        2 => "D".to_string(),
        3 => "D#".to_string(),
        4 => "E".to_string(),
        5 => "F".to_string(),
        6 => "F#".to_string(),
        7 => "G".to_string(),
        8 => "G#".to_string(),
        9 => "A".to_string(),
        10 => "A#".to_string(),
        11 => "B".to_string(),
        _ => unreachable!(),
    }
}

fn possible_chords(notes: &[u8]) -> Vec<String> {
    let mut result = Vec::new();
    if notes.is_empty() {
        return result;
    }
    for &root in notes.iter() {
        let mut intervals = notes.iter().map(|&n| (n + 12 - root) % 12).collect::<Vec<_>>();
        intervals.sort_unstable();
        for chord in all_chords().iter() {
            if chord.matches(&intervals) {
                result.push(chord.short_name(note_name(root).as_str()) + ", " + chord.name.as_str());
            }
        }
    }

    result
}

struct Chord {
    name: String,
    suffix: String,
    intervals: Vec<u8>,
}

impl Chord {
    fn new(name: &str, suffix: &str, intervals: Vec<u8>) -> Self {
        Self {
            name: name.to_string(),
            suffix: suffix.to_string(),
            intervals,
        }
    }

    fn matches(&self, intervals: &[u8]) -> bool {
        intervals == self.intervals
    }

    fn short_name(&self, note: &str) -> String {
        note.to_string() + &self.suffix
    }
}

fn all_chords() -> &'static Vec<Chord> {
    static INSTANCE: LazyLock<Vec<Chord>> = LazyLock::new(|| {
        vec![
            Chord::new("Power Chord", "5", vec![0, 7]),
            Chord::new("Major Triad", "", vec![0, 4, 7]),
            Chord::new("Minor Triad", "m", vec![0, 3, 7]),
            Chord::new("Dominant Seventh", "7", vec![0, 4, 7, 10]),
            // TODO add more chords
        ]
    });
    &*INSTANCE
}
