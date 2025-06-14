//!
//! Define serialization and deserialization for other types.
//!

use super::{ConstSize, Deserialize, Half, Serialize, Unit};
use godot::builtin::*;

// implement for boolean

impl ConstSize for bool {
    const SIZE: usize = 1;
}

impl Serialize for bool {
    #[inline]
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        buffer.encode_u8(offset, if *self { 1 } else { 0 })
    }
}

impl Deserialize for bool {
    #[inline]
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        buffer.decode_u8(offset).map(|byte| byte != 0)
    }
}

// implement for plane

impl ConstSize for Plane {
    const SIZE: usize = Vector3::SIZE + size_of::<f32>();
}
impl Serialize for Plane {
    #[inline]
    #[allow(clippy::unnecessary_cast)]
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        self.normal.serialize(buffer, offset)?;
        buffer.encode_float(offset + Vector3::SIZE, self.d as f32)
    }
}
impl Deserialize for Plane {
    #[inline]
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        let normal = Vector3::deserialize(buffer, offset)?;
        let d = buffer.decode_float(offset + Vector3::SIZE)? as real;
        Ok(Plane { normal, d })
    }
}

impl ConstSize for Unit<Plane> {
    const SIZE: usize = <Unit<Vector3>>::SIZE + size_of::<f32>();
}
impl Serialize for Unit<Plane> {
    #[inline]
    #[allow(clippy::unnecessary_cast)]
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        Unit(self.0.normal).serialize(buffer, offset)?;
        buffer.encode_float(offset + <Unit<Vector3>>::SIZE, self.0.d as f32)
    }
}
impl Deserialize for Unit<Plane> {
    #[inline]
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        let normal = <Unit<Vector3>>::deserialize(buffer, offset)?.0;
        let d = buffer.decode_float(offset + <Unit<Vector3>>::SIZE)? as real;
        Ok(Unit(Plane { normal, d }))
    }
}

impl ConstSize for Half<Plane> {
    const SIZE: usize = <Half<Vector3>>::SIZE + size_of::<f32>();
}
impl Serialize for Half<Plane> {
    #[inline]
    #[allow(clippy::unnecessary_cast)]
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        Half(self.0.normal).serialize(buffer, offset)?;
        buffer.encode_float(offset + <Half<Vector3>>::SIZE, self.0.d as f32)
    }
}
impl Deserialize for Half<Plane> {
    #[inline]
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        let normal = <Half<Vector3>>::deserialize(buffer, offset)?.0;
        let d = buffer.decode_float(offset + <Half<Vector3>>::SIZE)? as real;
        Ok(Half(Plane { normal, d }))
    }
}

// implement for color

impl ConstSize for Color {
    const SIZE: usize = 4;
}
impl Serialize for Color {
    #[inline]
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        const FACTOR: f32 = 255.0;

        buffer.encode_u8(offset, (self.r * FACTOR) as u8)?;
        buffer.encode_u8(offset + 1, (self.g * FACTOR) as u8)?;
        buffer.encode_u8(offset + 2, (self.b * FACTOR) as u8)?;
        buffer.encode_u8(offset + 3, (self.a * FACTOR) as u8)?;
        Ok(())
    }
}
impl Deserialize for Color {
    #[inline]
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        const FACTOR: f32 = 1.0 / 255.0;

        let r = buffer.decode_u8(offset)? as f32 * FACTOR;
        let g = buffer.decode_u8(offset + 1)? as f32 * FACTOR;
        let b = buffer.decode_u8(offset + 2)? as f32 * FACTOR;
        let a = buffer.decode_u8(offset + 3)? as f32 * FACTOR;

        Ok(Color { r, g, b, a })
    }
}
