use anyhow::bail;
use rustmiko::devices::cisco::CiscoTelnet;
use rustmiko::devices::generic::connection::{AuthorizedConnection, Connection};
use rustmiko::devices::generic::device_types::config::{Configurable, InterfaceConfigurable};

fn main() -> anyhow::Result<()> {
	let mut cisco = match CiscoTelnet::new("192.168.178.1:23") {
		Ok(cisco) => {
			println!("Connected successfully");
			cisco
		},
		Err(e) => {
			eprintln!("Failed to connect: {}", e);
			return Ok(());
		},
	};

	let _ = cisco.login("admin", "qwertzuiopas");

	{
		let mut config = cisco.enter_config()?;
		let interface = config.get_interface("FastEthernet", &[0, 7]);
		&config.interface_down(&interface);
	}

	if let Err(e) = cisco.save() {
		eprintln!("Failed to save configuration");
	}

	Ok(())
}