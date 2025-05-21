#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init();
    // let options = eframe::NativeOptions {
    //     viewport: egui::ViewportBuilder::default(),
    //     ..Default::default()
    // };
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Guitar Chords",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    name: String,
    age: u32,
    selected: Option<u32>,
    strings: Vec<Option<u32>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            selected: None,
            strings: vec![None; 6],
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Guitar Chords");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }

            ui.separator();

            for (_string, selected_fret) in self.strings.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    for fret in 0..=24 {
                        let sl = ui.selectable_label(
                            selected_fret.is_some_and(|v| v == fret),
                            fret.to_string(),
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
                let sl1 = ui.selectable_label(self.selected.is_some_and(|v| v == 1), "1");
                if sl1.clicked() {
                    if self.selected.is_some_and(|v| v == 1) {
                        self.selected = None;
                    } else {
                        self.selected = Some(1);
                    }
                }
                let sl2 = ui.selectable_label(self.selected.is_some_and(|v| v == 2), "2");
                if sl2.clicked() {
                    self.selected = Some(2);
                }
                let sl3 = ui.selectable_label(self.selected.is_some_and(|v| v == 3), "3");
                if sl3.clicked() {
                    self.selected = Some(3);
                }
            });

            ui.separator();

            ui.label(format!(
                "Hello '{}', age {}, selected {:?}",
                self.name, self.age, self.selected
            ));
        });
    }
}
