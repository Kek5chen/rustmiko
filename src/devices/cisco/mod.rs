//! All cisco device types.
mod cisco_api;

pub use cisco_api::CiscoDevice;


use crate::devices::generic::connection::{SSHConnection, TelnetConnection};

pub type CiscoSSH = CiscoDevice<SSHConnection>;
pub type CiscoTelnet = CiscoDevice<TelnetConnection>;
