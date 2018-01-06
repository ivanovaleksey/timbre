use rand::{thread_rng, Rng};
use super::Exercise;
use super::note::{Gamut, Note, Tonality, GAMUTS};

#[derive(Debug)]
pub struct State {
    // Chosen tonality
    tonality: Tonality,
    // Current exersice
    exersice: Exercise,
    // Current note
    pub note: Option<Note>,
    // Notes to play
    notes: Vec<Note>,
    // Right answered count
    pub right_count: u8,
    // Total played count
    pub total_count: u8,
}

impl State {
    pub fn new(tonality: Tonality, exersice: Exercise) -> State {
        let mut state = State {
            tonality,
            exersice,
            note: None,
            notes: Vec::new(),
            right_count: 0,
            total_count: 0,
        };

        state.generate_notes();
        state
    }

    // TODO: implement
    // pub fn load() -> State {}

    // TODO: implement
    // pub fn save(&self) {}

    fn generate_notes(&mut self) {
        let gamut: &Gamut = GAMUTS
            .iter()
            .find(|g| g.key == self.tonality.0)
            .expect("No gamut for this tonality");
        let octaves = &self.exersice.octaves;

        for octave in octaves.iter() {
            for pitch in gamut.scale.iter() {
                self.notes.push(Note {
                    octave: *octave,
                    pitch: *pitch,
                });
            }

            if let Some(next_octave) = octave.next() {
                self.notes.push(Note {
                    octave: next_octave,
                    pitch: gamut.key,
                });
            }
        }

        self.notes.dedup();
    }

    pub fn next_note(&mut self) -> Option<Note> {
        println!("next note is called");
        self.drop_note();
        self.note = thread_rng().choose(&self.notes).map(|n| *n);
        self.note
    }

    fn drop_note(&mut self) {
        if let Some(note) = self.note {
            let index = self.notes.iter().position(|&n| n == note);
            self.notes.remove(index.unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::EXERSICES;
    use super::super::note::{Octave, Pitch};

    #[test]
    fn generate_notes_first_ex() {
        let tonality = Tonality(Pitch::C);
        let exersice = EXERSICES.iter().nth(0).unwrap();
        let state = State::new(tonality, exersice);

        let notes = [
            Note {
                octave: Octave::First,
                pitch: Pitch::C,
            },
            Note {
                octave: Octave::First,
                pitch: Pitch::D,
            },
            Note {
                octave: Octave::First,
                pitch: Pitch::E,
            },
            Note {
                octave: Octave::First,
                pitch: Pitch::F,
            },
            Note {
                octave: Octave::First,
                pitch: Pitch::G,
            },
            Note {
                octave: Octave::First,
                pitch: Pitch::A,
            },
            Note {
                octave: Octave::First,
                pitch: Pitch::B,
            },
            Note {
                octave: Octave::Second,
                pitch: Pitch::C,
            },
        ];

        assert_eq!(state.notes, notes.to_vec());
    }

    #[test]
    fn generate_notes_second_ex() {
        let tonality = Tonality(Pitch::C);
        let exersice = EXERSICES.iter().nth(1).unwrap();
        let state = State::new(tonality, exersice);

        let notes = [
            Note {
                octave: Octave::First,
                pitch: Pitch::C,
            },
            Note {
                octave: Octave::First,
                pitch: Pitch::D,
            },
            Note {
                octave: Octave::First,
                pitch: Pitch::E,
            },
            Note {
                octave: Octave::First,
                pitch: Pitch::F,
            },
            Note {
                octave: Octave::First,
                pitch: Pitch::G,
            },
            Note {
                octave: Octave::First,
                pitch: Pitch::A,
            },
            Note {
                octave: Octave::First,
                pitch: Pitch::B,
            },
            Note {
                octave: Octave::Second,
                pitch: Pitch::C,
            },
            Note {
                octave: Octave::Second,
                pitch: Pitch::D,
            },
            Note {
                octave: Octave::Second,
                pitch: Pitch::E,
            },
            Note {
                octave: Octave::Second,
                pitch: Pitch::F,
            },
            Note {
                octave: Octave::Second,
                pitch: Pitch::G,
            },
            Note {
                octave: Octave::Second,
                pitch: Pitch::A,
            },
            Note {
                octave: Octave::Second,
                pitch: Pitch::B,
            },
            Note {
                octave: Octave::Third,
                pitch: Pitch::C,
            },
        ];

        assert_eq!(state.notes, notes.to_vec());
    }

    #[test]
    fn next_note_first_ex() {
        let tonality = Tonality(Pitch::C);
        let exersice = EXERSICES.iter().nth(0).unwrap();
        let mut state = State::new(tonality, exersice);

        assert!(state.note.is_none());

        let note_1 = state.next_note().unwrap();
        let note_2 = state.next_note().unwrap();
        let note_3 = state.next_note().unwrap();
        let note_4 = state.next_note().unwrap();
        let note_5 = state.next_note().unwrap();
        let note_6 = state.next_note().unwrap();
        let note_7 = state.next_note().unwrap();
        let note_8 = state.next_note().unwrap();

        let note_9 = state.next_note();
        assert!(note_9.is_none());

        let mut notes = [
            note_1, note_2, note_3, note_4, note_5, note_6, note_7, note_8
        ];

        let expected_notes = [
            Note {
                octave: Octave::First,
                pitch: Pitch::C,
            },
            Note {
                octave: Octave::First,
                pitch: Pitch::D,
            },
            Note {
                octave: Octave::First,
                pitch: Pitch::E,
            },
            Note {
                octave: Octave::First,
                pitch: Pitch::F,
            },
            Note {
                octave: Octave::First,
                pitch: Pitch::G,
            },
            Note {
                octave: Octave::First,
                pitch: Pitch::A,
            },
            Note {
                octave: Octave::First,
                pitch: Pitch::B,
            },
            Note {
                octave: Octave::Second,
                pitch: Pitch::C,
            },
        ];

        assert_ne!(notes, expected_notes);
        notes.sort();
        assert_eq!(notes, expected_notes);
    }

    #[test]
    fn next_note_second_ex() {
        let tonality = Tonality(Pitch::C);
        let exersice = EXERSICES.iter().nth(1).unwrap();
        let mut state = State::new(tonality, exersice);

        assert!(state.note.is_none());

        let note_1 = state.next_note().unwrap();
        let note_2 = state.next_note().unwrap();
        let note_3 = state.next_note().unwrap();
        let note_4 = state.next_note().unwrap();
        let note_5 = state.next_note().unwrap();
        let note_6 = state.next_note().unwrap();
        let note_7 = state.next_note().unwrap();
        let note_8 = state.next_note().unwrap();
        let note_9 = state.next_note().unwrap();
        let note_10 = state.next_note().unwrap();
        let note_11 = state.next_note().unwrap();
        let note_12 = state.next_note().unwrap();
        let note_13 = state.next_note().unwrap();
        let note_14 = state.next_note().unwrap();
        let note_15 = state.next_note().unwrap();

        let note_16 = state.next_note();
        assert!(note_16.is_none());

        let mut notes = [
            note_1, note_2, note_3, note_4, note_5, note_6, note_7, note_8, note_9, note_10,
            note_11, note_12, note_13, note_14, note_15,
        ];

        let expected_notes = [
            Note {
                octave: Octave::First,
                pitch: Pitch::C,
            },
            Note {
                octave: Octave::First,
                pitch: Pitch::D,
            },
            Note {
                octave: Octave::First,
                pitch: Pitch::E,
            },
            Note {
                octave: Octave::First,
                pitch: Pitch::F,
            },
            Note {
                octave: Octave::First,
                pitch: Pitch::G,
            },
            Note {
                octave: Octave::First,
                pitch: Pitch::A,
            },
            Note {
                octave: Octave::First,
                pitch: Pitch::B,
            },
            Note {
                octave: Octave::Second,
                pitch: Pitch::C,
            },
            Note {
                octave: Octave::Second,
                pitch: Pitch::D,
            },
            Note {
                octave: Octave::Second,
                pitch: Pitch::E,
            },
            Note {
                octave: Octave::Second,
                pitch: Pitch::F,
            },
            Note {
                octave: Octave::Second,
                pitch: Pitch::G,
            },
            Note {
                octave: Octave::Second,
                pitch: Pitch::A,
            },
            Note {
                octave: Octave::Second,
                pitch: Pitch::B,
            },
            Note {
                octave: Octave::Third,
                pitch: Pitch::C,
            },
        ];

        assert_ne!(notes, expected_notes);
        notes.sort();
        assert_eq!(notes, expected_notes);
    }
}
