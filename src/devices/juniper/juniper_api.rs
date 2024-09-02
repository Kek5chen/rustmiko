use std::error::Error;
use std::io;
use std::net::ToSocketAddrs;
use regex::Regex;
use crate::devices::generic::connection::Connection;
use crate::devices::generic::device_types::config::{Configurable, ConfigurationMode, InterfaceConfigurable};
use crate::devices::generic::device_types::interfaces::Interface;

/// A juniper (EX) device API implementation.
///
/// Usage of the base type is not suggested unless you implement your own ConnectionHandler.
/// It's recommended to use one of the predefined types:
/// [`JuniperSSH`] or
/// [`JuniperTelnet`]
///
/// [`JuniperSSH`]: crate::devices::juniper::JuniperSSH
/// [`JuniperTelnet`]: crate::devices::juniper::JuniperTelnet
pub struct JuniperDevice<C: Connection> {
    connection: C,
    prompt_end: Regex,
}

impl<C: Connection<ConnectionHandler = C>> JuniperDevice<C> {
    pub fn connect<A: ToSocketAddrs>(addr: A, username: &str, password: &str) -> Result<JuniperDevice<C>, Box<dyn Error>> {
        let mut device = JuniperDevice {
            connection: C::connect(addr, Some(username), Some(password))?,
            prompt_end: Regex::new(r"[>#%]")?,
        };

        device.connection.read_ignore(&device.prompt_end);
        Ok(device)
    }

    pub fn enter_cli(&mut self) -> io::Result<()> {
        self.execute_raw("cli")?;
        Ok(())
    }
}

impl<C: Connection> Configurable for JuniperDevice<C> {
    type SessionType = Self;

    fn enter_config(&mut self) -> io::Result<ConfigurationMode<Self>> {
        self.execute_raw("configure")?;
        Ok(ConfigurationMode::enter(self))
    }

    fn execute_raw(&mut self, command: &str) -> io::Result<()> {
        self.connection.execute_raw(command, &self.prompt_end)
    }

    fn exit(&mut self) -> io::Result<()> {
        self.execute_raw("exit")
    }
}

impl<'a, C: Connection> InterfaceConfigurable for ConfigurationMode<'a, JuniperDevice<C>> {
    fn interface_up(&mut self, interface: &Interface) -> io::Result<()> {
        self.session.execute_raw(&format!("set interfaces {} enable", interface.name()))
    }

    fn interface_down(&mut self, interface: &Interface) -> io::Result<()> {
        self.session.execute_raw(&format!("set interfaces {} disable", interface.name()))
    }
}

impl<T: Connection> ConfigurationMode<'_, JuniperDevice<T>> {
    pub fn commit(&mut self) -> io::Result<()> {
        self.session.execute_raw("commit")
    }
}