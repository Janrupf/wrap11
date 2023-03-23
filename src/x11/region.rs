use crate::xfixes_sys;
use crate::XDisplay;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct XRectangle {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug)]
pub struct XServerRegion<'a> {
    handle: xfixes_sys::XserverRegion,
    display: &'a XDisplay,
}

impl<'a> XServerRegion<'a> {
    /// Wraps an existing native X11 XserverRegion.
    ///
    /// # Arguments
    ///
    /// * `handle` - The underlying native pointer
    /// * `display` - The display the region belongs to
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure all arguments are valid.
    pub unsafe fn new(handle: xfixes_sys::XserverRegion, display: &'a XDisplay) -> Self {
        Self { handle, display }
    }

    /// Retrieves the underlying native X11 XserverRegion handle.
    pub fn handle(&self) -> xfixes_sys::XserverRegion {
        self.handle
    }
}

impl<'a> Drop for XServerRegion<'a> {
    fn drop(&mut self) {
        unsafe { xfixes_sys::XFixesDestroyRegion(self.display.handle(), self.handle) };
    }
}
