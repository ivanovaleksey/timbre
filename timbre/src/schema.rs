table! {
    octave_game_states (id) {
        id -> Integer,
        tonality -> Text,
        exercise -> Integer,
        note -> Text,
        notes -> Text,
        right_count -> Integer,
        total_count -> Integer,
        game_id -> Integer,
    }
}

table! {
    octave_games (id) {
        id -> Integer,
        tonality -> Text,
        created_at -> Timestamp,
        finished_at -> Nullable<Timestamp>,
    }
}

joinable!(octave_game_states -> octave_games (game_id));

allow_tables_to_appear_in_same_query!(octave_game_states, octave_games,);
