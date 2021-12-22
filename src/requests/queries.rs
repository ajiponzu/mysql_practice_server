mod q1_5;

#[derive(Debug, PartialEq)]
enum Day {
  First,
  End,
}

fn detect_day(question: &u16) -> Day {
  if question <= &5 {
    Day::First
  } else {
    Day::End
  }
}

pub fn queries(question: &u16) -> String {
  let query_day = detect_day(question);
  match query_day {
    Day::First => q1_5::query(question),
    _ => String::from("bad"),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_ne!(queries(&0), "bad"); // Debugが実装されていないとエラーが出ている構造体を用いると, 返却値の文字列がバグる -> アルファベット以外を使わないようにする
    assert_ne!(queries(&5), "bad");
    assert_eq!(queries(&6), "bad");
  }
}
