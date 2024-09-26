//! Rustmiko is a Rust crate designed to facilitate network device automation,
//! inspired by the popular Python library Netmiko.
//!
//! This crate aims to provide a type-safe and user-friendly interface for interacting
//! with network devices, emphasizing ease of use and abstraction over low-level detail management.
//!
//! Example of usage (Cisco Catalyst 2960 Switch):
//! ```ignore
//! fn main() -> anyhow::Result<()> {
//! 	let mut cisco = match CiscoTelnet::connect("192.168.1.101:23", "admin", "admin") {
//! 		Ok(cisco) => {
//! 			println!("Connected successfully");
//! 			cisco
//! 		},
//! 		Err(e) => {
//! 			eprintln!("Failed to connect: {}", e);
//! 			return Ok(());
//! 		},
//! 	};
//!
//! 	{
//! 		let mut config = cisco.enter_config()?;
//! 		for index in 1..=8 {
//! 			let interface = config.get_interface("gi", &[0, index]);
//! 			match config.interface_up(&interface) {
//!                 Ok(_) => println!("Interface {} is now up", interface.name()),
//!                 Err(_) => println!("Failed to set Interface {} up", interface.name())
//!             }
//! 		}
//! 	}
//!
//! 	if let Err(e) = cisco.save() {
//! 		eprintln!("Failed to save configuration: {e}");
//! 	}
//!
//! 	Ok(())
//! }
//! ```

pub mod devices;