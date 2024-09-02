use std::thread::sleep;
use std::time::Duration;
use rustmiko::devices::generic::device_types::config::{Configurable, InterfaceConfigurable};
use rustmiko::devices::juniper::JuniperSSH;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut juniper = match JuniperSSH::connect("192.168.178.1:22", "admin", "admin") {
        Ok(device) => {
            println!("Connected successfully");
            device
        },
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
            return Ok(());
        },
    };

    juniper.enter_cli()?;
    {
        let mut config = juniper.enter_config()?;
        let interface = config.get_interface("ge-", &[0, 0, 0]);

        config.interface_down(&interface)?;

        config.commit()?;

        sleep(Duration::from_secs(20));

        config.interface_up(&interface)?;

        config.commit()?;
    }

    Ok(())
}