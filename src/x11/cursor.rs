use crate::{xlib_sys, XDisplay};

#[derive(Debug)]
pub struct XCursor<'a> {
    handle: xlib_sys::Cursor,
    _display: &'a XDisplay,
}

impl<'a> XCursor<'a> {
    /// Wraps an existing X11 cursor.
    ///
    /// # Arguments
    ///
    /// * `handle` - The X11 cursor to wrap
    /// * `display` - The display the cursor belongs to
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure all arguments are valid.
    pub unsafe fn new(handle: xlib_sys::Cursor, display: &'a XDisplay) -> Self {
        Self {
            handle,
            _display: display,
        }
    }

    /// Retrieves the underlying native X11 cursor handle.
    pub fn handle(&self) -> xlib_sys::Cursor {
        self.handle
    }
}
