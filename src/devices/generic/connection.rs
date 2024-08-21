use std::io;
use std::net::ToSocketAddrs;
use std::time::Duration;
use telnet::{Event, Telnet};

pub trait Connection {
	type ConnectionHandler;

	fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self::ConnectionHandler>;
	fn read_ignore(&mut self);
	fn execute_raw(&mut self, command: &str) -> io::Result<()>;
}

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
				println!("Error discarded: {}", e);
				break;
			}
			if let Ok(Event::Data(data)) = event {
				println!("Discarded data {}", String::from_utf8_lossy(&data));
			} else {
				println!("Discarded event: {:?}", event);
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

pub trait AuthorizedConnection {
	fn login(&mut self, username: &str, password: &str) -> io::Result<()>;
}
