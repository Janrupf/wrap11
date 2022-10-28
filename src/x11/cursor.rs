use crate::{xfixes_sys, xlib_sys, XDisplay};

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

#[derive(Debug)]
pub struct XCursorImage {
    handle: *mut xfixes_sys::XFixesCursorImage,
}

impl XCursorImage {
    /// Wraps an existing X11 cursor image.
    ///
    /// # Arguments
    ///
    /// * `handle` - The X11 cursor image to wrap
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure all arguments are valid.
    pub unsafe fn new(handle: *mut xfixes_sys::XFixesCursorImage) -> Self {
        Self { handle }
    }

    /// Retrieves the height of the image.
    pub fn height(&self) -> u16 {
        unsafe { &*self.handle }.height
    }

    /// Retrieves the width of the image.
    pub fn width(&self) -> u16 {
        unsafe { &*self.handle }.width
    }

    /// Retrieves the hotspot x of the image.
    pub fn hotspot_x(&self) -> u16 {
        unsafe { &*self.handle }.xhot
    }

    /// Retrieves the hotspot y of the image.
    pub fn hotspot_y(&self) -> u16 {
        unsafe { &*self.handle }.yhot
    }

    /// Retrieves the cursor image as RGBA.
    pub fn rgba_data(&self) -> Vec<u8> {
        let size = (self.width() * self.height()) as usize;

        let mut buffer = Vec::with_capacity(size * 4);

        unsafe {
            for i in 0..size {
                let pixel = *(*self.handle).pixels.add(i);
                let pixel = (pixel & 0xFFFFFFFF) as u32;

                buffer.push(((pixel & 0x00FF0000) >> 16) as _);
                buffer.push(((pixel & 0x0000FF00) >> 8) as _);
                buffer.push((pixel & 0x000000FF) as _);
                buffer.push(((pixel & 0xFF000000) >> 24) as _);
            }
        }

        buffer
    }
}

impl Drop for XCursorImage {
    fn drop(&mut self) {
        unsafe { xlib_sys::XFree(self.handle as _) };
    }
}
