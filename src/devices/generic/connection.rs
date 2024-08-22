use std::io;
use std::net::ToSocketAddrs;
use std::time::Duration;
use log::debug;
use telnet::{Event, Telnet};

/// A Connection trait describes a basic set of functions that are necessary for the most basic of
/// implementations.
///
/// Connecting, reading and writing specifically need to be implemented.
pub trait Connection {
	type ConnectionHandler;

	/// Connects to the specified address.
	fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self::ConnectionHandler>;
	/// Reads input, sent by the server but ignores it.
	fn read_ignore(&mut self);
	/// Executes a raw string command on the connection.
	fn execute_raw(&mut self, command: &str) -> io::Result<()>;
}

/// A TelnetConnection is a Connection type, that uses Telnet to connect to the device.
pub struct TelnetConnection {
	conn: Telnet,
}

impl Connection for TelnetConnection {
	type ConnectionHandler = TelnetConnection;

	fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self::ConnectionHandler> {
		Ok(TelnetConnection {
			conn: Telnet::connect(addr, 256)?,
		})
	}

	fn read_ignore(&mut self) {
		loop {
			let event = self.conn.read_timeout(Duration::from_secs(1));
			if let Ok(Event::TimedOut) = event {
				break;
			}
			if let Err(e) = event {
				debug!("Error discarded: {}", e);
				break;
			}
			if let Ok(Event::Data(data)) = event {
				debug!("Discarded data {}", String::from_utf8_lossy(&data));
			} else {
				debug!("Discarded event: {:?}", event);
			}
		}
	}

	fn execute_raw(&mut self, command: &str) -> io::Result<()> {
		self.conn.write(command.as_bytes())?;
		self.conn.write(b"\n")?;
		self.read_ignore();
		Ok(())
	}
}

/// This trait is implemented for connections that require some form of authorization
/// against the device.
///
/// It provides the login() function that is needed to authorize after connecting.
pub trait AuthorizedConnection {
	fn login(&mut self, username: &str, password: &str) -> io::Result<()>;
}
