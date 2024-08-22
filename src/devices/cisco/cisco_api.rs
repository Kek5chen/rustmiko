use std::error::Error;
use std::io;
use std::net::ToSocketAddrs;
use crate::devices::generic::connection::Connection;
use crate::devices::generic::device_types::config::{Configurable, ConfigurationMode, InterfaceConfigurable};
use crate::devices::generic::device_types::interfaces::Interface;

/// A cisco (catalyst) device API implementation.
///
/// Usage of the base type is not suggested unless you implement your own ConnectionHandler.
/// It's recommended to use one of the predefined types:
/// [`CiscoSSH`] or
/// [`CiscoTelnet`]
///
/// [`CiscoSSH`]: crate::devices::cisco::CiscoSSH
/// [`CiscoTelnet`]: crate::devices::cisco::CiscoTelnet
pub struct CiscoDevice<C: Connection> {
    connection: C,
}

impl<C: Connection<ConnectionHandler = C>> CiscoDevice<C> {
    pub fn connect<A: ToSocketAddrs>(addr: A, username: &str, password: &str) -> Result<CiscoDevice<C>, Box<dyn Error>> {
        Ok(CiscoDevice {
            connection: C::connect(addr, Some(username), Some(password))?,
        })
    }

    pub fn enable(&mut self, password: &str) -> io::Result<()> {
        self.connection.execute_raw("enable")?;
        if !password.is_empty() {
            self.connection.execute_raw(password)?;
        }
        Ok(())
    }
}

impl<C: Connection> Configurable for CiscoDevice<C> {
    type SessionType = Self;

    fn enter_config(&mut self) -> io::Result<ConfigurationMode<Self>> {
        self.connection.execute_raw("configure terminal")?;
        Ok(ConfigurationMode::enter(self))
    }

    fn execute_raw(&mut self, command: &str) -> io::Result<()> {
        self.connection.execute_raw(command)
    }

    fn exit(&mut self) -> io::Result<()> {
        self.connection.execute_raw("exit")
    }

    fn save(&mut self) -> io::Result<()> {
        self.connection.execute_raw("write memory")
    }
}

impl<'a, C: Connection> InterfaceConfigurable for ConfigurationMode<'a, CiscoDevice<C>> {
    fn interface_up(&mut self, interface: &Interface) -> io::Result<()> {
        self.session.connection.execute_raw(&format!("interface {}", interface.name()))?;
        self.session.connection.execute_raw("no shutdown")?;
        self.session.exit()
    }

    fn interface_down(&mut self, interface: &Interface) -> io::Result<()> {
        self.session.connection.execute_raw(&format!("interface {}", interface.name()))?;
        self.session.connection.execute_raw("shutdown")?;
        self.session.exit()
    }
}
