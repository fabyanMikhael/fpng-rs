use autocxx::prelude::*;
use image::{RgbImage, RgbaImage};

include_cpp! {
    #include "fpng-rs.h"
    safety!(unsafe)
    generate!("encode_buffer")
    generate!("init_fpng")
}

pub struct Encoder {}

impl Encoder {
    #[inline(always)]
    pub fn init_fpng() {
        ffi::init_fpng();
    }

    #[inline(always)]
    fn _encode_image(image_start: *mut u8, w: u32, h: u32, channels: u32) -> Option<Vec<u8>> {
        let mut failed: bool = false;
        let out =
            unsafe { ffi::encode_buffer(image_start, w, h, channels, &mut failed as *mut bool) };

        if failed {
            Some(out.into_iter().copied().collect())
        } else {
            None
        }
    }
}

pub trait Encode<T> {
    fn encode(data: &T) -> Option<Vec<u8>>;
}

impl Encode<RgbaImage> for Encoder {
    #[inline(always)]
    fn encode(im: &RgbaImage) -> Option<Vec<u8>> {
        let (w, h) = im.dimensions();

        let image_start = im.as_ptr() as *mut u8;

        Encoder::_encode_image(image_start, w, h, 4)
    }
}

impl Encode<RgbImage> for Encoder {
    #[inline(always)]
    fn encode(im: &RgbImage) -> Option<Vec<u8>> {
        let (w, h) = im.dimensions();

        let image_start = im.as_ptr() as *mut u8;

        Encoder::_encode_image(image_start, w, h, 3)
    }
}
