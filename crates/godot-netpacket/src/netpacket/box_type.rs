//!
//! Define serialization and deserialization for box types.
//!

use super::{ConstSize, Deserialize, Half, Serialize};
use godot::builtin::*;

/// Implement traits for box types
macro_rules! impl_box {
    // simple implementation
    ( $box:ty [ $vec:ty ] ) => {
        impl ConstSize for $box {
            const SIZE: usize = <$vec>::SIZE * 2;
        }
        impl Serialize for $box {
            #[inline]
            fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
                self.position.serialize(buffer, offset)?;
                self.size.serialize(buffer, offset + <$vec>::SIZE)
            }
        }
        impl Deserialize for $box {
            #[inline]
            fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
            where
                Self: Sized,
            {
                let position = <$vec>::deserialize(buffer, offset)?;
                let size = <$vec>::deserialize(buffer, offset + <$vec>::SIZE)?;
                Ok(Self { position, size })
            }
        }
    };

    // optimized implementation
    ( $box:ty [ $vec:ty ] as Half ) => {
        impl ConstSize for Half<$box> {
            const SIZE: usize = <$vec>::SIZE + <Half<$vec>>::SIZE;
        }
        impl Serialize for Half<$box> {
            #[inline]
            fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
                self.0.position.serialize(buffer, offset)?;
                Half(self.0.size).serialize(buffer, offset + <$vec>::SIZE)
            }
        }
        impl Deserialize for Half<$box> {
            #[inline]
            fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
            where
                Self: Sized,
            {
                let position = <$vec>::deserialize(buffer, offset)?;
                let size = <Half<$vec>>::deserialize(buffer, offset + <$vec>::SIZE)?.0;
                Ok(Half(<$box>::new(position, size)))
            }
        }
    };
}

impl_box!(Rect2i[Vector2i]);
impl_box!(Rect2[Vector2]);
impl_box!(Aabb[Vector3]);

impl_box!(Rect2[Vector2] as Half);
impl_box!(Aabb[Vector3] as Half);
