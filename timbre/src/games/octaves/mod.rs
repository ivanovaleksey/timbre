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

pub struct Controller {
    config: Config,
    gramophone: mpsc::Sender<Sample>,
    state: Option<State>,
    tonality: Option<Tonality>,
    count_observers: Vec<Box<Fn(&Controller) -> ()>>,
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
            count_observers: Vec::new(),
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
        self.count_changed();
    }

    // TODO: implement
    fn load_game() {}

    // TODO: implement
    fn save_game() {}
}

impl Controller {
    pub fn check_answer(&mut self, answer: &str) -> bool {
        let answer: Pitch = Pitch::from_string(&answer);

        match self.current_note() {
            Some(note) => {
                let right = note.pitch == answer;
                if right {
                    println!("Right!");
                    self.inc_right_count();
                } else {
                    println!("Wrong!");
                }

                right
            }
            None => {
                println!("There is no current note to compare with");
                false
            }
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
                self.inc_total_count();
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

impl Controller {
    pub fn add_count_observer<F>(&mut self, f: F)
    where
        F: Fn(&Controller) -> () + 'static,
    {
        self.count_observers.push(Box::new(f));
    }

    fn count_changed(&self) {
        for f in &self.count_observers {
            f(self)
        }
    }

    pub fn right_count(&self) -> u8 {
        match self.state {
            Some(ref state) => state.right_count,
            None => 0,
        }
    }

    fn inc_right_count(&mut self) {
        if let Some(ref mut s) = self.state {
            s.right_count += 1;
        }
        self.count_changed();
    }

    pub fn total_count(&self) -> u8 {
        match self.state {
            Some(ref state) => state.total_count,
            None => 0,
        }
    }

    fn inc_total_count(&mut self) {
        if let Some(ref mut s) = self.state {
            s.total_count += 1;
        }
        self.count_changed();
    }
}
