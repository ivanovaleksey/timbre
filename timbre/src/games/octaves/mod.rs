use ears::{AudioController, Sound};
use std::thread;
use std::sync::mpsc;

pub use self::config::Config;
use self::state::State;
use self::note::{Note, Octave, Pitch, Tonality};

mod config;
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
    config: Config,
    gramophone: mpsc::Sender<Sample>,
    state: Option<State<'a>>,
    tonality: Option<Tonality>,
}

impl<'a> Controller<'a> {
    pub fn new(config: Config) -> Controller<'a> {
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
            config,
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

    // TODO: implement
    fn load_game() {}

    // TODO: implement
    fn save_game() {}
}

impl<'a> Controller<'a> {
    fn check_answer(&mut self, answer: Note) -> bool {
        match self.current_note() {
            Some(note) => {
                let right = note == answer;
                if right {
                    println!("Right!");
                    if let Some(ref mut s) = self.state {
                        s.right_count += 1;
                    }
                } else {
                    println!("Wrong!");
                }

                right
            }
            None => false,
        }
    }

    fn play_sample(&self, sample: Sample) {
        println!("{}\n", sample);
        self.gramophone.send(sample);
    }

    fn play_note(&self, note: Note) {
        let note_path = format!(
            "{}/{:?}{}.ogg",
            self.config.samples_path, note.pitch, note.octave as u8
        );
        self.play_sample(note_path);
    }

    pub fn play_sequence(&self) {
        let chord_path = format!("{}/Cmaj.ogg", self.config.samples_path);
        self.play_sample(chord_path);
    }

    pub fn play_next_note(&mut self) {
        let note = match self.state {
            Some(ref mut s) => s.next_note(),
            None => None,
        };

        match note {
            Some(n) => {
                println!("NEXT NOTE: {:?}", n);
                self.play_note(n);
                if let Some(ref mut s) = self.state {
                    s.total_count += 1;
                }
            }
            None => println!("Nothing to play\n"),
        }
    }

    pub fn repeat_note(&self) {
        self.current_note().map(|note| {
            println!("REPEAT NOTE: {:?}", note);
            self.play_note(note)
        });
    }

    fn current_note(&self) -> Option<Note> {
        match self.state {
            Some(ref state) => state.note,
            None => None,
        }
    }
}
