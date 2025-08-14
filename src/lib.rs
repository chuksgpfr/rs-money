mod arithmetic;
mod money_enum;

use std::fmt::Display;
use num_traits::Num;
use crate::money_enum::MoneyNum;

pub struct Money<T: MoneyNum> {
  pub amount: T,
  pub currency: Currency
}

pub trait MoneyTrait<T: MoneyNum> {
  fn new(amount: T, currency: Currency) -> Self;
  fn display(&self) -> String;
  fn add(&mut self, amount: T) -> &Self;
  fn subtract(&mut self, amount: T) -> &Self;
  fn split(&self, ways: u32) -> Vec<Money<T>>;
}

impl<T: MoneyNum> MoneyTrait<T> for Money<T> {
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

  fn add(&mut self, amount: T) -> &Self {
      let result = arithmetic::math::add(self.amount, amount);
      self.amount = result;
      return self;
  }

  fn subtract(&mut self, amount: T) -> &Self {
      let result = arithmetic::math::subtract(self.amount, amount);
      self.amount = result;
      return self;
  }

  fn split(&self, ways: u32) -> Vec<Money<T>> {
    let money = self;
    
    // Placeholder implementation, update as needed
    Vec::new()
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
