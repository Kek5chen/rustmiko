use std::io;
use std::io::{Read, Write};
use std::net::ToSocketAddrs;
use telnet::{Telnet};

pub struct TelnetConnection {
	conn: Telnet,
}

impl Write for TelnetConnection {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		self.conn.write(buf)
	}

	fn flush(&mut self) -> io::Result<()> {
		self.conn.flush()
	}
}

impl Read for TelnetConnection {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		let ev = self.conn.read()?;
		todo!()
	}
}

impl Connection for TelnetConnection {
	type ConnectionHandler = TelnetConnection;

	fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self::ConnectionHandler> {
		Ok(TelnetConnection {
			conn: Telnet::connect(addr, 256)?,
		})
	}
}



pub trait Connection: Write + Read {
	type ConnectionHandler;

	fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<Self::ConnectionHandler>;
}
