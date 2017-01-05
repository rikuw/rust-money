pub struct Money {
	pub amount: i32,
}

impl Money {
    pub fn eur(amnt: i32) -> Self {
    	Money {
    		amount: amnt,
    	}
	}

	pub fn add(&mut self, amnt: i32) -> &mut Self {
		self.amount += amnt;
		self
	}
}