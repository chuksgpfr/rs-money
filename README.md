
**RustMoney** provides ability to work with [monetary value using a currency's smallest unit](https://martinfowler.com/eaaCatalog/money.html).

This package provides basic and precise Money operations such as rounding, splitting and allocating.  Monetary values should not be stored as floats due to small rounding differences.

This is a simple example usage:

```rs
use rs_money::{Money, MoneyTrait, USD};


let amount = 100.00;
let currency = USD;
let money = Money::new(amount, currency);
```