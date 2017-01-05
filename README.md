# Money handling library for Rust
   
```rust
extern crate cash;

use cash::money::Money;

fn main() {
	let mut five_eur: Money = Money::eur(9895);
	five_eur.add(295);

	println!("Money: {}", five_eur.amount);
}
```