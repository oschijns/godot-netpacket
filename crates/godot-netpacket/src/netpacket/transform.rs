//!
//! Define encapsulated types
//!

use super::{ConstSize, Deserialize, Half, Serialize, Unit};
use godot::{builtin::*, prelude::real_consts::PI};

// implement for basis

impl ConstSize for Basis {
    const SIZE: usize = Vector3::SIZE * 3;
}

impl Serialize for Basis {
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        // iterate over each of the rows
        for (index, row) in self.rows.iter().enumerate() {
            row.serialize(buffer, offset + (Vector3::SIZE * index))?;
        }
        Ok(())
    }
}

impl Deserialize for Basis {
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        // read the rows from the buffer
        let mut rows = [Vector3::ZERO; 3];
        for (index, row) in rows.iter_mut().enumerate() {
            *row = Vector3::deserialize(buffer, offset + (Vector3::SIZE * index))?;
        }

        Ok(Basis { rows })
    }
}

impl ConstSize for Unit<Basis> {
    // use quaternion
    const SIZE: usize = <Unit<Quaternion>>::SIZE;
}

impl Serialize for Unit<Basis> {
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        // serialize as quaternion
        let quat = Unit(self.0.get_quaternion());
        quat.serialize(buffer, offset)
    }
}

impl Deserialize for Unit<Basis> {
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        // deserialize as a quaternion
        let quat = <Unit<Quaternion>>::deserialize(buffer, offset)?.0;
        Ok(Unit(Basis::from_quaternion(quat)))
    }
}

impl ConstSize for Half<Basis> {
    const SIZE: usize = <Half<Vector3>>::SIZE * 3;
}

impl Serialize for Half<Basis> {
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        const SIZE: usize = <Half<Vector3>>::SIZE;

        // iterate over each of the rows
        for (index, row) in self.0.rows.iter().enumerate() {
            Half(*row).serialize(buffer, offset + (SIZE * index))?;
        }
        Ok(())
    }
}

impl Deserialize for Half<Basis> {
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        const SIZE: usize = <Half<Vector3>>::SIZE;

        // read the rows from the buffer
        let mut rows = [Vector3::ZERO; 3];
        for (index, row) in rows.iter_mut().enumerate() {
            *row = <Half<Vector3>>::deserialize(buffer, offset + (SIZE * index))?.0;
        }

        Ok(Half(Basis { rows }))
    }
}

// implement for transform 2D

impl ConstSize for Transform2D {
    const SIZE: usize = Vector2::SIZE * 3;
}

impl Serialize for Transform2D {
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        const SIZE: usize = Vector2::SIZE;
        self.a.serialize(buffer, offset)?;
        self.b.serialize(buffer, offset + SIZE)?;
        self.origin.serialize(buffer, offset + (SIZE * 2))
    }
}

impl Deserialize for Transform2D {
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        const SIZE: usize = Vector2::SIZE;
        let a = Vector2::deserialize(buffer, offset)?;
        let b = Vector2::deserialize(buffer, offset + SIZE)?;
        let origin = Vector2::deserialize(buffer, offset + (SIZE * 2))?;
        Ok(Transform2D { a, b, origin })
    }
}

impl ConstSize for Unit<Transform2D> {
    // use angle and position
    const SIZE: usize = 1 + Vector2::SIZE;
}

impl Serialize for Unit<Transform2D> {
    // serialize an angle and a position
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        const FACTOR: real = 127.0 / PI;

        // encode the angle as a single byte
        let angle = (self.0.rotation() * FACTOR) as i8;
        buffer.encode_s8(offset, angle)?;
        self.0.origin.serialize(buffer, offset + 1)
    }
}

impl Deserialize for Unit<Transform2D> {
    // deserialize an angle and a position
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        const FACTOR: real = PI / 127.0;

        // decode the angle as a single byte
        let angle = buffer.decode_s8(offset)? as real * FACTOR;
        let origin = Vector2::deserialize(buffer, offset + 1)?;
        Ok(Unit(Transform2D::from_angle_origin(angle, origin)))
    }
}

impl ConstSize for Half<Transform2D> {
    const SIZE: usize = <Half<Vector2>>::SIZE * 2 + Vector2::SIZE;
}

impl Serialize for Half<Transform2D> {
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        const SIZE: usize = <Half<Vector2>>::SIZE;
        Half(self.0.a).serialize(buffer, offset)?;
        Half(self.0.b).serialize(buffer, offset + SIZE)?;
        self.0.origin.serialize(buffer, offset + (SIZE * 2))
    }
}

impl Deserialize for Half<Transform2D> {
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        const SIZE: usize = <Half<Vector2>>::SIZE;
        let a = <Half<Vector2>>::deserialize(buffer, offset)?.0;
        let b = <Half<Vector2>>::deserialize(buffer, offset + SIZE)?.0;
        let origin = Vector2::deserialize(buffer, offset + (SIZE * 2))?;
        Ok(Half(Transform2D { a, b, origin }))
    }
}

// implement for transform 3D

impl ConstSize for Transform3D {
    const SIZE: usize = Basis::SIZE + Vector3::SIZE;
}

impl Serialize for Transform3D {
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        self.basis.serialize(buffer, offset)?;
        self.origin.serialize(buffer, offset + Basis::SIZE)
    }
}

impl Deserialize for Transform3D {
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        let basis = Basis::deserialize(buffer, offset)?;
        let origin = Vector3::deserialize(buffer, offset + Basis::SIZE)?;
        Ok(Transform3D { basis, origin })
    }
}

impl ConstSize for Unit<Transform3D> {
    // use quaternion and position
    const SIZE: usize = <Unit<Quaternion>>::SIZE + Vector3::SIZE;
}

impl Serialize for Unit<Transform3D> {
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        const QUAT_SIZE: usize = <Unit<Quaternion>>::SIZE;

        // encode the transform as a quaternion and a position
        Unit(self.0.basis.get_quaternion()).serialize(buffer, offset)?;
        self.0.origin.serialize(buffer, offset + QUAT_SIZE)
    }
}

impl Deserialize for Unit<Transform3D> {
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        const QUAT_SIZE: usize = <Unit<Quaternion>>::SIZE;

        // decode the transform as a quaternion and a position
        let quat = <Unit<Quaternion>>::deserialize(buffer, offset)?.0;
        let origin = Vector3::deserialize(buffer, offset + QUAT_SIZE)?;
        let basis = Basis::from_quaternion(quat);
        Ok(Unit(Transform3D { basis, origin }))
    }
}

impl ConstSize for Half<Transform3D> {
    const SIZE: usize = <Half<Basis>>::SIZE + Vector3::SIZE;
}

impl Serialize for Half<Transform3D> {
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        const SIZE: usize = <Half<Basis>>::SIZE;
        Half(self.0.basis).serialize(buffer, offset)?;
        self.0.origin.serialize(buffer, offset + SIZE)
    }
}

impl Deserialize for Half<Transform3D> {
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        const SIZE: usize = <Half<Basis>>::SIZE;
        let basis = <Half<Basis>>::deserialize(buffer, offset)?.0;
        let origin = Vector3::deserialize(buffer, offset + SIZE)?;
        Ok(Half(Transform3D { basis, origin }))
    }
}
