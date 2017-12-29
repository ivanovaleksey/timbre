use ears::{AudioController, Sound};
use std::thread;
use std::sync::mpsc;

use self::state::State;
use self::note::{Note, Octave, Tonality};

pub mod note;
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

type Sample = String;

#[derive(Debug)]
pub struct Controller<'a> {
    gramophone: mpsc::Sender<Sample>,
    state: Option<State<'a>>,
    tonality: Option<Tonality>,
}

impl<'a> Controller<'a> {
    pub fn new() -> Controller<'a> {
        let (tx, rx) = mpsc::channel::<Sample>();

        thread::spawn(move || {
            for path in rx.iter() {
                let sample = &path;
                let mut snd = Sound::new(sample).unwrap();
                snd.play();
                while snd.is_playing() {}
            }
        });

        Controller {
            gramophone: tx,
            state: None,
            // TODO: is it required?
            tonality: None,
        }
    }

    pub fn new_game(&mut self, tonality: Tonality) {
        let exersice = EXERSICES.first().unwrap();
        let state = State::new(tonality, exersice);
        self.state = Some(state);
    }

    fn load_game() {}

    fn save_game() {}
}

impl<'a> Controller<'a> {
    fn check_answer() {}

    fn play_sample(&self, sample: Sample) {
        println!("{}\n", sample);
        self.gramophone.send(sample);
    }

    pub fn play_sequence(&self) {
        self.play_sample("/Users/aleksey/Downloads/Timbre/Cmaj.ogg".to_string());
    }

    pub fn play_note(&mut self, note: Option<Note>) {
        println!("NOTE: {:?}", note);

        match note {
            Some(n) => self.play_sample(n.sample()),
            None => {
                let note = match self.state {
                    Some(ref mut s) => s.next_note(),
                    None => None
                };

                match note {
                    Some(n) => {
                        println!("NEXT NOTE: {:?}", n);
                        self.play_sample(n.sample());
                    },
                    None => println!("Nothing to play\n"),
                }
            }
        }
    }

    pub fn repeat_note(&self) {
        self.current_note().map(|note| {
            println!("REPEAT NOTE: {:?}", note);
            self.play_sample(note.sample())}
        );
    }

    fn current_note(&self) -> Option<Note> {
        match self.state {
            Some(ref state) => state.note,
            None => None
        }
    }
}
