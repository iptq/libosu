//! Convenience traits for dealing with OSU binary data.

use std::io::{Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};

/// Result for Error
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Errors that could arise from reading binary beatmap data
#[derive(Debug, Error)]
pub enum Error {
  /// IO Error
  #[error("io error: {0}")]
  Io(#[from] std::io::Error),

  /// UTF8 String conversion error
  #[error("string conversion error: {0}")]
  Utf8(#[from] std::string::FromUtf8Error),

  /// ULEB overflows 128 bits
  #[error("uleb error")]
  UlebOverflow,

  /// Character in front of the string is not 0x0 or 0xb
  #[error("invalid string status char: {0}")]
  InvalidStringStatusChar(u8),
}

/// Extends Read with more reading functions specific to OSU
pub trait ReadBytesOsu: Read {
  /// Read data from the reader in [ULEB128][1] format
  ///
  /// [1]: https://en.wikipedia.org/wiki/LEB128#Unsigned_LEB128
  fn read_uleb128(&mut self) -> Result<u128> {
    let mut buf = [0];
    let mut byte_index = 0;
    self.read_exact(&mut buf)?;

    let mut total = (buf[0] & 0b01111111) as u128;
    while (buf[0] & 0b10000000) == 0b10000000 {
      byte_index += 1;
      if byte_index > 9 {
        return Err(Error::UlebOverflow);
      }

      self.read_exact(&mut buf)?;
      total += ((buf[0] & 0b01111111) as u128) << (7 * byte_index)
    }

    Ok(total)
  }

  /// Read a string from the reader
  ///
  /// The first byte indicates whether a string is there (0x0B) or not (0x00). Then the length is
  /// encoded as a ULEB128 number, and then finally the string itself is encoded as UTF-8
  fn read_uleb128_string(&mut self) -> Result<String> {
    match self.read_u8()? {
      // string isn't there
      0x0 => Ok(String::new()),

      // read string normally
      0xb => {
        let len = self.read_uleb128()?;
        if len == 0 {
          return Ok(String::new());
        }

        let mut buf = vec![0; len as usize];
        self.read_exact(&mut buf)?;
        let string = String::from_utf8(buf)?;
        Ok(string)
      }

      // error
      v => Err(Error::InvalidStringStatusChar(v)),
    }
  }
}

impl<R: Read + ?Sized> ReadBytesOsu for R {}

/// Extends Write with more writing functions specific to OSU
pub trait WriteBytesOsu: Write {
  /// Writes a ULEB128 value into the writer
  fn write_uleb128(&mut self, mut n: u128) -> Result<()> {
    while n > 0 {
      let mut byte = (n & 0x7fu128) as u8;
      n >>= 7;
      if n > 0 {
        // more bytes
        byte |= 1 << 7;
      }
      self.write_u8(byte)?;
    }
    Ok(())
  }

  /// Writes a string value into the writer
  fn write_uleb128_string(&mut self, string: impl AsRef<str>) -> Result<()> {
    let string = string.as_ref();
    if string.is_empty() {
      self.write_u8(0x0)?;
      return Ok(());
    }

    self.write_u8(0xb)?;
    self.write_uleb128(string.len() as u128)?;
    self.write_all(string.as_bytes())?;
    Ok(())
  }
}

impl<W: Write> WriteBytesOsu for W {}

#[cfg(test)]
mod tests {
  use super::*;
  use anyhow::Result;
  use std::io::Cursor;

  #[test]
  fn test_read_uleb128() -> Result<()> {
    let mut num = Cursor::new([0xE5, 0x8E, 0x26]);
    assert_eq!(num.read_uleb128()?, 624485);
    Ok(())
  }

  #[test]
  fn test_write_uleb128() -> Result<()> {
    let mut buf = Vec::new();
    let mut curs = Cursor::new(&mut buf);
    curs.write_uleb128(624485)?;
    assert_eq!(buf, [0xe5, 0x8e, 0x26]);
    Ok(())
  }

  #[test]
  fn test_read_uleb128_string() -> Result<()> {
    let text = "Hello World";
    let mut replay_string = vec![0x0Bu8];
    replay_string.push(text.len() as u8);
    replay_string.extend(text.bytes());

    let mut reader = Cursor::new(replay_string);
    assert_eq!(reader.read_uleb128_string()?, text.to_string());

    let mut reader_empty = Cursor::new(vec![0x00]);
    assert_eq!(reader_empty.read_uleb128_string()?, String::new());

    let mut reader_0_len = Cursor::new(vec![0x0B, 0x00]);
    assert_eq!(reader_0_len.read_uleb128_string()?, String::new());
    Ok(())
  }

  #[test]
  fn test_write_uleb128_string() -> Result<()> {
    let mut buf = Vec::new();
    let mut curs = Cursor::new(&mut buf);
    curs.write_uleb128_string("Hello, world!")?;
    assert_eq!(buf, b"\x0b\x0dHello, world!");
    Ok(())
  }
}
