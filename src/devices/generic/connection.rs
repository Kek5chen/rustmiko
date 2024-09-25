use std::error::Error;
use std::fmt::format;
use std::io;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use anyhow::format_err;
use log::debug;
use ssh2::{Channel, Session};
use telnet::{Event, Telnet};
use regex::Regex;

/// A Connection trait describes a basic set of functions that are necessary for the most basic of
/// implementations.
///
/// Connecting, reading and writing specifically need to be implemented.
pub trait Connection {
	type ConnectionHandler;

	/// Connects to the specified address using a Connection Handler.
	fn connect<A: ToSocketAddrs>(addr: A, username: Option<&str>, password: Option<&str>) -> Result<Self::ConnectionHandler, Box<dyn Error>>;
	/// Reads input, sent by the server but ignores it.
	fn read_ignore(&mut self, prompt_end: &Regex);
	/// Executes a raw string command on the connection.
	fn execute_raw(&mut self, command: &str, prompt_end: &Regex) -> io::Result<()>;
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
			conn.execute_raw(username, &Regex::new("[Pp]assword")?)?;

			if let Some(password) = password {
				conn.execute_raw(password, &Regex::new("^([Uu]sername)")?)?;
			}
		}

		Ok(conn)
	}

	/// Read input from the server, but just ignore it.
	fn read_ignore(&mut self, prompt_end: &Regex) {
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
				let datastr = String::from_utf8_lossy(&data);
				debug!("Discarded data {}", datastr);
				if prompt_end.is_match(datastr.trim_end()) {
					debug!("Found prompt. Ready for next command");
					break;
				}
			} else {
				debug!("Discarded event: {:?}", event);
			}
		}
	}

	/// Execute a raw command. A new line is automatically appended for the user.
	fn execute_raw(&mut self, command: &str, prompt_end: &Regex) -> io::Result<()> {
		self.conn.write(command.as_bytes())?;
		self.conn.write(b"\n")?;
		self.read_ignore(prompt_end);
		Ok(())
	}
}

pub struct SSHConnection {
	#[allow(dead_code)]
	sess: Session,
	channel: Channel,
}

impl SSHConnection {
	fn establish_connection<A: ToSocketAddrs>(addr: A, timeout: Option<Duration>) -> Result<Session, Box<dyn Error>> {
		let tcp = match timeout {
			None => TcpStream::connect(addr)?,
			// TODO: Handle this better
			Some(timeout) => TcpStream::connect_timeout(&addr.to_socket_addrs()?.next().ok_or_else(|| format_err!("uh"))?, timeout)?
		};
		let mut sess = Session::new()?;
		sess.set_timeout(60000);

		sess.set_tcp_stream(tcp);
		sess.handshake()?;

		Ok(sess)
	}

	fn make_channel_session(session: Session) -> Result<SSHConnection, Box<dyn Error>>{
		let mut channel = session.channel_session()?;
		channel.request_pty("rustmiko", None, None)?;
		channel.shell()?;

		Ok(SSHConnection {
			sess: session,
			channel,
		})
	}

	pub fn connect_agentauth<A: ToSocketAddrs>(addr: A, username: &str, timeout: Option<Duration>) -> Result<SSHConnection, Box<dyn Error>> {
		let sess = Self::establish_connection(addr, timeout)?;

		sess.userauth_agent(username)?;

		if !sess.authenticated() {
			return Err("Couldn't authenticate properly against SSH Server using SSH Agent.".into());
		}

		Self::make_channel_session(sess)
	}
}

impl Connection for SSHConnection {
	type ConnectionHandler = SSHConnection;

	/// Connect to device at ip:port addr using SSH, with an optional username and password
	/// which are sent to the device right after the connection is made.
	fn connect<A: ToSocketAddrs>(addr: A, username: Option<&str>, password: Option<&str>) -> Result<SSHConnection, Box<dyn Error>> {
		if username.is_none() || password.is_none() {
			// Could also panic here because this should never happen as it's implemented on the
			// device side
			return Err("Can't connect to SSH without username and password".into());
		}

		let sess = Self::establish_connection(addr, None)?;

		let username = username.unwrap();
		let password = password.unwrap();
		sess.userauth_password(username, password)?;

		if !sess.authenticated() {
			return Err("Couldn't authenticate properly against SSH Server using password auth".into());
		}

		Self::make_channel_session(sess)
	}

	fn read_ignore(&mut self, prompt_end: &Regex) {
		debug!("Reading...");
		loop {
			let mut buf = [0u8; 1024];

			let size = match self.channel.read(&mut buf) {
				Ok(s) => s,
				Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
					debug!("Timed out... Assuming no data");
					break;
				}
				Err(e) => {
					debug!("Ignored error: {}", e);
					break;
				},
			};

			let str = String::from_utf8_lossy(&buf[..size]);
			debug!("Ignored \"{}\"", str);

			if prompt_end.is_match(&str) {
				debug!("Found prompt. Ready for next command");
				break;
			}
		}
	}

	fn execute_raw(&mut self, command: &str, prompt_end: &Regex) -> io::Result<()> {
		debug!("Wrote: {}", command);

		self.channel.write_all(command.as_bytes())?;
		self.channel.write_all(b"\n")?;
		self.read_ignore(prompt_end);

		Ok(())
	}
}
