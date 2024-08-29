mod juniper_api;

use crate::devices::generic::connection::{SSHConnection, TelnetConnection};
use crate::devices::juniper::juniper_api::JuniperDevice;

pub type JuniperSSH = JuniperDevice<SSHConnection>;
pub type JuniperTelnet = JuniperDevice<TelnetConnection>;
