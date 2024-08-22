pub struct Interface {
	name: String,
}

/// Interface is a simple data class to convey the meaning of an interface.
impl Interface {
	/// Create a new interface with a name only.
	pub fn new<S: Into<String>>(name: S) -> Interface {
		Interface {
			name: name.into(),
		}
	}

	/// Get the name of the interface.
	pub fn name(&self) -> &str {
		&self.name
	}
}

