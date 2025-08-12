use std::fmt::Display;
use num_traits::Num;

pub struct Money<T: Num + Copy + Display> {
  pub amount: T,
  pub currency: Currency
}

pub trait MoneyTrait<T: Num + Copy + Display> {
  fn new(amount: T, currency: Currency) -> Self;
  fn display(&self) -> String;
}

impl<T: Num + Copy + Display> MoneyTrait<T> for Money<T> {
  fn new(amount: T, currency: Currency) -> Self {
    return Money {
      amount,
      currency
    };
  }

  fn display(&self) -> String {
    let res = String::new();
    return res;
  }
}

pub enum Currency {
  USD(&'static str),
}

impl Currency {
  pub fn show(self) -> &'static str {
    let currency_str = match self {
            Currency::USD(code) => code,
            // Add other variants here if needed
        };
    return currency_str;
  }
}


pub const USD: Currency = Currency::USD("USD");
