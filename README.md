# calculator-rs

Parses infix string via the [Shunting yard](https://en.wikipedia.org/wiki/Shunting_yard_algorithm) algorithm, which is then evaluated and returned as custom `Number` type.

Examples:

```rust
// Order of operations
let result = parse_expression("3 + 4 * 2 / (1 - 5)").unwrap();
println!("{result}"); // Number::Int(1)

// Fractions
let result = parse_expression("1 / 2").unwrap();
println!("{result}"); // Number::Decimal(0.5)

// Exponentiation
let result = parse_expression("2 ^ 3").unwrap();
println!("{result}"); // Number::Int(8)
```
