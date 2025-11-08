//!
//! Helpers for handling network packets
//!

/// Packet traits implementation for number types
mod number;

/// Packet traits implementation for vector types
mod vector;

/// Packet traits implementation for box types
mod box_type;

/// Packet traits implementation for wrapped vector types
mod other;

/// Packet traits implementation for transformation types
mod transform;

/// Packet traits implementation for composite types
mod composite;

use godot::builtin::*;

/// Size of the data to serialize or deserialize
pub trait ConstSize {
    const SIZE: usize;
}

/// Serialize to a godot byte buffer
pub trait Serialize {
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()>;
}

/// Deserialize from a godot byte buffer
pub trait Deserialize {
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized;
}

/// Encapsulate vector types to be stored using unit numbers [0.0, 1.0]
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Unit<T>(pub T);

/// Encapsulate vector types to be stored using half floating point numbers
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Half<T>(pub T);

/// Size of an unit
const UNIT_SIZE: usize = 1;

/// Size of an half
const HALF_SIZE: usize = 2;

/// Expecting a real number in the range [-1.0, 1.0] convert it into a byte.
#[inline]
fn to_unit(num: real) -> i8 {
    const FACTOR: real = 127.0;
    (num * FACTOR) as i8
}

/// Convert a byte into a real in the range [-1.0, 1.0]
#[inline]
fn from_unit(byte: i8) -> real {
    const FACTOR: real = 1.0 / 127.0;
    byte as real * FACTOR
}

#[cfg(test)]
mod tests {
    use super::{ConstSize, Deserialize, Serialize};
    //use gd_rehearse::itest::gditest;
    use godot::builtin::{PackedByteArray, Vector2i};
    use godot_netpacket_macros::{ConstSize, Deserialize, Serialize};
    extern crate self as godot_netpacket;

    #[derive(Debug, Serialize, ConstSize, Deserialize, PartialEq, Eq)]
    struct APacket {
        pos: Vector2i,
    }

    #[derive(Debug, Serialize, ConstSize, Deserialize, PartialEq, Eq)]
    enum BPacket {
        A,
        B,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    enum CPacket {
        A { toto: i32 },
        B { foo: i64, bar: bool },
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    fn test_struct() {
        // initial payload
        let a = APacket {
            pos: Vector2i::new(12, 34),
        };

        // Allocate a buffer
        let mut buffer = PackedByteArray::new();
        buffer.resize(APacket::SIZE);

        // serialize and deserialize
        a.serialize(&mut buffer, 0).unwrap();
        let a2 = APacket::deserialize(&buffer, 0).unwrap();

        assert_eq!(a, a2);
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    fn test_simple_enum() {
        // initial payload
        let b = BPacket::B;

        // Allocate a buffer
        let mut buffer = PackedByteArray::new();
        buffer.resize(BPacket::SIZE);

        // serialize and deserialize
        b.serialize(&mut buffer, 0).unwrap();
        let b2 = BPacket::deserialize(&buffer, 0).unwrap();

        assert_eq!(b, b2);
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    fn test_complex_enum() {
        // initial payload
        let c = CPacket::B {
            foo: 1234,
            bar: true,
        };

        // Allocate a buffer
        let mut buffer = PackedByteArray::new();
        buffer.resize(i64::SIZE + bool::SIZE);

        // serialize and deserialize
        c.serialize(&mut buffer, 0).unwrap();
        let c2 = CPacket::deserialize(&buffer, 0).unwrap();

        assert_eq!(c, c2);
    }
}
