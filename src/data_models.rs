use crate::schema::*;
use diesel::mysql::types::Datetime;
use diesel::sql_types::Date;

/* Database Read Structures */
#[derive(Debug, Queryable)]
pub struct Countries {
  pub id: i32,
  pub name: String,
  pub ranking: i32,
  pub group_name: String,
}

#[derive(Debug, Queryable)]
pub struct Goals {
  pub id: i32,
  pub pairing_id: i32,
  pub player_id: i32,
  pub goal_time: String,
}

#[derive(Debug, Queryable)]
pub struct GoalsTmp {
  pub my_country: String,
  pub enemy_country: String,
  pub goal_time: String,
  pub player_time: String,
}

#[derive(Debug, Queryable)]
pub struct Pairings {
  pub id: i32,
  pub kickoff: Datetime,
  pub my_country_id: i32,
  pub enemy_country_id: i32,
}

#[derive(Debug, Queryable)]
pub struct PairingsTmp {
  pub kickoff: Datetime,
  pub my_country: String,
  pub enemy_country: String,
}

#[derive(Debug, Queryable)]
pub struct Players {
  pub id: i32,
  pub country_id: i32,
  pub uniform_num: i32,
  pub position: String,
  pub name: String,
  pub club: String,
  pub birth: Date,
  pub height: i32,
  pub weight: i32,
}

#[derive(Debug, Queryable)]
pub struct PlayersTmp {
  pub country: String,
  pub uniform_num: i32,
  pub position: String,
  pub name: String,
  pub club: String,
  pub birth: Date,
  pub height: i32,
  pub weight: i32,
}
/* end */
