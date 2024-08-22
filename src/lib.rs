//! Rustmiko is a Rust crate designed to facilitate network device automation,
//! inspired by the popular Python library Netmiko.
//!
//! This crate aims to provide a type-safe and user-friendly interface for interacting
//! with network devices, emphasizing ease of use and abstraction over low-level detail management.
//!
//! Example of usage (Cisco Catalyst 2960 Switch):
//! ```rust
//! fn main() -> anyhow::Result<()> {
//!     let mut cisco = match CiscoTelnet::new("192.168.178.1:23") {
//!         Ok(cisco) => {
//!             println!("Connected successfully");
//!             cisco
//!         },
//!         Err(e) => {
//!             eprintln!("Failed to connect: {}", e);
//!             return Ok(());
//!         },
//!     };
//!
//!     let _ = cisco.login("admin", "admin");
//!
//!     {
//!         let mut config = cisco.enter_config()?;
//!         for index in 1..=8 {
//!             let interface = config.get_interface("FastEthernet", &[0, index]);
//!             let _ = config.interface_up(&interface);
//!         }
//!     }
//!
//!     if let Err(e) = cisco.save() {
//!         eprintln!("Failed to save configuration: {e}");
//!     }
//!
//!     Ok(())
//! }
//! ```

pub mod devices;