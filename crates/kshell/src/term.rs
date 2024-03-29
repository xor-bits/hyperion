use alloc::boxed::Box;
use core::fmt;

use hyperion_color::Color;
use hyperion_escape::decode::{DecodedPart, EscapeDecoder};
use hyperion_framebuffer::{font::FONT, framebuffer::Framebuffer};

use super::CHAR_SIZE;

//

pub struct Term {
    escapes: EscapeDecoder,
    pub stdout_cursor: (usize, usize),
    pub cursor: (usize, usize),
    pub size: (usize, usize),
    buf: Box<[u8]>,
    old_buf: Box<[u8]>,
}

//

impl Term {
    pub fn new() -> Self {
        let Some(vbo) = Framebuffer::get() else {
            // TODO: serial only
            panic!("cannot run kshell without a framebuffer");
        };
        let mut vbo = vbo.lock();
        vbo.clear();

        let vbo_info = vbo.info();

        let cursor = (0, 0);

        let size = (
            vbo_info.width / CHAR_SIZE.0 as usize,
            vbo_info.height / CHAR_SIZE.1 as usize,
        );

        let buf = (0..size.0 * size.1).map(|_| b' ').collect();
        let old_buf = (0..size.0 * size.1).map(|_| b'=').collect();

        Self {
            escapes: EscapeDecoder::new(),
            stdout_cursor: cursor,
            cursor,
            size,
            buf,
            old_buf,
        }
    }

    pub fn flush(&mut self) {
        // framebuffer is already proven to be Some
        let Some(mut vbo) = Framebuffer::get().unwrap().try_lock() else {
            return;
        };

        // let mut updates = 0u32;
        for ((idx, ch), _) in self
            .buf
            .iter()
            .enumerate()
            .zip(self.old_buf.iter())
            .filter(|((_, b1), b0)| **b1 != **b0)
        {
            let x = (idx % self.size.0) * CHAR_SIZE.0 as usize;
            let y = (idx / self.size.0) * CHAR_SIZE.1 as usize;

            // updates += 1;
            if FONT[*ch as usize].1 {
                continue;
            }
            vbo.ascii_char(x, y, *ch, Color::WHITE, Color::BLACK);
        }
        // debug!("updates: {updates}");
        self.old_buf.copy_from_slice(&self.buf);
    }

    /* pub fn cursor_next(&mut self) {
        self.cursor.0 += 1;

        if self.cursor.0 >= self.size.0 {
            self.cursor.0 = 0;
            self.cursor.1 += 1;
        }
    } */

    pub fn cursor_prev(&mut self) {
        if self.cursor.0 == 0 {
            if self.cursor.1 == 0 {
                return;
            }
            self.cursor.0 = self.size.0 - 1;
            self.cursor.1 -= 1;
        }

        self.cursor.0 -= 1;
    }

    pub fn write_bytes(&mut self, b: &[u8]) {
        for b in b {
            self.write_byte(*b);
        }
    }

    pub fn clear(&mut self) {
        self.cursor = (0, 0);
        self.buf.fill(b' ');
        self.flush();
    }

    pub fn clear_line(&mut self) {
        self.cursor.0 = 0;
        self.buf[self.cursor.1 * self.size.0..(self.cursor.1 + 1) * self.size.0].fill(b' ');
        self.flush();
    }

    pub fn read_at(&self, cursor: (usize, usize)) -> u8 {
        self.buf[cursor.0 + cursor.1 * self.size.0]
    }

    pub fn write_byte(&mut self, b: u8) {
        match self.escapes.next(b) {
            DecodedPart::Byte(b) => self.write_raw_byte(b),
            DecodedPart::Bytes(b) => {
                for b in b.into_iter().take_while(|b| *b != 0) {
                    self.write_raw_byte(b)
                }
            }
            DecodedPart::FgColor(_) => {}
            DecodedPart::BgColor(_) => {}
            DecodedPart::Reset => {}
            DecodedPart::CursorUp(n) => self.cursor.1 = self.cursor.1.saturating_sub(n as usize),
            DecodedPart::CursorDown(n) => {
                self.cursor.1 += n as usize;
                self.cursor.1 = self.cursor.1.min(self.size.1);
            }
            DecodedPart::CursorLeft(n) => self.cursor.0 = self.cursor.0.saturating_sub(n as usize),
            DecodedPart::CursorRight(n) => {
                self.cursor.0 += n as usize;
                self.cursor.0 = self.cursor.0.min(self.size.0);
            }
            DecodedPart::None => {}
        }
    }

    pub fn write_raw_byte(&mut self, b: u8) {
        if self.cursor.0 >= self.size.0 {
            self.cursor.0 = 0;
            self.cursor.1 += 1;
        }
        if self.cursor.1 >= self.size.1 {
            let len = self.buf.len();
            self.stdout_cursor.1 = self.stdout_cursor.1.saturating_sub(1);
            self.cursor.1 -= 1;
            self.buf.copy_within(self.size.0.., 0);
            self.buf[len - self.size.0..].fill(b' ');
        }

        // crate::debug!("{b}");
        match b {
            b'\n' => {
                self.cursor.0 = 0;
                self.cursor.1 += 1;
            }
            other => {
                self.buf[self.cursor.0 + self.cursor.1 * self.size.0] = other;
                self.cursor.0 += 1;
            }
        }
    }
}

impl Default for Term {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Write for Term {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_bytes(s.as_bytes());
        Ok(())
    }
}
