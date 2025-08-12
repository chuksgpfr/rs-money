use rs_money::{self, Money, MoneyTrait};

fn main() {
    let amount = 100.00;
    let currency = rs_money::USD;
    let money = Money::new(amount, currency);
    println!("Hello, world!, {:#?}", money.amount);
}
