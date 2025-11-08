//!
//! Define serialization and deserialization for tuples and arrays.
//!

use super::{ConstSize, Deserialize, Serialize};
use godot::builtin::*;
use std::{
    mem::{MaybeUninit, forget},
    ptr::read,
};

// implement for static arrays

impl<T, const S: usize> ConstSize for [T; S]
where
    T: ConstSize,
{
    const SIZE: usize = T::SIZE * S;
}

impl<T, const S: usize> Serialize for [T; S]
where
    T: Serialize + ConstSize,
{
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        for (i, t) in self.iter().enumerate() {
            let _ = t.serialize(buffer, offset + i * T::SIZE)?;
        }
        Ok(())
    }
}

impl<T, const S: usize> Deserialize for [T; S]
where
    T: Deserialize + ConstSize,
{
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        // SAFETY: creating uninitialized array of MaybeUninit<T>
        let mut array: [MaybeUninit<T>; S] = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..S {
            array[i].write(T::deserialize(buffer, offset + i * T::SIZE)?);
        }

        // SAFETY: all elements initialized, so this is valid
        let initialized = unsafe { read(&array as *const _ as *const [T; S]) };

        // Prevent dropping of uninitialized memory
        forget(array);

        Ok(initialized)
    }
}

// implement for tuples

impl<T0> ConstSize for (T0,)
where
    T0: ConstSize,
{
    const SIZE: usize = T0::SIZE;
}

impl<T0> Serialize for (T0,)
where
    T0: Serialize + ConstSize,
{
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        let _ = self.0.serialize(buffer, offset)?;
        Ok(())
    }
}

impl<T0> Deserialize for (T0,)
where
    T0: Deserialize + ConstSize,
{
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        Ok((T0::deserialize(buffer, offset)?,))
    }
}

impl<T0, T1> ConstSize for (T0, T1)
where
    T0: ConstSize,
    T1: ConstSize,
{
    const SIZE: usize = T0::SIZE + T1::SIZE;
}

impl<T0, T1> Serialize for (T0, T1)
where
    T0: Serialize + ConstSize,
    T1: Serialize + ConstSize,
{
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        let _ = self.0.serialize(buffer, offset)?;
        let _ = self.1.serialize(buffer, offset + T0::SIZE)?;
        Ok(())
    }
}

impl<T0, T1> Deserialize for (T0, T1)
where
    T0: Deserialize + ConstSize,
    T1: Deserialize + ConstSize,
{
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        Ok((
            T0::deserialize(buffer, offset)?,
            T1::deserialize(buffer, offset + T0::SIZE)?,
        ))
    }
}

impl<T0, T1, T2> ConstSize for (T0, T1, T2)
where
    T0: ConstSize,
    T1: ConstSize,
    T2: ConstSize,
{
    const SIZE: usize = T0::SIZE + T1::SIZE + T2::SIZE;
}

impl<T0, T1, T2> Serialize for (T0, T1, T2)
where
    T0: Serialize + ConstSize,
    T1: Serialize + ConstSize,
    T2: Serialize + ConstSize,
{
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        let i0 = offset;
        let i1 = i0 + T0::SIZE;
        let i2 = i1 + T1::SIZE;

        let _ = self.0.serialize(buffer, i0)?;
        let _ = self.1.serialize(buffer, i1)?;
        let _ = self.2.serialize(buffer, i2)?;
        Ok(())
    }
}

impl<T0, T1, T2> Deserialize for (T0, T1, T2)
where
    T0: Deserialize + ConstSize,
    T1: Deserialize + ConstSize,
    T2: Deserialize + ConstSize,
{
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        let i0 = offset;
        let i1 = i0 + T0::SIZE;
        let i2 = i1 + T1::SIZE;

        Ok((
            T0::deserialize(buffer, i0)?,
            T1::deserialize(buffer, i1)?,
            T2::deserialize(buffer, i2)?,
        ))
    }
}

impl<T0, T1, T2, T3> ConstSize for (T0, T1, T2, T3)
where
    T0: ConstSize,
    T1: ConstSize,
    T2: ConstSize,
    T3: ConstSize,
{
    const SIZE: usize = T0::SIZE + T1::SIZE + T2::SIZE + T3::SIZE;
}

impl<T0, T1, T2, T3> Serialize for (T0, T1, T2, T3)
where
    T0: Serialize + ConstSize,
    T1: Serialize + ConstSize,
    T2: Serialize + ConstSize,
    T3: Serialize + ConstSize,
{
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        let i0 = offset;
        let i1 = i0 + T0::SIZE;
        let i2 = i1 + T1::SIZE;
        let i3 = i2 + T2::SIZE;

        let _ = self.0.serialize(buffer, i0)?;
        let _ = self.1.serialize(buffer, i1)?;
        let _ = self.2.serialize(buffer, i2)?;
        let _ = self.3.serialize(buffer, i3)?;
        Ok(())
    }
}

impl<T0, T1, T2, T3> Deserialize for (T0, T1, T2, T3)
where
    T0: Deserialize + ConstSize,
    T1: Deserialize + ConstSize,
    T2: Deserialize + ConstSize,
    T3: Deserialize + ConstSize,
{
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        let i0 = offset;
        let i1 = i0 + T0::SIZE;
        let i2 = i1 + T1::SIZE;
        let i3 = i2 + T2::SIZE;

        Ok((
            T0::deserialize(buffer, i0)?,
            T1::deserialize(buffer, i1)?,
            T2::deserialize(buffer, i2)?,
            T3::deserialize(buffer, i3)?,
        ))
    }
}

impl<T0, T1, T2, T3, T4> ConstSize for (T0, T1, T2, T3, T4)
where
    T0: ConstSize,
    T1: ConstSize,
    T2: ConstSize,
    T3: ConstSize,
    T4: ConstSize,
{
    const SIZE: usize = T0::SIZE + T1::SIZE + T2::SIZE + T3::SIZE + T4::SIZE;
}

impl<T0, T1, T2, T3, T4> Serialize for (T0, T1, T2, T3, T4)
where
    T0: Serialize + ConstSize,
    T1: Serialize + ConstSize,
    T2: Serialize + ConstSize,
    T3: Serialize + ConstSize,
    T4: Serialize + ConstSize,
{
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        let i0 = offset;
        let i1 = i0 + T0::SIZE;
        let i2 = i1 + T1::SIZE;
        let i3 = i2 + T2::SIZE;
        let i4 = i3 + T3::SIZE;

        let _ = self.0.serialize(buffer, i0)?;
        let _ = self.1.serialize(buffer, i1)?;
        let _ = self.2.serialize(buffer, i2)?;
        let _ = self.3.serialize(buffer, i3)?;
        let _ = self.4.serialize(buffer, i4)?;
        Ok(())
    }
}

impl<T0, T1, T2, T3, T4> Deserialize for (T0, T1, T2, T3, T4)
where
    T0: Deserialize + ConstSize,
    T1: Deserialize + ConstSize,
    T2: Deserialize + ConstSize,
    T3: Deserialize + ConstSize,
    T4: Deserialize + ConstSize,
{
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        let i0 = offset;
        let i1 = i0 + T0::SIZE;
        let i2 = i1 + T1::SIZE;
        let i3 = i2 + T2::SIZE;
        let i4 = i3 + T3::SIZE;

        Ok((
            T0::deserialize(buffer, i0)?,
            T1::deserialize(buffer, i1)?,
            T2::deserialize(buffer, i2)?,
            T3::deserialize(buffer, i3)?,
            T4::deserialize(buffer, i4)?,
        ))
    }
}

impl<T0, T1, T2, T3, T4, T5> ConstSize for (T0, T1, T2, T3, T4, T5)
where
    T0: ConstSize,
    T1: ConstSize,
    T2: ConstSize,
    T3: ConstSize,
    T4: ConstSize,
    T5: ConstSize,
{
    const SIZE: usize = T0::SIZE + T1::SIZE + T2::SIZE + T3::SIZE + T4::SIZE + T5::SIZE;
}

impl<T0, T1, T2, T3, T4, T5> Serialize for (T0, T1, T2, T3, T4, T5)
where
    T0: Serialize + ConstSize,
    T1: Serialize + ConstSize,
    T2: Serialize + ConstSize,
    T3: Serialize + ConstSize,
    T4: Serialize + ConstSize,
    T5: Serialize + ConstSize,
{
    fn serialize(&self, buffer: &mut PackedByteArray, offset: usize) -> Result<(), ()> {
        let i0 = offset;
        let i1 = i0 + T0::SIZE;
        let i2 = i1 + T1::SIZE;
        let i3 = i2 + T2::SIZE;
        let i4 = i3 + T3::SIZE;
        let i5 = i4 + T4::SIZE;

        let _ = self.0.serialize(buffer, i0)?;
        let _ = self.1.serialize(buffer, i1)?;
        let _ = self.2.serialize(buffer, i2)?;
        let _ = self.3.serialize(buffer, i3)?;
        let _ = self.4.serialize(buffer, i4)?;
        let _ = self.5.serialize(buffer, i5)?;
        Ok(())
    }
}

impl<T0, T1, T2, T3, T4, T5> Deserialize for (T0, T1, T2, T3, T4, T5)
where
    T0: Deserialize + ConstSize,
    T1: Deserialize + ConstSize,
    T2: Deserialize + ConstSize,
    T3: Deserialize + ConstSize,
    T4: Deserialize + ConstSize,
    T5: Deserialize + ConstSize,
{
    fn deserialize(buffer: &PackedByteArray, offset: usize) -> Result<Self, ()>
    where
        Self: Sized,
    {
        let i0 = offset;
        let i1 = i0 + T0::SIZE;
        let i2 = i1 + T1::SIZE;
        let i3 = i2 + T2::SIZE;
        let i4 = i3 + T3::SIZE;
        let i5 = i4 + T4::SIZE;

        Ok((
            T0::deserialize(buffer, i0)?,
            T1::deserialize(buffer, i1)?,
            T2::deserialize(buffer, i2)?,
            T3::deserialize(buffer, i3)?,
            T4::deserialize(buffer, i4)?,
            T5::deserialize(buffer, i5)?,
        ))
    }
}
