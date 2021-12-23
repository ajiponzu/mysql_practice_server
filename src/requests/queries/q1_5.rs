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
    min_ranking: i32,
    #[allow(dead_code)]
    max_ranking: i32,
  }

  impl QueryableByName<DB> for Answer {
    fn build<R: diesel::row::NamedRow<diesel::mysql::Mysql>>(
      row: &R,
    ) -> diesel::deserialize::Result<Self> {
      Ok(Answer {
        min_ranking: row.get("Min(ranking)")?,
        max_ranking: row.get("Max(ranking)")?,
      })
    }
  }

  let connection = establish_connection();
  let answers: Vec<Answer> = diesel::sql_query(
    "
    Select Min(ranking), Max(ranking)
    from countries Group By group_name;
    ",
  )
  .load(&connection)
  .unwrap();
  format!("{:?}", answers)
}

fn query_2() -> String {
  String::from("not implemented2.")
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
    assert_eq!(query(&1), "not implemented1.");
    assert_eq!(query(&2), "not implemented2.");
    assert_eq!(query(&3), "not implemented3.");
    assert_eq!(query(&4), "not implemented4.");
    assert_eq!(query(&5), "not implemented5.");
  }
}
