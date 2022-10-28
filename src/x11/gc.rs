use crate::{xlib_sys, XFont};
use crate::{XDisplay, XDrawable};

/// A graphics context bound to a drawable.
#[derive(Debug)]
pub struct XGC<'a, T>
where
    T: XDrawable<'a>,
{
    handle: xlib_sys::GC,
    drawable: &'a T,
    display: &'a XDisplay,
}

impl<'a, T> XGC<'a, T>
where
    T: XDrawable<'a>,
{
    /// Wraps an existing native graphics context.
    ///
    /// # Arguments
    ///
    /// * `handle` - The underlying native X11 graphics context
    /// * `drawable` - The drawable this graphics context is bound to
    /// * `display` - The display this graphics context resides on
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure that all arguments are valid.
    pub unsafe fn new(handle: xlib_sys::GC, drawable: &'a T, display: &'a XDisplay) -> Self {
        Self {
            handle,
            drawable,
            display,
        }
    }

    /// Sets the foreground color of the graphics context.
    ///
    /// # Arguments
    ///
    /// * `foreground` - The foreground color in ARGB format
    pub fn set_foreground(&self, foreground: u64) {
        unsafe { xlib_sys::XSetForeground(self.display.handle(), self.handle, foreground) };
    }

    /// Sets the background color of the graphics context.
    ///
    /// # Arguments
    ///
    /// * `background` - The background color in ARGB format
    pub fn set_background(&self, background: u64) {
        unsafe { xlib_sys::XSetBackground(self.display.handle(), self.handle, background) };
    }

    /// Sets the font of the graphics context.
    ///
    /// # Arguments
    ///
    /// * `font` - The font to use
    pub fn set_font(&self, font: &XFont<'a>) {
        unsafe { xlib_sys::XSetFont(self.display.handle(), self.handle, font.id().0) };
    }

    /// Fills a rectangle.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate to start drawing at
    /// * `y` - The y coordinate to start drawing at
    /// * `width` - The width to draw starting from `x`
    /// * `height` - The height to draw starting from `y`
    pub fn fill_rect(&self, x: i32, y: i32, width: u32, height: u32) {
        unsafe {
            xlib_sys::XFillRectangle(
                self.display.handle(),
                self.drawable.drawable_handle(),
                self.handle,
                x,
                y,
                width,
                height,
            );
        }
    }

    /// Draws a string.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate to start drawing at
    /// * `y` - The y coordinate to start drawing at
    /// * `s` - The string to draw
    pub fn draw_string(&self, x: i32, y: i32, s: impl AsRef<str>) {
        let text_bytes = s.as_ref().as_bytes();

        unsafe {
            xlib_sys::XDrawString(
                self.display.handle(),
                self.drawable.drawable_handle(),
                self.handle,
                x,
                y,
                text_bytes.as_ptr() as _,
                text_bytes.len() as _,
            );
        }
    }

    /// Retrieves the underlying native X11 graphics context.
    pub fn handle(&self) -> xlib_sys::GC {
        self.handle
    }
}

impl<'a, T> Drop for XGC<'a, T>
where
    T: XDrawable<'a>,
{
    fn drop(&mut self) {
        unsafe { xlib_sys::XFreeGC(self.display.handle(), self.handle) };
    }
}
