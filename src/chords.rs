use crate::notes::note_name;
use std::sync::LazyLock;

pub struct Chord {
    name: String,
    suffix: String,
    intervals: Vec<u8>,
}

impl Chord {
    pub fn new(name: &str, suffix: &str, intervals: Vec<u8>) -> Self {
        Self {
            name: name.to_string(),
            suffix: suffix.to_string(),
            intervals,
        }
    }

    pub fn matches(&self, intervals: &[u8]) -> bool {
        intervals == self.intervals
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn short_name(&self, note: &str) -> String {
        note.to_string() + &self.suffix
    }
}

pub fn all_chords() -> &'static Vec<Chord> {
    static ALL_CHORDS: LazyLock<Vec<Chord>> = LazyLock::new(|| {
        vec![
            // Triads
            Chord::new("Power Chord", "5", vec![0, 7]),
            Chord::new("Major Triad", "", vec![0, 4, 7]),
            Chord::new("Minor Triad", "m", vec![0, 3, 7]),
            Chord::new("Diminished Triad", "dim", vec![0, 3, 6]),
            // 7 chords
            Chord::new("Major 7", "M7", vec![0, 4, 7, 11]),
            Chord::new("Dominant 7", "7", vec![0, 4, 7, 10]),
            Chord::new("Minor 7", "m7", vec![0, 3, 7, 10]),
            Chord::new("Minor 7 Flat 5", "m7(b5)", vec![0, 3, 6, 10]),
            Chord::new("Diminished 7", "dim7", vec![0, 3, 6, 9]),
            // other chords
            Chord::new("Major 6", "6", vec![0, 4, 7, 9]),
            Chord::new("Major 6/9", "6/9", vec![0, 2, 4, 7, 9]),
            Chord::new("Major 9", "M9", vec![0, 2, 4, 7, 11]),
            Chord::new("Major 11", "M11", vec![0, 2, 4, 5, 7, 11]),
            Chord::new("Major 13", "M13", vec![0, 2, 4, 5, 7, 9, 11]),
            // TODO add more chords
        ]
    });
    &*ALL_CHORDS
}

pub fn possible_chords(notes: &[u8]) -> Vec<String> {
    let mut result = Vec::new();
    if notes.is_empty() {
        return result;
    }
    for &root in notes.iter() {
        let mut intervals = notes.iter().map(|&n| (n + 12 - root) % 12).collect::<Vec<_>>();
        intervals.sort_unstable();
        for chord in all_chords().iter() {
            if chord.matches(&intervals) {
                result.push(chord.short_name(note_name(root).as_str()) + ", " + chord.name());
            }
        }
    }

    result
}
