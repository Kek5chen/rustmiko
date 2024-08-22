use std::error::Error;
use std::io;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use log::debug;
use ssh2::{Channel, Session};
use telnet::{Event, Telnet};

/// A Connection trait describes a basic set of functions that are necessary for the most basic of
/// implementations.
///
/// Connecting, reading and writing specifically need to be implemented.
pub trait Connection {
	type ConnectionHandler;

	/// Connects to the specified address using a Connection Handler.
	fn connect<A: ToSocketAddrs>(addr: A, username: Option<&str>, password: Option<&str>) -> Result<Self::ConnectionHandler, Box<dyn Error>>;
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

	/// Connect to device at ip:port addr, using telnet with an optional username and password
	/// which are sent to the device right after the connection is made.
	fn connect<A: ToSocketAddrs>(addr: A, username: Option<&str>, password: Option<&str>) -> Result<TelnetConnection, Box<dyn Error>> {
		let mut conn = TelnetConnection {
			conn: Telnet::connect(addr, 1024)?,
		};

		// Authenticate
		if let Some(username) = username {
			conn.execute_raw(username)?;

			if let Some(password) = password {
				conn.execute_raw(password)?;
			}
		}

		Ok(conn)
	}

	/// Read input from the server, but just ignore it.
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

	/// Execute a raw command. A new line is automatically appended for the user.
	fn execute_raw(&mut self, command: &str) -> io::Result<()> {
		self.conn.write(command.as_bytes())?;
		self.conn.write(b"\n")?;
		self.read_ignore();
		Ok(())
	}
}

pub struct SSHConnection {
	sess: Session,
	channel: Channel,
}

impl Connection for SSHConnection {
	type ConnectionHandler = SSHConnection;

	/// Connect to device at ip:port addr using SSH, with an optional username and password
	/// which are sent to the device right after the connection is made.
	fn connect<A: ToSocketAddrs>(addr: A, username: Option<&str>, password: Option<&str>) -> Result<SSHConnection, Box<dyn Error>> {
		let tcp = TcpStream::connect(addr)?;
		let mut sess = Session::new()?;

		sess.set_tcp_stream(tcp);
		sess.handshake()?;

		let mut channel = sess.channel_session()?;

		if let Some(username) = username {
			channel.write_all(username.as_bytes())?;
			if let Some(password) = password {
				channel.write_all(password.as_bytes())?;
			}
		}

		Ok(SSHConnection {
			sess,
			channel,
		})
	}

	fn read_ignore(&mut self) {
		let mut buf: Vec<u8> = Vec::with_capacity(1024);

		match self.channel.read(&mut buf) {
			Ok(s) => debug!("Ignored {} bytes: {}", s, String::from_utf8_lossy(&buf)),
			Err(e) => debug!("Ignored error: {}", e),
		};
	}

	fn execute_raw(&mut self, command: &str) -> io::Result<()> {
		self.channel.write_all(command.as_bytes())?;
		self.channel.write_all(b"\n")?;
		self.read_ignore();
		Ok(())
	}
}
