pub type Note = u8;

pub const C: Note = 0;
pub const CS: Note = 1;
pub const D: Note = 2;
pub const DS: Note = 3;
pub const E: Note = 4;
pub const F: Note = 5;
pub const FS: Note = 6;
pub const G: Note = 7;
pub const GS: Note = 8;
pub const A: Note = 9;
pub const AS: Note = 10;
pub const B: Note = 11;

pub fn note_button_label(note: Note) -> String {
    match note {
        C => " C\n".to_string(),
        CS => "C#\nDb".to_string(),
        D => " D\n".to_string(),
        DS => "D#\nEb".to_string(),
        E => " E\n".to_string(),
        F => " F\n".to_string(),
        FS => "F#\nGb".to_string(),
        G => " G\n".to_string(),
        GS => "G#\nAb".to_string(),
        A => " A\n".to_string(),
        AS => "A#\nBb".to_string(),
        B => " B\n".to_string(),
        _ => unreachable!(),
    }
}

pub fn note_name(note: Note) -> String {
    match note {
        C => "C".to_string(),
        CS => "C#".to_string(),
        D => "D".to_string(),
        DS => "D#".to_string(),
        E => "E".to_string(),
        F => "F".to_string(),
        FS => "F#".to_string(),
        G => "G".to_string(),
        GS => "G#".to_string(),
        A => "A".to_string(),
        AS => "A#".to_string(),
        B => "B".to_string(),
        _ => unreachable!(),
    }
}
