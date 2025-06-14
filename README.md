# Godot Network Packet

Simple library for building network packets for Godot engine.
This crates provides derive macros to implement binary serialization and deserialization of Rust's structs containing Godot's primitive types.
It also provides wrapper to optimize the serialized size of some types.

## Traits

This library provides the following traits:
- `ConstSize`: define the constant size of the serialized type, this **must** match the number of bytes written in the `Serialize` implementation and the number of bytes read in the `Deserialize` implementation.
- `Serialize`: define how to convert the type to a raw sequence of bytes.
- `Deserialize`: define how to recover the type from a raw sequence of bytes.

`Serialize` and `Deserialize` may be used without the `ConstSize` trait for types with dynamic size such as strings, sequences and maps.

## Wrappers

This library also provide two wrapper types:
- `Half<T>`: will use `encode_half` and `decode_half` to write and read floating point numbers thus using two bytes instead of four.
- `Unit<T>`: assuming the floating point number is in the range [-1, 1], it will be encoded using a single byte instead of four.
