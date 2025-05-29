use crate::notes::*;

pub struct GuitarString {
    pub tuning: Note,
}

pub struct Guitar {
    pub guitar_strings: Vec<GuitarString>,
}

impl Guitar {
    pub fn standard_6_string() -> Self {
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

    pub fn fret_to_note(&self, string: usize, fret: u8) -> Note {
        (self.guitar_strings[string].tuning + fret) % 12
    }
}
