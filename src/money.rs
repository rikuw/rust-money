pub struct Money {
	pub amount: String,
}

impl Money {
    pub fn eur(amnt: String) -> Self {
    	Money {
    		amount: amnt,
    	}
	}

	pub fn add(&mut self, amnt: String) -> &mut Self {
		let a: i32 = self.amount.parse().unwrap();
		let b: i32 = amnt.parse().unwrap();

		let c = a + b;

		self.amount = c.to_string();
		self
	}
}