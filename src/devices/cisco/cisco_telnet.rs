use std::io;
use std::net::ToSocketAddrs;
use crate::devices::generic::device_types::config::{Configurable, ConfigurationMode, InterfaceConfigurable};
use crate::devices::generic::connection::{AuthorizedConnection, Connection, TelnetConnection};
use crate::devices::generic::device_types::interfaces::Interface;

pub struct CiscoTelnet {
	conn: TelnetConnection,
}

impl CiscoTelnet {
	pub fn new<A: ToSocketAddrs>(addr: A) -> io::Result<CiscoTelnet> {
		let mut telnet = CiscoTelnet {
			conn: TelnetConnection::connect(addr)?,
		};
		telnet.conn.read_ignore();
		Ok(telnet)
	}
}

impl Configurable for CiscoTelnet {
	type SessionType = CiscoTelnet;

	fn enter_config(&mut self) -> io::Result<ConfigurationMode<CiscoTelnet>> {
		self.conn.execute_raw("configure terminal")?;
		Ok(ConfigurationMode::enter(self))
	}
	fn exit(&mut self) -> io::Result<()> {
		self.conn.execute_raw("exit")
	}

	fn save(&mut self) -> io::Result<()> {
		self.conn.execute_raw("write memory")
	}
}

impl<'a> InterfaceConfigurable for ConfigurationMode<'a, CiscoTelnet> {
	fn interface_up(&mut self, interface: &Interface) -> io::Result<()> {
		self.session.conn.execute_raw(&format!("interface {}", interface.name()))?;
		self.session.conn.execute_raw("no shutdown")?;
		self.session.exit()
	}

	fn interface_down(&mut self, interface: &Interface) -> io::Result<()> {
		self.session.conn.execute_raw(&format!("interface {}", interface.name()))?;
		self.session.conn.execute_raw("shutdown")?;
		self.session.exit()
	}
}

impl AuthorizedConnection for CiscoTelnet {
	fn login(&mut self, username: &str, password: &str) -> io::Result<()> {
		self.conn.execute_raw(username)?;
		self.conn.execute_raw(password)?;
		Ok(())
	}
}