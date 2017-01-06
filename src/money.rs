pub struct Money {
	pub amount: i64,
}

impl Money {
    pub fn eur(amnt: i64) -> Self {
    	Money {
    		amount: amnt
    	}
	}

	pub fn add(&mut self, amnt: i64) -> &mut Self {
		self.amount += amnt;
		self
	}

	pub fn format(self) -> String {
		let tmp = self.amount.to_string();
		let decim_point = tmp.chars().count() - 2;
		let decimals = tmp[decim_point..].to_string();
		let ints = tmp[..decim_point].to_string();
		let mut money_str: String = "$".to_string();

		money_str.push_str(&ints);
		money_str.push_str(",");
		money_str.push_str(&decimals);

		money_str
	}
}