#[allow(unused_imports)]
use crate::data_models;

use crate::mysql_connector::establish_connection;

#[allow(unused_imports)]
use crate::schema;

use diesel::deserialize::QueryableByName;
use diesel::prelude::*;

#[allow(unused_imports)]
use diesel::sql_types;

type DB = diesel::mysql::Mysql;

pub fn query(question: &u16) -> String {
  if question == &1 {
    query_1()
  } else if question == &2 {
    query_2()
  } else if question == &3 {
    query_3()
  } else if question == &4 {
    query_4()
  } else if question == &5 {
    query_5()
  } else {
    String::from("bad query")
  }
}

fn query_1() -> String {
  #[derive(Debug)]
  struct Answer {
    #[allow(dead_code)]
    group_name: String,
    #[allow(dead_code)]
    min_ranking: i32,
    #[allow(dead_code)]
    max_ranking: i32,
  }

  impl QueryableByName<DB> for Answer {
    fn build<R: diesel::row::NamedRow<diesel::mysql::Mysql>>(
      row: &R,
    ) -> diesel::deserialize::Result<Self> {
      Ok(Answer {
        group_name: row.get("group_name")?,
        min_ranking: row.get("Min(ranking)")?,
        max_ranking: row.get("Max(ranking)")?,
      })
    }
  }

  let connection = establish_connection();
  let answers: Vec<Answer> = diesel::sql_query(
    "
    Select group_name, Min(ranking), Max(ranking)
    from countries Group By group_name;
    ",
  )
  .load(&connection)
  .unwrap();

  let mut ret = String::new();
  for answer in answers {
    ret += &format!(
      "{}, {}, {}\n",
      answer.group_name, answer.min_ranking, answer.max_ranking
    );
  }
  ret
}

fn query_2() -> String {
  #[derive(Debug)]
  struct Answer {
    #[allow(dead_code)]
    avgw: i32,
    #[allow(dead_code)]
    avgh: i32,
  }

  impl QueryableByName<DB> for Answer {
    fn build<R: diesel::row::NamedRow<diesel::mysql::Mysql>>(
      row: &R,
    ) -> diesel::deserialize::Result<Self> {
      Ok(Answer {
        avgw: row.get("weight")?,
        avgh: row.get("height")?,
      })
    }
  }

  let connection = establish_connection();
  // let answers: Vec<Answer> = diesel::sql_query(
  //   "
  //   Select countries.name As country_name, Avg(players.height) As avg_height
  //   from countries
  //   Inner Join players
  //   On countries.id = players.country_id
  //   Group By countries.id;
  //   ",
  // )
  let answers: Vec<Answer> = diesel::sql_query(
    "
    Select weight, height
    From players
    Where position = 'GK';
    ",
  )
  .load(&connection)
  .unwrap();

  let mut ret = String::new();
  for answer in answers {
    ret += &format!("{}, {}\n", answer.avgw, answer.avgh);
  }
  ret
}

fn query_3() -> String {
  String::from("not implemented3.")
}

fn query_4() -> String {
  String::from("not implemented4.")
}

fn query_5() -> String {
  String::from("not implemented5.")
}

#[cfg(test)]
mod tests {
  use super::query; // このモジュールにとってはモジュール外がsuperなので, super::使用したい関数名

  #[test]
  fn test() {
    assert_eq!(query(&0), "bad query");
    assert_ne!(query(&1), "not implemented1.");
    assert_ne!(query(&2), "not implemented2.");
    assert_eq!(query(&3), "not implemented3.");
    assert_eq!(query(&4), "not implemented4.");
    assert_eq!(query(&5), "not implemented5.");
  }

  #[test]
  fn print_test() {
    println!("{}", query(&2));
  }
}
