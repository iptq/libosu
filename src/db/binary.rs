//! Convenience traits for dealing with OSU binary data.

use std::io::{Read, Write};

use byteorder::ReadBytesExt;

/// Result for Error
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Errors that could arise from reading binary beatmap data
#[derive(Debug, Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("string conversion error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("uleb error")]
    UlebOverflow,

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
pub trait WriteBytesOsu: Write {}

impl<W: Write> WriteBytesOsu for W {}
