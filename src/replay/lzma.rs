pub use lzma_impl::{decode, encode};

static_assertions::assert_cfg!(
    not(all(feature = "replay-data", feature = "replay-data-xz2")),
    "Cannot enable both replay-data (pure-Rust) and replay-data-xz2 (native xz2) at the same time.",
);

#[cfg(all(feature = "replay-data", not(feature = "replay-data-xz2")))]
mod lzma_impl {
    use std::io::BufRead;

    use lzma_rs::{lzma_compress, lzma_decompress};

    use super::super::ReplayResult;

    pub fn decode(mut input: impl BufRead) -> ReplayResult<Vec<u8>> {
        let mut data = Vec::new();
        lzma_decompress(&mut input, &mut data)?;
        Ok(data)
    }

    pub fn encode(mut input: impl BufRead) -> ReplayResult<Vec<u8>> {
        let mut data = Vec::new();
        lzma_compress(&mut input, &mut data)?;
        Ok(data)
    }
}

#[cfg(all(feature = "replay-data-xz2", not(feature = "replay-data")))]
mod lzma_impl {
    use std::{io::BufRead, io::Read};
    use xz2::bufread::{XzDecoder, XzEncoder};
    use xz2::stream::{LzmaOptions, Stream};

    use super::super::ReplayResult;

    pub fn decode(input: impl BufRead) -> ReplayResult<Vec<u8>> {
        let lzma_decoder = Stream::new_lzma_decoder(std::u32::MAX as u64)?;
        let mut xz_decoder = XzDecoder::new_stream(input, lzma_decoder);

        let mut data = Vec::new();
        xz_decoder.read_to_end(&mut data)?;

        Ok(data)
    }

    pub fn encode(input: impl BufRead) -> ReplayResult<Vec<u8>> {
        let lzma_decoder = Stream::new_lzma_encoder(&LzmaOptions::new_preset(6)?)?;
        let mut xz_encoder = XzEncoder::new_stream(input, lzma_decoder);

        let mut data = Vec::new();
        xz_encoder.read_to_end(&mut data)?;

        Ok(data)
    }
}
