CREATE TABLE octave_game_states (
  id INTEGER NOT NULL PRIMARY KEY,
  exercise INTEGER NOT NULL DEFAULT 1,
  note TEXT NOT NULL DEFAULT '',
  notes TEXT NOT NULL DEFAULT '',
  right_count INTEGER NOT NULL DEFAULT 0,
  total_count INTEGER NOT NULL DEFAULT 0,
  game_id INTEGER NOT NULL REFERENCES octave_games(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX octave_game_states_game_id_idx ON octave_game_states(game_id);
