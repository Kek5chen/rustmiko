use std::io;
use crate::devices::generic::device_types::interfaces::Interface;

pub trait Configurable {
	type SessionType: Configurable;

	fn enter_config(&mut self) -> io::Result<ConfigurationMode<Self::SessionType>>;
	fn exit(&mut self) -> io::Result<()>;
	fn save(&mut self) -> io::Result<()>;
}

pub trait InterfaceConfigurable {
	fn interface_up(&mut self, interface: &Interface) -> io::Result<()>;
	fn interface_down(&mut self, interface: &Interface) -> io::Result<()>;
}

pub struct ConfigurationMode<'a, T: Configurable> {
	pub(crate) session: &'a mut T,
}

impl<'a, T: Configurable> ConfigurationMode<'a, T> {
	pub fn get_interface(&self, name: &str, indices: &[u32]) -> Interface {
		let indices_str: String = indices.iter()
			.map(|index| index.to_string())
			.collect::<Vec<String>>()
			.join("/");
		Interface::new(format!("{}{}", name, indices_str))
	}
}

impl<'a, T: Configurable> ConfigurationMode<'a, T> {
	pub fn enter(session: &mut T) -> ConfigurationMode<T> {
		ConfigurationMode {
			session,
		}
	}
}

impl<T: Configurable> Drop for ConfigurationMode<'_, T> {
	fn drop(&mut self) {
		let _ = self.session.exit();
	}
}