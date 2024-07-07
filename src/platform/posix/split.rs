//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

use std::io::{self, Read, Write};
use std::os::unix::io::{AsRawFd, RawFd};
use std::sync::Arc;

use crate::platform::posix::Fd;

/// Read-only end for a file descriptor.
#[derive(Clone)]
pub struct Reader(pub(crate) Arc<Fd>);

/// Write-only end for a file descriptor.
#[derive(Clone)]
pub struct Writer(pub(crate) Arc<Fd>);

impl Read for Reader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let ptr = Arc::as_ptr(&self.0).cast_mut();
        unsafe { Fd::read(ptr.as_mut().unwrap(), buf) }
    }

    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        let ptr = Arc::as_ptr(&self.0).cast_mut();
        unsafe { Fd::read_vectored(ptr.as_mut().unwrap(), bufs) }
    }
}

impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let ptr = Arc::as_ptr(&self.0).cast_mut();
        unsafe { Fd::write(ptr.as_mut().unwrap(), buf) }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        let ptr = Arc::as_ptr(&self.0).cast_mut();
        unsafe { Fd::write_vectored(ptr.as_mut().unwrap(), bufs) }
    }
}

impl AsRawFd for Reader {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl AsRawFd for Writer {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}
