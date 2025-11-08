//!
//! Define serialization and deserialization for number types.
//!

use super::{
    ConstSize, Deserialize, HALF_SIZE, Half, Serialize, UNIT_SIZE, Unit, from_unit, to_unit,
};
use godot::builtin::*;

/// Macro to quickly implement traits for network packets.
macro_rules! impl_packet {
    // implement for numeric types
    ( $num:ty => $stored:ty ; $encode:ident ; $decode:ident ) => {
        impl ConstSize for $num {
            const SIZE: usize = size_of::<$stored>();
        }
        impl Serialize for $num {
            #[inline]
            fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
                buffer.$encode(offset, *self as $stored)
            }
        }
        impl Deserialize for $num {
            #[inline]
            fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
            where
                Self: Sized,
            {
                buffer.$decode(offset).map(|num| num as Self)
            }
        }
    };
}

// implement for numeric types

impl_packet![ u8  => u8  ; encode_u8  ; decode_u8  ];
impl_packet![ u16 => u16 ; encode_u16 ; decode_u16 ];
impl_packet![ u32 => u32 ; encode_u32 ; decode_u32 ];
impl_packet![ u64 => u64 ; encode_u64 ; decode_u64 ];

impl_packet![ i8  => i8  ; encode_s8  ; decode_s8  ];
impl_packet![ i16 => i16 ; encode_s16 ; decode_s16 ];
impl_packet![ i32 => i32 ; encode_s32 ; decode_s32 ];
impl_packet![ i64 => i64 ; encode_s64 ; decode_s64 ];

impl_packet![ f32 => f32 ; encode_float  ; decode_float  ];
impl_packet![ f64 => f64 ; encode_double ; decode_double ];

// implement unit for floating point number

impl ConstSize for Unit<f32> {
    const SIZE: usize = UNIT_SIZE;
}
impl Serialize for Unit<f32> {
    #[inline]
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        buffer.encode_s8(offset, to_unit(self.0))
    }
}
impl Deserialize for Unit<f32> {
    #[inline]
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        buffer.decode_s8(offset).map(|num| Self(from_unit(num)))
    }
}

// implement half for floating point number

impl ConstSize for Half<f32> {
    const SIZE: usize = HALF_SIZE;
}
impl Serialize for Half<f32> {
    #[inline]
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        buffer.encode_half(offset, self.0)
    }
}
impl Deserialize for Half<f32> {
    #[inline]
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        buffer.decode_half(offset).map(Self)
    }
}
