use core::any::Any;

use hyperion_vfs::{
    device::FileDevice,
    error::{IoError, IoResult},
};

//

pub struct Null;

impl FileDevice for Null {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn len(&self) -> usize {
        1
    }

    fn set_len(&mut self, _: usize) -> IoResult<()> {
        Err(IoError::PermissionDenied)
    }

    fn read(&self, _: usize, _: &mut [u8]) -> IoResult<usize> {
        Ok(0)
    }

    fn write(&mut self, _: usize, buf: &[u8]) -> IoResult<usize> {
        Ok(buf.len())
    }
}
