use chrono::Utc;
use diesel;
use diesel::prelude::*;
use ears::{AudioController, Sound};

use std::thread;
use std::path::PathBuf;
use std::sync::mpsc;
use std::cell::RefCell;
use std::rc::Rc;

pub use self::config::Config;
use self::state::State;
use self::note::{Note, Octave, Pitch, Tonality};
use games::octaves::models::{Game, GameState, NewGame, NewGameState};
use establish_connection;
use schema::{octave_games, octave_game_states};
use xdg_dirs;

mod config;
pub mod models;
pub mod note;
mod state;

lazy_static! {
    static ref NOTES_PATH: PathBuf = xdg_dirs::SAMPLES.join("notes");
    static ref TONES_PATH: PathBuf = xdg_dirs::SAMPLES.join("tonal-centers");

    static ref EXERCISES: Vec<Exercise> = {
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
    pub num: u8,
    octaves: Vec<Octave>,
}

type Sample = String;

pub struct Controller {
    config: Config,
    gramophone: mpsc::Sender<Sample>,
    state: Option<State>,
    // tonality: Option<Tonality>,
    count_observers: Vec<Box<Fn(&Controller) -> ()>>,
    pub next_exercise_observer: Option<Box<Fn(&'static Exercise) -> ()>>,
    pub game_over_observer: Option<Box<Fn() -> ()>>,
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
            // tonality: None,
            count_observers: Vec::new(),
            next_exercise_observer: None,
            game_over_observer: None,
        }
    }

    pub fn new_shared(config: Config) -> SharedController {
        let ctrl = Controller::new(config);
        Rc::new(RefCell::new(ctrl))
    }

    pub fn new_game(&mut self, tonality: Tonality) {
        let exercise = EXERCISES.first().cloned().unwrap();
        let state = State::new(tonality, exercise);
        self.new_game_with_state(state);

        let new_game = NewGame {
            tonality: tonality.to_string(),
        };

        let conn = establish_connection();
        diesel::insert_into(octave_games::table)
            .values(&new_game)
            .execute(&conn)
            .expect("Failed to save a game");
    }

    fn new_game_with_state(&mut self, state: State) {
        self.state = Some(state);
        self.count_changed();
    }

    pub fn load_game(&mut self) -> Option<GameState> {
        let conn = establish_connection();
        let game = Controller::current_game()?;

        let game_state = GameState::belonging_to(&game)
            .first::<GameState>(&conn)
            .optional()
            .unwrap();

        if let Some(ref s) = game_state {
            let state = State::load(s);
            self.new_game_with_state(state);
        }

        game_state
    }

    pub fn finish_game(&self) -> Game {
        let conn = establish_connection();
        let game = Controller::current_game().unwrap();

        diesel::update(octave_games::table)
            .set(octave_games::finished_at.eq(Utc::now().naive_utc()))
            .filter(octave_games::id.eq(game.id))
            .execute(&conn)
            .unwrap();

        octave_games::table.find(game.id).get_result(&conn).unwrap()
    }

    pub fn save_state(&self) {
        if let Some(ref state) = self.state {
            let conn = establish_connection();
            let game = Controller::current_game().unwrap();

            let game_state = GameState::belonging_to(&game)
                .first::<GameState>(&conn)
                .optional()
                .unwrap();
            match game_state {
                Some(current_state) => {
                    let changeset = state.changeset();

                    diesel::update(&current_state)
                        .set(&changeset)
                        .execute(&conn)
                        .unwrap();
                }
                None => {
                    let changeset = state.changeset();
                    let new_state = NewGameState {
                        tonality: state.tonality.to_string(),
                        exercise: changeset.exercise,
                        note: changeset.note,
                        notes: changeset.notes,
                        right_count: changeset.right_count,
                        total_count: changeset.total_count,
                        game_id: game.id,
                    };

                    diesel::insert_into(octave_game_states::table)
                        .values(&new_state)
                        .execute(&conn)
                        .unwrap();
                }
            }
        }
    }

    fn current_game() -> Option<Game> {
        let conn = establish_connection();
        octave_games::table
            .filter(octave_games::finished_at.is_null())
            .order(octave_games::created_at.desc())
            .first::<Game>(&conn)
            .optional()
            .unwrap()
    }
}

impl Controller {
    pub fn check_answers(&mut self, answers: &[&str]) -> Option<bool> {
        match self.state {
            Some(ref mut s) => {
                if s.attempts_left == 0 {
                    return None;
                }
                s.attempts_left -= 1;
            }
            None => unreachable!(),
        }

        let res = answers.iter().any(|a| self.check_answer(a));
        Some(res)
    }

    fn check_answer(&mut self, answer: &str) -> bool {
        let answer = answer.parse::<Pitch>().unwrap();

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
            None => unreachable!(),
        }
    }

    fn play_sample(&self, sample: Sample) {
        println!("{}\n", sample);
        self.gramophone
            .send(sample)
            .expect("Failed to play a sample");
    }

    fn play_note(&self, note: Note) {
        let sample_path = format!("{}/{}.ogg", NOTES_PATH.display(), note);
        self.play_sample(sample_path);
    }

    pub fn play_tonal_center(&self) {
        if let Some(ref state) = self.state {
            let sample_path = format!("{}/IIVVIPAC - {}.ogg", TONES_PATH.display(), state.tonality);
            self.play_sample(sample_path);
        }
    }

    pub fn play_next_note(&mut self) {
        let mut note = match self.state {
            Some(ref mut s) => s.next_note(),
            None => None,
        };

        if note.is_none() {
            // Looks like this exercise is over, try to unlock the next one
            if let Some(ref mut s) = self.state {
                if s.total_count == s.right_count {
                    let exercise = s.next_exercise();
                    if let Some(ref observer) = self.next_exercise_observer {
                        if let Some(exercise) = exercise {
                            observer(exercise);
                        }
                    }
                    note = s.next_note();
                }
            }
        }

        match note {
            Some(n) => {
                println!("NEXT NOTE: {:?}", n);
                self.play_note(n);
                self.inc_total_count();
                self.grant_attempts();
            }
            None => {
                if let Some(ref observer) = self.game_over_observer {
                    self.save_state();
                    self.finish_game();

                    observer();
                }
            }
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

impl Controller {
    fn grant_attempts(&mut self) {
        if let Some(ref mut s) = self.state {
            s.attempts_left = 1;
        }
    }

    pub fn is_finished(&self) -> bool {
        match self.state {
            Some(ref state) => state.notes.is_empty(),
            None => true,
        }
    }
}
