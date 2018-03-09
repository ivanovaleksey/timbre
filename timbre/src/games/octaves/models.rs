use chrono::NaiveDateTime;
use schema::{octave_game_states, octave_games};

#[derive(Identifiable, Queryable)]
#[table_name = "octave_games"]
pub struct Game {
    pub id: i32,
    pub tonality: String,
    pub created_at: NaiveDateTime,
    pub finished_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[table_name = "octave_games"]
pub struct NewGame {
    pub tonality: String,
}

#[derive(Associations, Identifiable, Queryable)]
#[belongs_to(Game)]
#[table_name = "octave_game_states"]
pub struct GameState {
    pub id: i32,
    pub exercise: i32,
    pub note: String,
    pub notes: String,
    pub right_count: i32,
    pub total_count: i32,
    pub game_id: i32,
}

#[derive(Insertable)]
#[table_name = "octave_game_states"]
pub struct NewGameState {
    pub exercise: i32,
    pub note: String,
    pub notes: String,
    pub right_count: i32,
    pub total_count: i32,
    pub game_id: i32,
}
