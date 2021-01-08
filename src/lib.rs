//! # libosu
//!
//! `libosu` is an attempt to make a convenient library for writing osu!-related programs. It
//! includes data structures and parsers for beatmaps, replays, and more.
//!
//! Please note that until this crate hits `1.0`, none of the APIs in this crate will be stable, so
//! take care when using this crate.

#![deny(missing_docs)]

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate thiserror;

#[cfg(feature = "apiv1")]
mod apiv1;
mod beatmap;
mod color;

mod enums;
mod hitobject;
mod hitsounds;
mod math;
mod osudb;
mod parsing;
#[cfg(feature = "replay")]
mod replay;
mod spline;
mod timing;

#[cfg(feature = "apiv1")]
pub use apiv1::*;
pub use beatmap::*;
pub use color::*;
pub use enums::*;
pub use hitobject::*;
pub use hitsounds::*;
pub use math::*;
pub use osudb::*;
#[cfg(feature = "replay")]
pub use replay::*;
pub use spline::*;
pub use timing::*;

macro_rules! read_num_le_fn {
    ($name:ident => $typ:ident) => {
        pub(crate) fn $name<R: std::io::Read>(reader: &mut R) -> Result<$typ, anyhow::Error> {
            let mut buf: [u8; std::mem::size_of::<$typ>()] = [0; std::mem::size_of::<$typ>()];
            reader.read_exact(&mut buf)?;
            Ok($typ::from_le_bytes(buf))
        }
    };
}

// conviences functions to read a number from an `io::Read`
//   another option here is to use the `byteorder` crate, which has a nicer implementation of this.
//   but since we only need 5 functions and the implementation is _really_ simple this is fine.

read_num_le_fn!(read_u8 => u8);
read_num_le_fn!(read_u16le => u16);
read_num_le_fn!(read_u32le => u32);
read_num_le_fn!(read_u64le => u64);
read_num_le_fn!(read_f32le => f32);
read_num_le_fn!(read_f64le => f64);

/// Read an unsigned LEB128 encoded number
/// Although the number is variable width, this will only read up to a 63 bit number (9 bytes)
pub(crate) fn read_uleb128<R: std::io::Read>(reader: &mut R) -> Result<u64, anyhow::Error> {
    let mut buf: [u8; 1] = [0];
    let mut byte_index = 0;

    reader.read_exact(&mut buf)?;

    let mut total = (buf[0] & 0b01111111) as u64;

    while (buf[0] & 0b10000000) == 0b10000000 {
        byte_index += 1;
        if byte_index > 9 {
            bail!("ULEB128 number may overflow u64");
        }

        reader.read_exact(&mut buf)?;

        total += ((buf[0] & 0b01111111) as u64) << (7 * byte_index)
    }

    Ok(total)
}

/// read an string from an osu! uleb128.
pub(crate) fn read_uleb128_string<R: std::io::Read>(
    reader: &mut R,
) -> Result<String, anyhow::Error> {
    let mut empty_string_byte_buffer: [u8; 1] = [0];
    reader.read_exact(&mut empty_string_byte_buffer)?;

    // 0 means the string is not present
    if empty_string_byte_buffer[0] == 0 {
        return Ok(String::new());
    }

    if empty_string_byte_buffer[0] != 0x0B {
        bail!(
            "expected string to start with either 0x0B or 0x00, got {:x}",
            empty_string_byte_buffer[0]
        );
    }

    let length = read_uleb128(reader)?;
    if length == 0 {
        return Ok(String::new());
    }

    let mut read_buffer = vec![0u8; length as usize];
    reader.read_exact(read_buffer.as_mut())?;

    Ok(String::from_utf8(read_buffer)?)
}

/// Says "hello there"
#[deprecated]
pub fn say_hello_there() {
    println!("hello there");
}
