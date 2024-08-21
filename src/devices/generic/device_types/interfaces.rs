use crate::devices::generic::device_types::config::ConfigurationMode;

trait TripletInterfaces {
}

pub struct Interface {
	name: String,
}

impl Interface {
	pub fn new<S: Into<String>>(name: S) -> Interface {
		Interface {
			name,
		}
	}
	pub fn up(&mut self, config: ConfigurationMode) {

	}
}

