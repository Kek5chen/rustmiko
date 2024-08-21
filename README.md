# Rustmiko

(Currently) less compatible, but better typed Netmiko alternative in Rust.

# Compatibility

| Brand   | Device        | Support |
|---------|---------------|---------|
| Cisco   | Catalyst 2960 | Partial |

# Example

This example will set all ports up on a Cisco Catalyst 2960 Series Switch.
```rust
fn main() -> anyhow::Result<()> {
    let mut cisco = match CiscoTelnet::new("192.168.178.1:23") {
        Ok(cisco) => {
            println!("Connected successfully");
            cisco
        },
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            return Ok(());
        },
    };

    let _ = cisco.login("admin", "admin");

    {
        let mut config = cisco.enter_config()?;
        for index in 1..=8 {
            let interface = config.get_interface("FastEthernet", &[0, index]);
            let _ = config.interface_up(&interface);
        }
    }

    if let Err(e) = cisco.save() {
        eprintln!("Failed to save configuration: {e}");
    }

    Ok(())
}
```

# Contributions

Due to me being a normal human being I do not have the ability or resources
to own every switch in the entire world to test. I'd be really grateful if more configurations
for different switches are added.

Thanks!