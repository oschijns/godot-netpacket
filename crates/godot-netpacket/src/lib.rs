//!

/// Module for network packets
pub mod netpacket;

// re-export elements
pub use netpacket::{ConstSize, Deserialize, Half, Serialize, Unit};

// re-export derive macros
pub mod macros {
    pub use godot_netpacket_macros::{ConstSize, Deserialize, Serialize};
}
