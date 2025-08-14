use num_traits::{Num, NumCast};
use std::fmt::Display;

pub trait MoneyNum: Num + NumCast + Copy + Display {
  fn from_i64(val: i64) -> Self {
      Self::from(val).unwrap()
  }

  fn from_f64(val: f64) -> Self {
      Self::from(val).unwrap()
  }
}
impl<T: Num + NumCast + Copy + Display> MoneyNum for T {}

