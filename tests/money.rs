use rs_money::{self, Money, MoneyTrait};

#[test]
fn instantiate_money() {
  let money = Money::new(100, rs_money::USD);
  assert_eq!(money.amount, 100)
}

#[test]
fn check_currency_show() {
  let money = Money::new(100, rs_money::USD);
  assert_eq!(money.currency.show(), "USD")
}