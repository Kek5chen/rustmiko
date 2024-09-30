use regex::Regex;
use rustmiko::devices::generic::connection::{Connection, ConnectionOptions, TelnetConnection};

fn main() {
    let mut device = TelnetConnection::connect("192.168.1.101:23", &ConnectionOptions {
        username: Some("admin"),
        password: Some("password"),
        timeout: None
    }).unwrap();
    let prompt_end = Regex::new(r"[#>]").unwrap();

    device.execute_raw("show running configuration", &prompt_end).unwrap();
}