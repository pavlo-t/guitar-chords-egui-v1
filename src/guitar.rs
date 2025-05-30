use crate::notes::*;

#[derive(PartialEq)]
pub struct GuitarString {
    pub tuning: Note,
}

#[derive(PartialEq)]
pub struct Guitar {
    pub guitar_strings: Vec<GuitarString>,
}

impl Guitar {
    pub fn guitar_6_string_standard() -> Self {
        Self {
            guitar_strings: vec![
                GuitarString { tuning: E },
                GuitarString { tuning: B },
                GuitarString { tuning: G },
                GuitarString { tuning: D },
                GuitarString { tuning: A },
                GuitarString { tuning: E },
            ],
        }
    }

    pub fn bass_4_string_standard() -> Self {
        Self {
            guitar_strings: vec![
                GuitarString { tuning: G },
                GuitarString { tuning: D },
                GuitarString { tuning: A },
                GuitarString { tuning: E },
            ],
        }
    }

    pub fn bass_5_string_standard() -> Self {
        Self {
            guitar_strings: vec![
                GuitarString { tuning: G },
                GuitarString { tuning: D },
                GuitarString { tuning: A },
                GuitarString { tuning: E },
                GuitarString { tuning: B },
            ],
        }
    }

    pub fn fret_to_note(&self, string: usize, fret: u8) -> Note {
        (self.guitar_strings[string].tuning + fret) % 12
    }

    pub fn add_string<F>(&mut self, map_prev: F, default: Note)
    where
        F: FnOnce(Note) -> Note,
    {
        let last_tuning = self.guitar_strings.last().map(|s| s.tuning);
        let tuning = last_tuning.map(map_prev).unwrap_or(default);
        self.guitar_strings.push(GuitarString { tuning });
    }
}
