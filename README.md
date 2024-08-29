# Rustmiko

(Currently) less compatible, but better typed Netmiko alternative in Rust.

## Documentation
Documentation is available on [docs.rs](https://docs.rs/rustmiko/latest).

# Compatibility

| Type      | Meaning                                |
|-----------|----------------------------------------|
| Automated | Tested, Supported and unit tests exist |
| Full      | Tested and fully supported             |
| Partial   | Tested, but limited usage possible     |
| Buggy     | No guarantee on anything               |

| Brand   | Device               | Support |
|---------|----------------------|---------|
| Cisco   | Catalyst 2960 Series | Full    |
| Juniper | EX Series            | Partial |

# Example

This example will set all ports up on a Cisco Catalyst 2960/CX Series Switch.

```rust
fn main() -> anyhow::Result<()> {
	let mut cisco = match CiscoTelnet::connect("192.168.1.101:23", "admin", "admin") {
		Ok(cisco) => {
			println!("Connected successfully");
			cisco
		},
		Err(e) => {
			eprintln!("Failed to connect: {}", e);
			return Ok(());
		},
	};

	{
		let mut config = cisco.enter_config()?;
		for index in 1..=8 {
			let interface = config.get_interface("gi", &[0, index]);
			match config.interface_up(&interface) {
                Ok(_) => println!("Interface {} is now up", interface.name()),
                Err(_) => println!("Failed to set Interface {} up", interface.name())
            }
		}
	}

	if let Err(e) = cisco.save() {
		eprintln!("Failed to save configuration: {e}");
	}

	Ok(())
}
```

# Contributions

Due to the disadvantage of me being a human being, I do not have the ability or resources
to own every switch in the entire world to test. I'd be really grateful if more configurations
for different switches are added.

Thanks!