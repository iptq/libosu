pub use lzma_impl::{decode, encode_writer};

// TODO: static assert that either lzma-rs or lzma-sys needs to be enabled
// static_assertions::assert_cfg!(any(lzma_rs, lzma_sys));

// TODO: add lzma-rs implementation

#[cfg(all(feature = "lzma-sys", not(feature = "lzma-rs")))]
mod lzma_impl {
    use std::{io::Read, io::Write};

    use super::super::ReplayResult;

    pub fn decode(input: impl Read) -> ReplayResult<Vec<u8>> {
        use std::io::BufReader;
        use xz2::bufread::XzDecoder;
        use xz2::stream::Stream;

        let lzma_decoder = Stream::new_lzma_decoder(std::u64::MAX)?;
        let data_reader = BufReader::new(input);
        let mut xz_decoder = XzDecoder::new_stream(data_reader, lzma_decoder);

        let mut data = Vec::new();
        xz_decoder.read_to_end(&mut data)?;

        Ok(data)
    }

    #[cfg(all(feature = "lzma-sys", not(feature = "lzma-rs")))]
    pub fn encode_writer(target: &mut Vec<u8>) -> ReplayResult<impl Write + '_> {
        use xz2::{
            stream::{LzmaOptions, Stream},
            write::XzEncoder,
        };

        // write thru the encoder
        // TODO: presets? options?
        let opts = LzmaOptions::new_preset(0)?;
        let stream = Stream::new_lzma_encoder(&opts)?;
        let xz = XzEncoder::new_stream(target, stream);

        Ok(xz)
    }
}
