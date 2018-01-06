use ears::{AudioController, Sound};
use std::thread;
use std::sync::mpsc;
use std::cell::RefCell;
use std::rc::Rc;

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

#[derive(Clone, Debug)]
pub struct Exercise {
    num: u8,
    octaves: Vec<Octave>,
}

type Sample = String;

#[derive(Debug)]
pub struct Controller {
    config: Config,
    gramophone: mpsc::Sender<Sample>,
    state: Option<State>,
    tonality: Option<Tonality>,
}

pub type SharedController = Rc<RefCell<Controller>>;

impl Controller {
    pub fn new(config: Config) -> Controller {
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

    pub fn new_shared(config: Config) -> SharedController {
        let ctrl = Controller::new(config);
        Rc::new(RefCell::new(ctrl))
    }

    pub fn new_game(&mut self, tonality: Tonality) {
        let exersice = EXERSICES.first().cloned().unwrap();
        let state = State::new(tonality, exersice);
        self.state = Some(state);
        self.tonality = Some(tonality);
    }

    // TODO: implement
    fn load_game() {}

    // TODO: implement
    fn save_game() {}
}

impl Controller {
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
        let sample_path = format!("{}/{}.ogg", self.config.notes_path, note);
        self.play_sample(sample_path);
    }

    pub fn play_tonal_center(&self) {
        let sample_path = format!(
            "{}/IIVVIPAC - {}.ogg",
            self.config.tonal_centers_path,
            self.tonality.unwrap()
        );
        self.play_sample(sample_path);
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
