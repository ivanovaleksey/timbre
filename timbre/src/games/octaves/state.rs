use rand::{thread_rng, Rng};
use games::octaves::{Exercise, EXERCISES};

use super::note::{Gamut, Note, Tonality, GAMUTS};
use games::octaves::models::{GameState, GameStateChangeset};

#[derive(Debug)]
pub struct State {
    // Chosen tonality
    pub tonality: Tonality,
    // Current exercise
    pub exercise: Exercise,
    // Current note
    pub note: Option<Note>,
    // Notes to play
    pub notes: Vec<Note>,
    // Right answered count
    pub right_count: u8,
    // Total played count
    pub total_count: u8,
    // Answer attempts
    pub attempts_left: u8,
}

impl State {
    pub fn new(tonality: Tonality, exercise: Exercise) -> State {
        let mut state = State {
            tonality,
            exercise,
            note: None,
            notes: vec![],
            right_count: 0,
            total_count: 0,
            attempts_left: 0,
        };

        state.generate_notes();
        state
    }

    pub fn load(game_state: &GameState) -> State {
        let tonality = game_state.tonality.parse::<Tonality>().unwrap();
        let exercise = EXERCISES
            .iter()
            .find(|&ex| ex.num == game_state.exercise as u8)
            .cloned()
            .unwrap();
        let note = game_state.note.parse::<Note>().unwrap();
        let notes = game_state
            .notes
            .split(",")
            .map(|s| s.parse::<Note>().unwrap())
            .collect::<Vec<_>>();

        State {
            tonality,
            exercise,
            note: Some(note),
            notes,
            right_count: game_state.right_count as u8,
            total_count: game_state.total_count as u8,
            attempts_left: 0,
        }
    }

    // TODO: implement
    // pub fn save(&self) {}

    pub fn changeset(&self) -> GameStateChangeset {
        GameStateChangeset {
            exercise: self.exercise.num as i32,
            note: self.note.map_or("".to_owned(), |note| note.to_string()),
            notes: self.notes
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(","),
            right_count: self.right_count as i32,
            total_count: self.total_count as i32,
        }
    }

    fn generate_notes(&mut self) {
        let gamut: &Gamut = GAMUTS
            .iter()
            .find(|g| g.key == self.tonality.0)
            .expect("No gamut for this tonality");
        let octaves = &self.exercise.octaves;

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

    pub fn next_exercise(&mut self) -> Option<&'static Exercise> {
        let num = self.exercise.num + 1;
        EXERCISES.iter().find(|&ex| ex.num == num).and_then(|ex| {
            self.exercise = ex.clone();
            self.generate_notes();

            Some(ex)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::note::{Octave, Pitch};

    #[test]
    fn generate_notes_first_ex() {
        let tonality = Tonality(Pitch::C);
        let exercise = EXERCISES.iter().nth(0).cloned().unwrap();
        let state = State::new(tonality, exercise);

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
        let exercise = EXERCISES.iter().nth(1).cloned().unwrap();
        let state = State::new(tonality, exercise);

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
        let exercise = EXERCISES.iter().nth(0).cloned().unwrap();
        let mut state = State::new(tonality, exercise);

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
        let exercise = EXERCISES.iter().nth(1).cloned().unwrap();
        let mut state = State::new(tonality, exercise);

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
