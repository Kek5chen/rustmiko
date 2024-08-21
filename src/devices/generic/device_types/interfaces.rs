pub struct Interface {
	name: String,
}

impl Interface {
	pub fn new<S: Into<String>>(name: S) -> Interface {
		Interface {
			name: name.into(),
		}
	}

	pub fn name(&self) -> &str {
		&self.name
	}
}

