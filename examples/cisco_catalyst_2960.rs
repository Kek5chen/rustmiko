use rustmiko::devices::cisco::CiscoTelnet;
use rustmiko::devices::generic::connection::Connection;
use rustmiko::devices::generic::device_types::config::Configurable;

fn main() {
	let mut cisco = CiscoTelnet::new();
	match cisco.connect("192.168.1.101") {
		Ok(_) => println!("Connected successfully"),
		Err(e) => eprintln!("Failed to connect: {}", e),
	}

	{
		let mut config = cisco.enter_config();
		let interface = config.get_interface("FastEthernet", &[0, 1]);
		&config.interface_up(&interface);
	}

	if let Err(e) = cisco.save() {
		eprintln!("Failed to save configuration");
	}
}