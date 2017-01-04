# Money handling library for Rust
   
```rust
extern crate cash;

use cash::money::Money;

fn main() {
	let mut five_eur: Money = Money::eur("9895".to_string());
	five_eur.add("295".to_string());

	println!("Money: {}", five_eur.amount);
}
```