use core::{
    fmt,
    ops::{Deref, DerefMut},
};

use hyperion_boot_interface::FramebufferCreateInfo;
use hyperion_color::Color;
use spin::{Mutex, Once};

use super::font::FONT;

//

pub struct Framebuffer {
    buf: &'static mut [u8],
    pub info: FramebufferInfo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FramebufferInfo {
    pub width: usize, // not the pixels to the next row
    pub height: usize,
    pub pitch: usize, // pixels to the next row
}

//

impl Framebuffer {
    pub fn new(buf: &'static mut [u8], info: FramebufferInfo) -> Self {
        Self { buf, info }
    }

    pub fn get() -> Option<&'static Mutex<Framebuffer>> {
        static FBO: Once<Option<Mutex<Framebuffer>>> = Once::new();

        FBO.call_once(|| {
            let mut fb @ FramebufferCreateInfo {
                width,
                height,
                pitch,
                ..
            } = hyperion_boot::framebuffer()?;
            let mut fbo = Framebuffer::new(
                unsafe { fb.buf_mut() },
                FramebufferInfo {
                    width,
                    height,
                    pitch,
                },
            );

            fbo.clear();
            Some(Mutex::new(fbo))
        })
        .as_ref()
    }

    pub fn buf(&self) -> &[u8] {
        self.buf
    }

    pub fn buf_mut(&mut self) -> &mut [u8] {
        self.buf
    }

    pub fn pixel(&mut self, x: usize, y: usize, color: Color) {
        let spot = x * 4 + y * self.pitch;
        self.buf[spot..spot + 4].copy_from_slice(&color.as_arr()[..]);
    }

    pub fn fill(&mut self, x: usize, y: usize, w: usize, h: usize, color: Color) {
        for yd in y..y + h {
            let spot = x * 4 + yd * self.pitch;
            self.buf[spot..spot + 4 * w]
                .as_chunks_mut::<4>()
                .0
                .fill(color.as_arr());
        }
    }

    pub fn ascii_char(&mut self, x: usize, y: usize, ch: u8, fg: Color, bg: Color) -> bool {
        let (map, double_wide) = FONT[ch as usize];

        let w = if double_wide { 16 } else { 8 };

        for (yd, row) in map.into_iter().enumerate() {
            for xd in 0..w {
                let px_col = if (row & 1 << xd) != 0 { fg } else { bg };
                self.pixel(x + xd, y + yd, px_col);
            }
        }

        double_wide
    }

    pub fn clear(&mut self) {
        self.buf.fill(0);
    }

    pub fn info(&self) -> FramebufferInfo {
        self.info
    }
}

impl Deref for Framebuffer {
    type Target = FramebufferInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

impl DerefMut for Framebuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.info
    }
}

impl fmt::Debug for Framebuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Framebuffer")
            .field("info", &self.info)
            .finish()
    }
}

//

#[cfg(test)]
mod tests {
    use hyperion_color::Color;

    use super::Framebuffer;

    //

    #[test]
    fn fbo_draw() {
        if let Some(fbo) = Framebuffer::get() {
            let mut fbo = fbo.lock();
            fbo.fill(440, 340, 40, 40, Color::RED);
            fbo.fill(450, 350, 60, 40, Color::GREEN);
            fbo.fill(405, 315, 80, 20, Color::BLUE);
        }
    }
}
