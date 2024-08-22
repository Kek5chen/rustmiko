//! Devices describe all currently possible, out of the box supported devices that can be used
//! through our connection handlers. They abstract the usage and commands needed to manage a
//! networking device of the specified type.
//!
//! Devices are usually implemented in the sense that an API is defined, which wraps some type of
//! Connection Handler (SSHConnection / TelnetConnection) and then interacts over this line with
//! the device. This allows the user to be able to just call device specific functions to configure
//! their device.
//!
//! Usage of the base types is not suggested unless you implement your own ConnectionHandler. \
//! **It's recommended to use one of the predefined types like: [`CiscoSSH`] or [`CiscoTelnet`]
//! instead of [`CiscoDevice`]**
//!
//! [`CiscoSSH`]: crate::devices::cisco::CiscoSSH
//! [`CiscoTelnet`]: crate::devices::cisco::CiscoTelnet
//! [`CiscoDevice`]: crate::devices::cisco::CiscoDevice

pub mod generic;
#[cfg(feature = "cisco")]
pub mod cisco;