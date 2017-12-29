use self::state::State;
use self::note::{Note, Octave, Tonality};

mod note;
mod state;

lazy_static! {
    static ref EXERSICES: Vec<Exercise> = {
        let mut v = Vec::new();

        let mut octaves = Vec::new();
        octaves.push(Octave::First);
        v.push(Exercise { num: 1, octaves: octaves });

        let mut octaves = Vec::new();
        octaves.push(Octave::First);
        octaves.push(Octave::Second);
        v.push(Exercise { num: 2, octaves: octaves });

        let mut octaves = Vec::new();
        octaves.push(Octave::Small);
        octaves.push(Octave::First);
        v.push(Exercise { num: 3, octaves: octaves });

        let mut octaves = Vec::new();
        octaves.push(Octave::First);
        octaves.push(Octave::Second);
        octaves.push(Octave::Third);
        v.push(Exercise { num: 4, octaves: octaves });

        let mut octaves = Vec::new();
        octaves.push(Octave::Great);
        octaves.push(Octave::Small);
        octaves.push(Octave::First);
        v.push(Exercise { num: 5, octaves: octaves });

        let mut octaves = Vec::new();
        octaves.push(Octave::Small);
        octaves.push(Octave::First);
        octaves.push(Octave::Second);
        octaves.push(Octave::Third);
        v.push(Exercise { num: 6, octaves: octaves });

        let mut octaves = Vec::new();
        octaves.push(Octave::Great);
        octaves.push(Octave::Small);
        octaves.push(Octave::First);
        octaves.push(Octave::Second);
        octaves.push(Octave::Third);
        v.push(Exercise { num: 7, octaves: octaves });

        v
    };
}

#[derive(Debug)]
pub struct Exercise {
    num: u8,
    octaves: Vec<Octave>,
}

struct Controller<'a> {
    state: Option<State<'a>>,
    tonality: Option<Tonality>,
}

impl<'a> Controller<'a> {
    fn check_answer() {}

    fn play_sequence() {}

    fn play_note(&self, note: Note) {}

    fn repeat_note(&self) {
        if let Some(ref s) = self.state {
            if let Some(n) = s.note {
                self.play_note(n);
            }
        }
    }
}

impl<'a> Controller<'a> {
    fn new() -> Controller<'a> {
        Controller {
            state: None,
            tonality: None,
        }
    }

    fn new_game(&mut self, tonality: Tonality) {
        let exersice = EXERSICES.first().unwrap();
        let state = State::new(tonality, exersice);
        self.state = Some(state);
    }

    fn load_game() {}

    fn save_game() {}
}
