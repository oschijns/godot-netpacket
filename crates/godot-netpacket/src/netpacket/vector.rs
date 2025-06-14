//!
//! Define serialization and deserialization for vector types.
//!

use super::{
    ConstSize, Deserialize, HALF_SIZE, Half, Serialize, UNIT_SIZE, Unit, from_unit, to_unit,
};
use godot::builtin::{math::FloatExt, *};

/// Macro to quickly implement traits for network packets.
macro_rules! impl_packet {
    // implement for 2D vector types
    ( $vec:ty [ $num:ty => $stored:ty ] [ 2 ] ; $encode:ident ; $decode:ident ) => {
        impl ConstSize for $vec {
            const SIZE: usize = size_of::<$stored>() * 2;
        }
        impl Serialize for $vec {
            #[inline]
            fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
                const SIZE: usize = size_of::<$stored>();
                buffer.$encode(offset, self.x as $stored)?;
                buffer.$encode(offset + SIZE, self.y as $stored)?;
                Ok(())
            }
        }
        impl Deserialize for $vec {
            #[inline]
            fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
            where
                Self: Sized,
            {
                const SIZE: usize = size_of::<$stored>();
                let x = buffer.$decode(offset)? as $num;
                let y = buffer.$decode(offset + SIZE)? as $num;
                Ok(Self { x, y })
            }
        }
    };

    // implement for 3D vector types
    ( $vec:ty [ $num:ty => $stored:ty ] [ 3 ] ; $encode:ident ; $decode:ident ) => {
        impl ConstSize for $vec {
            const SIZE: usize = size_of::<$stored>() * 3;
        }
        impl Serialize for $vec {
            #[inline]
            fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
                const SIZE: usize = size_of::<$stored>();
                buffer.$encode(offset, self.x as $stored)?;
                buffer.$encode(offset + SIZE, self.y as $stored)?;
                buffer.$encode(offset + (SIZE * 2), self.z as $stored)?;
                Ok(())
            }
        }
        impl Deserialize for $vec {
            #[inline]
            fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
            where
                Self: Sized,
            {
                const SIZE: usize = size_of::<$stored>();
                let x = buffer.$decode(offset)? as $num;
                let y = buffer.$decode(offset + SIZE)? as $num;
                let z = buffer.$decode(offset + (SIZE * 2))? as $num;
                Ok(Self { x, y, z })
            }
        }
    };

    // implement for 4D vector types
    ( $vec:ty [ $num:ty => $stored:ty ] [ 4 ] ; $encode:ident ; $decode:ident ) => {
        impl ConstSize for $vec {
            const SIZE: usize = size_of::<$stored>() * 4;
        }
        impl Serialize for $vec {
            #[inline]
            fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
                const SIZE: usize = size_of::<$stored>();
                buffer.$encode(offset, self.x as $stored)?;
                buffer.$encode(offset + SIZE, self.y as $stored)?;
                buffer.$encode(offset + (SIZE * 2), self.z as $stored)?;
                buffer.$encode(offset + (SIZE * 3), self.w as $stored)?;
                Ok(())
            }
        }
        impl Deserialize for $vec {
            #[inline]
            fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
            where
                Self: Sized,
            {
                const SIZE: usize = size_of::<$stored>();
                let x = buffer.$decode(offset)? as $num;
                let y = buffer.$decode(offset + SIZE)? as $num;
                let z = buffer.$decode(offset + (SIZE * 2))? as $num;
                let w = buffer.$decode(offset + (SIZE * 3))? as $num;
                Ok(Self { x, y, z, w })
            }
        }
    };
}

// implement for vector types

impl_packet![ Vector2i [ i32 => i32 ] [2] ; encode_s32 ; decode_s32 ];
impl_packet![ Vector3i [ i32 => i32 ] [3] ; encode_s32 ; decode_s32 ];
impl_packet![ Vector4i [ i32 => i32 ] [4] ; encode_s32 ; decode_s32 ];

impl_packet![ Vector2    [ real => f32 ] [2] ; encode_float ; decode_float ];
impl_packet![ Vector3    [ real => f32 ] [3] ; encode_float ; decode_float ];
impl_packet![ Vector4    [ real => f32 ] [4] ; encode_float ; decode_float ];
impl_packet![ Quaternion [ real => f32 ] [4] ; encode_float ; decode_float ];

// macros

macro_rules! impl_serial_field {
    ( $buffer:ident [ $offset:ident + $index:literal ] = Unit( $field:expr ) ) => {
        ($buffer.encode_s8($offset + (UNIT_SIZE * $index), to_unit($field))?)
    };

    ( $buffer:ident [ $offset:ident + $index:literal ] = Half( $field:expr ) ) => {
        ($buffer.encode_half($offset + (HALF_SIZE * $index), ($field) as real)?)
    };
}

/// Implement deserialization of a field
macro_rules! impl_deserial_field {
    ( Unit = $buffer:ident [ $offset:ident + $index:literal ] ) => {
        from_unit($buffer.decode_s8($offset + (UNIT_SIZE * $index))?)
    };

    ( Half = $buffer:ident [ $offset:ident + $index:literal ] ) => {
        (($buffer.decode_half($offset + (HALF_SIZE * $index))?) as real)
    };
}

/// Squared root
#[inline]
fn sqrt(num: real) -> real {
    godot::global::sqrt(num as f64) as real
}

// implement unit for 2D vector type

impl ConstSize for Unit<Vector2> {
    const SIZE: usize = UNIT_SIZE * 2;
}
impl Serialize for Unit<Vector2> {
    #[inline]
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        impl_serial_field![buffer[offset + 0] = Unit(self.0.x)];
        impl_serial_field![buffer[offset + 1] = Unit(self.0.y)];
        Ok(())
    }
}
impl Deserialize for Unit<Vector2> {
    #[inline]
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        let x = impl_deserial_field![Unit = buffer[offset + 0]];
        let y = impl_deserial_field![Unit = buffer[offset + 1]];
        Ok(Unit(Vector2 { x, y }.limit_length(None)))
    }
}

// implement unit for 3D vector type

impl ConstSize for Unit<Vector3> {
    const SIZE: usize = UNIT_SIZE * 3;
}
impl Serialize for Unit<Vector3> {
    #[inline]
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        impl_serial_field![buffer[offset + 0] = Unit(self.0.x)];
        impl_serial_field![buffer[offset + 1] = Unit(self.0.y)];
        impl_serial_field![buffer[offset + 2] = Unit(self.0.z)];
        Ok(())
    }
}
impl Deserialize for Unit<Vector3> {
    #[inline]
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        let x = impl_deserial_field![Unit = buffer[offset + 0]];
        let y = impl_deserial_field![Unit = buffer[offset + 1]];
        let z = impl_deserial_field![Unit = buffer[offset + 2]];
        Ok(Unit(Vector3 { x, y, z }.limit_length(None)))
    }
}

// implement unit for 4D vector type

impl ConstSize for Unit<Vector4> {
    const SIZE: usize = UNIT_SIZE * 4;
}
impl Serialize for Unit<Vector4> {
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        impl_serial_field![buffer[offset + 0] = Unit(self.0.x)];
        impl_serial_field![buffer[offset + 1] = Unit(self.0.y)];
        impl_serial_field![buffer[offset + 2] = Unit(self.0.z)];
        impl_serial_field![buffer[offset + 3] = Unit(self.0.w)];
        Ok(())
    }
}
impl Deserialize for Unit<Vector4> {
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        let x = impl_deserial_field![Unit = buffer[offset + 0]];
        let y = impl_deserial_field![Unit = buffer[offset + 1]];
        let z = impl_deserial_field![Unit = buffer[offset + 2]];
        let w = impl_deserial_field![Unit = buffer[offset + 3]];

        // construct a 4D vector
        let vec4 = Vector4 { x, y, z, w };

        // prevent the vector to be longer than 1.0
        let sqr_len = vec4.length_squared();
        Ok(Unit(if sqr_len > 1.0 {
            vec4 / sqrt(sqr_len)
        } else {
            vec4
        }))
    }
}

// implement unit for quaternion type

impl ConstSize for Unit<Quaternion> {
    const SIZE: usize = UNIT_SIZE * 4;
}
impl Serialize for Unit<Quaternion> {
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        impl_serial_field![buffer[offset + 0] = Unit(self.0.x)];
        impl_serial_field![buffer[offset + 1] = Unit(self.0.y)];
        impl_serial_field![buffer[offset + 2] = Unit(self.0.z)];
        impl_serial_field![buffer[offset + 3] = Unit(self.0.w)];
        Ok(())
    }
}
impl Deserialize for Unit<Quaternion> {
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        let x = impl_deserial_field![Unit = buffer[offset + 0]];
        let y = impl_deserial_field![Unit = buffer[offset + 1]];
        let z = impl_deserial_field![Unit = buffer[offset + 2]];
        let w = impl_deserial_field![Unit = buffer[offset + 3]];

        // construct a quaternion
        let quat = Quaternion { x, y, z, w };

        // try to normalize it
        if quat.length_squared().is_zero_approx() {
            Err(())
        } else {
            Ok(Unit(quat.normalized()))
        }
    }
}

// implement half for 2D vector type

impl ConstSize for Half<Vector2> {
    const SIZE: usize = HALF_SIZE * 2;
}
impl Serialize for Half<Vector2> {
    #[inline]
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        impl_serial_field![buffer[offset + 0] = Half(self.0.x)];
        impl_serial_field![buffer[offset + 1] = Half(self.0.y)];
        Ok(())
    }
}
impl Deserialize for Half<Vector2> {
    #[inline]
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        let x = impl_deserial_field![Half = buffer[offset + 0]];
        let y = impl_deserial_field![Half = buffer[offset + 1]];
        Ok(Half(Vector2 { x, y }))
    }
}

// implement half for 3D vector type

impl ConstSize for Half<Vector3> {
    const SIZE: usize = HALF_SIZE * 3;
}
impl Serialize for Half<Vector3> {
    #[inline]
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        impl_serial_field![buffer[offset + 0] = Half(self.0.x)];
        impl_serial_field![buffer[offset + 1] = Half(self.0.y)];
        impl_serial_field![buffer[offset + 2] = Half(self.0.z)];
        Ok(())
    }
}
impl Deserialize for Half<Vector3> {
    #[inline]
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        let x = impl_deserial_field![Half = buffer[offset + 0]];
        let y = impl_deserial_field![Half = buffer[offset + 1]];
        let z = impl_deserial_field![Half = buffer[offset + 2]];
        Ok(Half(Vector3 { x, y, z }))
    }
}

// implement half for 4D vector type

impl ConstSize for Half<Vector4> {
    const SIZE: usize = HALF_SIZE * 4;
}
impl Serialize for Half<Vector4> {
    #[inline]
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        impl_serial_field![buffer[offset + 0] = Half(self.0.x)];
        impl_serial_field![buffer[offset + 1] = Half(self.0.y)];
        impl_serial_field![buffer[offset + 2] = Half(self.0.z)];
        impl_serial_field![buffer[offset + 3] = Half(self.0.w)];
        Ok(())
    }
}
impl Deserialize for Half<Vector4> {
    #[inline]
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        let x = impl_deserial_field![Half = buffer[offset + 0]];
        let y = impl_deserial_field![Half = buffer[offset + 1]];
        let z = impl_deserial_field![Half = buffer[offset + 2]];
        let w = impl_deserial_field![Half = buffer[offset + 3]];
        Ok(Half(Vector4 { x, y, z, w }))
    }
}

// implement half for quaternion type

impl ConstSize for Half<Quaternion> {
    const SIZE: usize = HALF_SIZE * 4;
}
impl Serialize for Half<Quaternion> {
    #[inline]
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        impl_serial_field![buffer[offset + 0] = Half(self.0.x)];
        impl_serial_field![buffer[offset + 1] = Half(self.0.y)];
        impl_serial_field![buffer[offset + 2] = Half(self.0.z)];
        impl_serial_field![buffer[offset + 3] = Half(self.0.w)];
        Ok(())
    }
}
impl Deserialize for Half<Quaternion> {
    #[inline]
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        let x = impl_deserial_field![Half = buffer[offset + 0]];
        let y = impl_deserial_field![Half = buffer[offset + 1]];
        let z = impl_deserial_field![Half = buffer[offset + 2]];
        let w = impl_deserial_field![Half = buffer[offset + 3]];

        // construct a quaternion
        let quat = Quaternion { x, y, z, w };

        // try to normalize it
        if quat.length_squared().is_zero_approx() {
            Err(())
        } else {
            Ok(Half(quat.normalized()))
        }
    }
}
