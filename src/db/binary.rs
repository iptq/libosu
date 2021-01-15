//! Functions and macros for reading data structures

/// Result type for ReadError
pub type ReadResult<T, E = ReadError> = std::result::Result<T, E>;

/// Errors that could occur while reading from binary files
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum ReadError {
    #[error("utf8 decoding error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("uleb128 number may overflow u64")]
    UlebOverflow,

    #[error("expected string to start with either 0x0B or 0x00, got {0}")]
    MalformedString(u8),
}

macro_rules! read_num_le_fn {
    ($name:ident => $typ:ident) => {
        pub(crate) fn $name<R: std::io::Read>(reader: &mut R) -> Result<$typ, ReadError> {
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
pub fn read_uleb128<R: std::io::Read>(reader: &mut R) -> Result<u64, ReadError> {
    let mut buf: [u8; 1] = [0];
    let mut byte_index = 0;

    reader.read_exact(&mut buf)?;

    let mut total = (buf[0] & 0b01111111) as u64;

    while (buf[0] & 0b10000000) == 0b10000000 {
        byte_index += 1;
        if byte_index > 9 {
            return Err(ReadError::UlebOverflow);
        }

        reader.read_exact(&mut buf)?;

        total += ((buf[0] & 0b01111111) as u64) << (7 * byte_index)
    }

    Ok(total)
}

/// read an string from an osu! uleb128.
pub fn read_uleb128_string<R: std::io::Read>(reader: &mut R) -> Result<String, ReadError> {
    let mut empty_string_byte_buffer: [u8; 1] = [0];
    reader.read_exact(&mut empty_string_byte_buffer)?;

    // 0 means the string is not present
    if empty_string_byte_buffer[0] == 0 {
        return Ok(String::new());
    }

    if empty_string_byte_buffer[0] != 0x0B {
        return Err(ReadError::MalformedString(empty_string_byte_buffer[0]));
    }

    let length = read_uleb128(reader)?;
    if length == 0 {
        return Ok(String::new());
    }

    let mut read_buffer = vec![0u8; length as usize];
    reader.read_exact(read_buffer.as_mut())?;

    Ok(String::from_utf8(read_buffer)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_read_uleb128() {
        let mut num = io::Cursor::new([0xE5, 0x8E, 0x26]);
        assert_eq!(read_uleb128(&mut num).unwrap(), 624485u64);
    }

    #[test]
    fn test_read_uleb128_string() {
        let text = "Hello World";
        let mut replay_string = vec![0x0Bu8];
        replay_string.push(text.len() as u8);
        replay_string.extend(text.bytes());

        let mut reader = io::Cursor::new(replay_string);
        assert_eq!(read_uleb128_string(&mut reader).unwrap(), text.to_string());

        let mut reader_empty = io::Cursor::new(vec![0x00]);
        assert_eq!(
            read_uleb128_string(&mut reader_empty).unwrap(),
            String::new()
        );

        let mut reader_zero_length = io::Cursor::new(vec![0x0B, 0x00]);
        assert_eq!(
            read_uleb128_string(&mut reader_zero_length).unwrap(),
            String::new()
        );
    }
}
