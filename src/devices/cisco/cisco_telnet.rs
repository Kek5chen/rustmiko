use std::io;
use std::io::{ErrorKind, Write};
use std::net::ToSocketAddrs;
use anyhow::format_err;
use crate::devices::generic::device_types::config::{Configurable, ConfigurationMode};
use crate::devices::generic::connection::{Connection, TelnetConnection};

pub struct CiscoTelnet {
	conn: TelnetConnection,
}

impl CiscoTelnet {
	pub fn save(&mut self) -> anyhow::Result<()> {
		self.conn
			.as_mut()
			.ok_or_else(|| format_err!("No connection exists"))?
			.write(b"write memory")?;
		Ok(())
	}
}

impl CiscoTelnet {
	pub fn new<S: Into<String>>(connect: S) -> CiscoTelnet {
		
	}
}

impl Configurable<CiscoTelnet> for CiscoTelnet {
	fn enter_config(&mut self) -> ConfigurationMode<CiscoTelnet> {
		ConfigurationMode::enter(self)
	}
}
