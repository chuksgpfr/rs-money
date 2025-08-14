// use::crate::MoneyNum;

pub mod math {
  use crate::MoneyNum;
  
  pub fn add<T:MoneyNum>(a: T, b: T) -> T {
      a + b
  }
  pub fn subtract<T: MoneyNum>(a: T, b: T) -> T {
    a - b
  }

  // #[cfg(test)]
  #[test]
  fn test_addition() {
    let first: i64 = 2;
    let answer = add(first, 5);
    let expected_answer = 7;
    assert_eq!(answer, expected_answer, "Expected {} but got {}", expected_answer, answer)
  }

  #[test]
  fn test_subtraction() {
    let first: i64 = 10;
    let answer = subtract(first, 5);
    let expected_answer = 5;
    assert_eq!(answer, expected_answer, "Expected {} but got {}", expected_answer, answer)
  }
}

