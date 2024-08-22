use std::io;
use crate::devices::generic::device_types::interfaces::Interface;

/// This trait describes a configurable device. It can enter a sort of configuration mode and
/// execute commands in this mode, as well as exit the config mode and save.
pub trait Configurable {
	type SessionType: Configurable;

	/// Enter the configuration mode
	fn enter_config(&mut self) -> io::Result<ConfigurationMode<Self::SessionType>>;
	/// Execute a raw command on the device, the state here can't be checked or enforced anymore.
	fn execute_raw(&mut self, command: &str) -> io::Result<()>;
	/// Exit the current state.
	fn exit(&mut self) -> io::Result<()>;
	/// Save the current running configuration to the device.
	fn save(&mut self) -> io::Result<()>;
}

/// This trait describes a device that has interfaces it can bring up or down.
pub trait InterfaceConfigurable {
	/// Configures an interface to be available.
	fn interface_up(&mut self, interface: &Interface) -> io::Result<()>;
	/// Configures an interface to not be available.
	fn interface_down(&mut self, interface: &Interface) -> io::Result<()>;
}

/// A ConfigurationMode object exists for the purpose of encapsulating the configuration mode from
/// the "normal mode". When you configure a device, usually you enter a special mode with a custom
/// command set. This will lock the session into the configure mode until the object is dropped.
///
/// ```
/// let device = DeviceTelnet::new("127.0.0.1:23");
/// {
/// 	// locks device
/// 	let config = device.enter_config()?;
/// 	let interface = config.get_interface("FastEthernet", &[0, 1]);
/// 	config.interface_up(&interface);
/// }
/// // able to use device here again after Drop
/// ```
///
pub struct ConfigurationMode<'a, T: Configurable> {
	pub(crate) session: &'a mut T,
}

impl<'a, T: Configurable> ConfigurationMode<'a, T> {
	/// Will build an interface string from a name and indices \[name]((group)/subgroup/index)
	pub fn get_interface(&self, name: &str, indices: &[u32]) -> Interface {
		let indices_str: String = indices.iter()
			.map(|index| index.to_string())
			.collect::<Vec<String>>()
			.join("/");
		Interface::new(format!("{}{}", name, indices_str))
	}
}

impl<'a, T: Configurable> ConfigurationMode<'a, T> {
	/// Enter the configuration mode and steal the session while in configuration mode.
	pub fn enter(session: &mut T) -> ConfigurationMode<T> {
		ConfigurationMode {
			session,
		}
	}

	/// Execute any raw command on the device from configuration mode
	pub fn execute_raw(&mut self, command: &str) -> io::Result<()> {
		self.session.execute_raw(command)
	}
}

impl<T: Configurable> Drop for ConfigurationMode<'_, T> {
	/// Drops the ConfigurationMode object at the end of the scope
	/// To free the locked session, making it usable again.
	/// Also exits the current state in the session.
	fn drop(&mut self) {
		let _ = self.session.exit();
	}
}