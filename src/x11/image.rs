use crate::XDisplay;

use crate::xlib_sys;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum XBitmapPadding {
    Bit8 = 8,
    Bit16 = 16,
    Bit32 = 32,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum XImageFormat {
    /// Bitmap organized in planes
    XYBitmap = xlib_sys::XYBitmap,

    /// RGB image organized in planes
    XYPixmap = xlib_sys::XYPixmap,

    /// RGB image organized in pixels
    ZPixmap = xlib_sys::ZPixmap,
}

/// X11 image.
///
/// An X11 image is a client side image buffer which can be uploaded to the server.
pub struct XImage<'a> {
    handle: *mut xlib_sys::XImage,
    _display: &'a XDisplay,
}

impl<'a> XImage<'a> {
    /// Wraps an existing X11 image.
    ///
    /// # Arguments
    ///
    /// * `handle` - The X11 image to wrap
    /// * `display` - The display the image belongs to
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure all arguments are valid.
    pub unsafe fn new(handle: *mut xlib_sys::XImage, display: &'a XDisplay) -> Self {
        Self {
            handle,
            _display: display,
        }
    }

    /// Retrieves the underlying native X11 image handle.
    pub fn handle(&self) -> *mut xlib_sys::XImage {
        self.handle
    }
}

impl<'a> Drop for XImage<'a> {
    fn drop(&mut self) {
        unsafe { xlib_sys::XDestroyImage(self.handle) };
    }
}
