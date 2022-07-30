pub use lzma_impl::{decode, encode};

static_assertions::assert_cfg!(
    any(feature = "lzma-rs", feature = "lzma-sys"),
    "You must enable either lzma-rs or lzma-sys to use the replay-data feature"
);

static_assertions::assert_cfg!(
    any(not(feature = "lzma-rs"), not(feature = "lzma-sys")),
    "You cannot enable both lzma-rs and lzma-sys"
);

/// Stub module to avoid extra noise in compiler errors. Only the static assert failures should be shown.
#[cfg(any(
    all(not(feature = "lzma-rs"), not(feature = "lzma-sys")),
    all(feature = "lzma-rs", feature = "lzma-sys")
))]
mod lzma_impl {
    use std::io::BufRead;

    use super::super::ReplayResult;

    pub fn decode(_input: impl BufRead) -> ReplayResult<Vec<u8>> {
        unimplemented!()
    }

    pub fn encode(_input: impl BufRead) -> ReplayResult<Vec<u8>> {
        unimplemented!()
    }
}

// TODO: add lzma-rs implementation
#[cfg(all(feature = "lzma-rs", not(feature = "lzma-sys")))]
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

#[cfg(all(feature = "lzma-sys", not(feature = "lzma-rs")))]
mod lzma_impl {
    use std::{io::BufRead, io::Read};
    use xz2::bufread::{XzDecoder, XzEncoder};
    use xz2::stream::{LzmaOptions, Stream};

    use super::super::ReplayResult;

    pub fn decode(input: impl BufRead) -> ReplayResult<Vec<u8>> {
        let lzma_decoder = Stream::new_lzma_decoder(std::u64::MAX)?;
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
