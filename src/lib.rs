#![no_std]

extern crate minimp3_sys as ffi;
pub const MAX_SAMPLES_PER_FRAME: usize = ffi::MINIMP3_MAX_SAMPLES_PER_FRAME as usize;
use core::mem::MaybeUninit;

#[derive(Debug)]
pub enum DecodeResult<'a> {
    Successful(usize, Metadata<'a>),
    SkippedData(usize),
    InsufficientData,
}

#[derive(Debug)]
pub struct Metadata<'a> {
    pub channels: usize,
    pub sample_count: usize,
    pub sample_rate: u32,
    pub samples: &'a [i16],
}

pub struct DecoderData {
    dec: MaybeUninit<ffi::mp3dec_t>,
    pcm: [i16; MAX_SAMPLES_PER_FRAME],
}

impl DecoderData {
    pub const fn new() -> Self {
        let dec = MaybeUninit::uninit();
        Self {
            dec,
            pcm: [0; MAX_SAMPLES_PER_FRAME],
        }
    }
}

pub struct Decoder<'a> {
    dec: &'a mut MaybeUninit<ffi::mp3dec_t>,
    pcm: &'a mut [i16; MAX_SAMPLES_PER_FRAME],
}

impl<'a> Decoder<'a> {
    pub fn new(data: &'a mut DecoderData) -> Self {
        unsafe {
            ffi::mp3dec_init(data.dec.as_mut_ptr());
        };
        Self {
            dec: &mut data.dec,
            pcm: &mut data.pcm,
        }
    }

    pub fn decode(&mut self, data: &(impl AsRef<[u8]> + ?Sized)) -> DecodeResult {
        let data = data.as_ref();
        let out_ptr: *mut i16 = self.pcm.as_mut_ptr();
        let buf_size = data.len() as usize;
        let data_ptr: *const u8 = data.as_ptr();
        let mut ffi_frame: MaybeUninit<ffi::mp3dec_frame_info_t> = MaybeUninit::uninit();

        let ffi_frame_ptr = ffi_frame.as_mut_ptr();
        let sample_count: cty::c_int = unsafe {
            ffi::mp3dec_decode_frame(
                self.dec.as_mut_ptr(),
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
                    samples: &self.pcm[..],
                },
            )
        } else if ffi_frame.frame_bytes > 0 {
            DecodeResult::SkippedData(ffi_frame.frame_bytes.max(0) as usize)
        } else {
            DecodeResult::InsufficientData
        }
    }
}
