//! # embedded-mp3
//!
//! > Wrapper over minimp3 library for embedded rust. One important functionality is
//! that it allows the input mp3 to be streamed into the library.
//!
//! For more about minimp3 see https://github.com/lieff/minimp3
//!
#![no_std]

pub const MAX_SAMPLES_PER_FRAME: usize = ffi::MINIMP3_MAX_SAMPLES_PER_FRAME as usize;
use core::mem::{self, MaybeUninit};
use core::ptr;
mod ffi;

/// Metadata of a decoded frame
#[derive(Debug)]
pub struct Metadata {
    pub channels: usize,
    pub sample_count: usize,
    pub sample_rate: u32,
}

/// Result of decoding
#[derive(Debug)]
pub enum DecodeResult {
    /// Decoding was successful
    Successful(usize, Metadata),
    /// Frames were skipped
    SkippedData(usize),
    /// Can't decode frame
    InsufficientData,
}

/// Wrapper object over the minimp3 decoder
pub struct Decoder {
    dec: ffi::mp3dec_t,
}

impl Decoder {
    pub fn new() -> Self {
        Self {
            dec: unsafe {
                let mut decoder: ffi::mp3dec_t = mem::zeroed();
                ffi::mp3dec_init(&mut decoder);
                decoder
            },
        }
    }

    /// Decode a frame
    /// * `data`: mp3 data as a u8 slice
    /// * `pcm`: Output buffer, where decoded data is placed
    /// * returns: Enumeration DecodeResult
    pub fn decode<'b>(
        &mut self,
        data: &(impl AsRef<[u8]> + ?Sized),
        pcm: &'b mut [i16; MAX_SAMPLES_PER_FRAME],
    ) -> DecodeResult {
        self.ffi_decode(data, pcm)
    }

    fn ffi_decode<'b>(
        &mut self,
        data: &(impl AsRef<[u8]> + ?Sized),
        pcm: &'b mut [i16],
    ) -> DecodeResult {
        let data = data.as_ref();
        let buf_size = data.len() as usize;
        let data_ptr: *const u8 = data.as_ptr();
        let mut ffi_frame: MaybeUninit<ffi::mp3dec_frame_info_t> = MaybeUninit::uninit();

        let out_ptr: *mut i16 = if pcm.len() == 0 {
            ptr::null_mut()
        } else {
            pcm.as_mut_ptr()
        };
        let ffi_frame_ptr = ffi_frame.as_mut_ptr();
        let sample_count: cty::c_int = unsafe {
            ffi::mp3dec_decode_frame(
                &mut self.dec,
                data_ptr,
                buf_size as cty::c_int,
                out_ptr,
                ffi_frame_ptr,
            )
        };
        let ffi_frame = unsafe { ffi_frame.assume_init() };
        if sample_count != 0 {
            DecodeResult::Successful(
                ffi_frame.frame_bytes.max(0) as usize,
                Metadata {
                    channels: ffi_frame.channels.max(0) as usize,
                    sample_count: sample_count.max(0) as usize,
                    sample_rate: ffi_frame.hz.max(0) as u32,
                },
            )
        } else if ffi_frame.frame_bytes > 0 {
            DecodeResult::SkippedData(ffi_frame.frame_bytes.max(0) as usize)
        } else {
            DecodeResult::InsufficientData
        }
    }
}
