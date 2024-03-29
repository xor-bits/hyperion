use alloc::{format, sync::Arc};
use core::fmt;

use hyperion_color::Color;

//

#[must_use]
pub fn color_to_code(Color { r, g, b }: Color) -> Arc<str> {
    format!("\x1B[38;2;{r};{g};{b}m").into()
}

//

pub struct CursorUp(pub u8);

impl fmt::Display for CursorUp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[{}A", self.0)
    }
}

//

pub struct CursorDown(pub u8);

impl fmt::Display for CursorDown {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[{}B", self.0)
    }
}

//

pub struct CursorLeft(pub u8);

impl fmt::Display for CursorLeft {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[{}C", self.0)
    }
}

//

pub struct CursorRight(pub u8);

impl fmt::Display for CursorRight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[{}D", self.0)
    }
}

//

pub trait EscapeEncoder
where
    Self: Sized,
{
    fn with_escape_code(self, code: &str) -> EncodedPart<Self> {
        EncodedPart {
            code,
            data: self,
            reset: true,
        }
    }

    fn true_red(self) -> EncodedPart<'static, Self> {
        self.with_escape_code("\x1B[38;2;255;0;0m")
    }

    fn true_green(self) -> EncodedPart<'static, Self> {
        self.with_escape_code("\x1B[38;2;0;255;0m")
    }

    fn true_blue(self) -> EncodedPart<'static, Self> {
        self.with_escape_code("\x1B[38;2;0;0;255m")
    }

    fn true_cyan(self) -> EncodedPart<'static, Self> {
        self.with_escape_code("\x1B[38;2;0;255;255m")
    }

    fn true_magenta(self) -> EncodedPart<'static, Self> {
        self.with_escape_code("\x1B[38;2;255;0;255m")
    }

    fn true_yellow(self) -> EncodedPart<'static, Self> {
        self.with_escape_code("\x1B[38;2;255;255;0m")
    }

    fn true_black(self) -> EncodedPart<'static, Self> {
        self.with_escape_code("\x1B[38;2;0;0;0m")
    }

    fn true_darkgrey(self) -> EncodedPart<'static, Self> {
        self.with_escape_code("\x1B[38;2;64;64;64m")
    }

    fn true_grey(self) -> EncodedPart<'static, Self> {
        self.with_escape_code("\x1B[38;2;128;128;128m")
    }

    fn true_lightgrey(self) -> EncodedPart<'static, Self> {
        self.with_escape_code("\x1B[38;2;192;192;192m")
    }

    fn true_white(self) -> EncodedPart<'static, Self> {
        self.with_escape_code("\x1B[38;2;255;255;255m")
    }

    fn reset_after(self) -> EncodedPart<'static, Self> {
        self.with_escape_code("")
    }
}

#[derive(Clone, Copy)]
pub struct EncodedPart<'a, T: Sized> {
    code: &'a str,
    reset: bool,
    data: T,
}

//

// impl EscapeEncoder for &str {}
//
// impl EscapeEncoder for char {}

impl<T> EscapeEncoder for T {}

impl<T> EncodedPart<'_, T> {
    pub fn with_reset(mut self, reset: bool) -> Self {
        self.reset = reset;
        self
    }

    fn write_end(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.reset {
            write!(f, "\x1B[m")?;
        }
        Ok(())
    }
}

impl Default for EncodedPart<'static, ()> {
    fn default() -> Self {
        Self {
            code: "",
            data: (),
            reset: true,
        }
    }
}

impl<T> From<T> for EncodedPart<'_, T> {
    fn from(value: T) -> Self {
        Self {
            code: "",
            data: value,
            reset: false,
        }
    }
}

impl<T> fmt::Display for EncodedPart<'_, T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.code, self.data)?;
        self.write_end(f)
    }
}

impl<T> fmt::Debug for EncodedPart<'_, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.code)?;
        self.data.fmt(f)?;
        self.write_end(f)
    }
}

impl<T> fmt::LowerHex for EncodedPart<'_, T>
where
    T: fmt::LowerHex,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.code)?;
        self.data.fmt(f)?;
        self.write_end(f)
    }
}
