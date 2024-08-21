use std::io::Write;
use crate::devices::generic::device_types::interfaces::Interface;

pub trait Configurable<T: Write> {
	fn enter_config(&self) -> ConfigurationMode<T>;
}

pub struct ConfigurationMode<'a, T: Write> {
	session: &'a mut T,
}

impl<'a, T: Write> ConfigurationMode<'a, T> {
	pub fn get_interface(&self, name: &str, indices: &[u32]) -> Interface {
		Interface::new(format!("{}{}", name, indices.join("/")))
	}
}

impl<T: Write> ConfigurationMode<T> {
	pub fn enter(session: &mut T) -> ConfigurationMode<T> {
		ConfigurationMode {
			session,
		}
	}
}
