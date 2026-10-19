use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;

/// An in-memory sink for `print` output.
///
/// Clones share one buffer, so a caller can hand a clone to
/// [`Evaluator::with_writer`](super::Evaluator::with_writer) and still read
/// back everything the program printed.
#[derive(Debug, Clone, Default)]
pub struct SharedBuffer(Rc<RefCell<Vec<u8>>>);

impl SharedBuffer {
    pub fn new() -> Self {
        SharedBuffer::default()
    }

    /// Everything written so far, as text.
    pub fn contents(&self) -> String {
        String::from_utf8_lossy(&self.0.borrow()).into_owned()
    }
}

impl Write for SharedBuffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.borrow_mut().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
