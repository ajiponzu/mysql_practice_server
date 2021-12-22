table! {
    countries (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        ranking -> Nullable<Integer>,
        group_name -> Nullable<Varchar>,
    }
}

table! {
    goals (id) {
        id -> Integer,
        pairing_id -> Nullable<Integer>,
        player_id -> Nullable<Integer>,
        goal_time -> Nullable<Varchar>,
    }
}

table! {
    goals_tmp (my_country, enemy_country, goal_time) {
        my_country -> Varchar,
        enemy_country -> Varchar,
        goal_time -> Varchar,
        player_name -> Nullable<Varchar>,
    }
}

table! {
    pairings (id) {
        id -> Integer,
        kickoff -> Nullable<Datetime>,
        my_country_id -> Nullable<Integer>,
        enemy_country_id -> Nullable<Integer>,
    }
}

table! {
    pairings_tmp (kickoff, my_country) {
        kickoff -> Datetime,
        my_country -> Varchar,
        enemy_country -> Nullable<Varchar>,
    }
}

table! {
    players (id) {
        id -> Integer,
        country_id -> Nullable<Integer>,
        uniform_num -> Nullable<Integer>,
        position -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        club -> Nullable<Varchar>,
        birth -> Nullable<Date>,
        height -> Nullable<Integer>,
        weight -> Nullable<Integer>,
    }
}

table! {
    players_tmp (uniform_num, club, name) {
        country -> Nullable<Varchar>,
        uniform_num -> Integer,
        position -> Nullable<Varchar>,
        name -> Varchar,
        club -> Varchar,
        birth -> Nullable<Date>,
        height -> Nullable<Integer>,
        weight -> Nullable<Integer>,
    }
}

allow_tables_to_appear_in_same_query!(
    countries,
    goals,
    goals_tmp,
    pairings,
    pairings_tmp,
    players,
    players_tmp,
);
