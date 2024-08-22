use rustmiko::devices::cisco::CiscoTelnet;
use rustmiko::devices::generic::device_types::config::{Configurable, InterfaceConfigurable};

fn main() -> anyhow::Result<()> {
	env_logger::init();

	let mut cisco = match CiscoTelnet::connect("192.168.178.1:23", "admin", "admin") {
		Ok(cisco) => {
			println!("Connected successfully");
			cisco
		},
		Err(e) => {
			eprintln!("Failed to connect: {}", e);
			return Ok(());
		},
	};

	{
		let mut config = cisco.enter_config()?;
		for index in 1..=8 {
			let interface = config.get_interface("gi", &[0, index]);
			match config.interface_up(&interface) {
			    Ok(_) => println!("Interface {} is now up", interface.name()),
			    Err(_) => println!("Failed to set Interface {} up", interface.name())
			}
		}
	}

	if let Err(e) = cisco.save() {
		eprintln!("Failed to save configuration: {e}");
	}

	Ok(())
}