use super::constants::*;
use super::error::FrameError;

pub fn escape(input: &[u8], out: &mut [u8]) -> Result<usize, FrameError> {
    let mut idx = 0;

    for &b in input {
        let needs_escape = matches!(b, SOF | EOF | ESC);
        if needs_escape {
            if idx + 2 > out.len() {
                return Err(FrameError::BufferTooSmall);
            }
            out[idx] = ESC;
            out[idx + 1] = b ^ ESC_XOR;
            idx += 2;
        } else {
            if idx + 1 > out.len() {
                return Err(FrameError::BufferTooSmall);
            }
            out[idx] = b;
            idx += 1;
        }
    }

    Ok(idx)
}

pub fn unescape(input: &[u8], out: &mut [u8]) -> Result<usize, FrameError> {
    let mut i = 0;
    let mut o = 0;

    while i < input.len() {
        let b = input[i];
        if b == ESC {
            if i + 1 >= input.len() {
                return Err(FrameError::InvalidEscape);
            }
            out[o] = input[i + 1] ^ ESC_XOR;
            i += 2;
        } else {
            out[o] = b;
            i += 1;
        }
        o += 1;
    }

    Ok(o)
}

