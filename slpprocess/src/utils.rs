use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Incorrect file type, expected '.slp', got {0}")]
    FileType(String),
    #[error("Replay must have exactly 2 human players")]
    PlayerCount,
    #[error("Expected {0}, got {1}")]
    Value(String, String),
}

use std::mem;
use std::{cmp, ptr};

use bytes::Bytes;

macro_rules! buf_get_impl {
    ($this:ident, $typ:tt::$conv:tt) => {{
        const SIZE: usize = mem::size_of::<$typ>();
        // try to convert directly from the bytes
        // this Option<ret> trick is to avoid keeping a borrow on self
        // when advance() is called (mut borrow) and to call bytes() only once
        let ret = $this
            .chunk()
            .get(..SIZE)
            .map(|src| unsafe { $typ::$conv(*(src as *const _ as *const [_; SIZE])) });

        if let Some(ret) = ret {
            // if the direct conversion was possible, advance and return
            $this.advance(SIZE);
            return ret;
        } else {
            // if not we copy the bytes in a temp buffer then convert
            let mut buf = [0; SIZE];
            $this.copy_to_slice_unchecked(&mut buf); // (do the advance)
            return $typ::$conv(buf);
        }
    }};
    (le => $this:ident, $typ:tt, $len_to_read:expr) => {{
        debug_assert!(mem::size_of::<$typ>() >= $len_to_read);

        // The same trick as above does not improve the best case speed.
        // It seems to be linked to the way the method is optimised by the compiler
        let mut buf = [0; (mem::size_of::<$typ>())];
        $this.copy_to_slice_unchecked(&mut buf[..($len_to_read)]);
        return $typ::from_le_bytes(buf);
    }};
    (be => $this:ident, $typ:tt, $len_to_read:expr) => {{
        debug_assert!(mem::size_of::<$typ>() >= $len_to_read);

        let mut buf = [0; (mem::size_of::<$typ>())];
        $this.copy_to_slice_unchecked(&mut buf[mem::size_of::<$typ>() - ($len_to_read)..]);
        return $typ::from_be_bytes(buf);
    }};
}

pub trait BufUnchecked: bytes::Buf {
    // fn advance_unchecked(&mut self, cnt: usize) {
    //     unsafe {
    //         self.inc_start(cnt);
    //     }
    // }

    fn copy_to_slice_unchecked(&mut self, dst: &mut [u8]) {
        let mut off = 0;

        while off < dst.len() {
            let cnt;

            unsafe {
                let src = self.chunk();
                cnt = cmp::min(src.len(), dst.len() - off);

                ptr::copy_nonoverlapping(src.as_ptr(), dst[off..].as_mut_ptr(), cnt);

                off += cnt;
            }

            self.advance(cnt);
        }
    }

    fn get_u8_unchecked(&mut self) -> u8 {
        let ret = self.chunk()[0];
        self.advance(1);
        ret
    }

    fn get_i32_unchecked(&mut self) -> i32 {
        buf_get_impl!(self, i32::from_be_bytes);
    }

    fn get_u32_unchecked(&mut self) -> u32 {
        buf_get_impl!(self, u32::from_be_bytes);
    }

    fn get_f32_unchecked(&mut self) -> f32 {
        f32::from_bits(Self::get_u32_unchecked(self))
    }
}
