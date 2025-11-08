//! Allow serializing and deserializing godot basic types into network packets

/// Module for network packets
pub mod netpacket;

// re-export elements
pub use netpacket::{ConstSize, Deserialize, Half, Serialize, Unit};

// re-export derive macros
pub mod macros {
    pub use godot_netpacket_macros::{ConstSize, Deserialize, Serialize};
}

#[allow(unused_imports)]
use godot::prelude::*;

/// Root of the library
#[cfg(feature = "standalone")]
struct NetPacketExtension;

#[cfg(feature = "standalone")]
#[gdextension]
unsafe impl ExtensionLibrary for NetPacketExtension {}
