use crate::{xlib_sys, XFont, XImage};
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

    /// Copies an image onto the target.
    ///
    /// # Arguments
    ///
    /// * `image` - The image to copy
    /// * `src_x` - The x offset in the image to start copying from
    /// * `src_y` - The y offset in the image to start copying from
    /// * `dest_x` - The x offset in the drawable to start copying to
    /// * `dest_y` - The y offset in the drawable to start copying to
    /// * `width` - The width of the image to copy
    /// * `height` - The height of the image to copy
    #[allow(clippy::too_many_arguments)]
    pub fn put_image(
        &self,
        image: &XImage,
        src_x: i32,
        src_y: i32,
        dest_x: i32,
        dest_y: i32,
        width: u32,
        height: u32,
    ) {
        unsafe {
            xlib_sys::XPutImage(
                self.display.handle(),
                self.drawable.drawable_handle(),
                self.handle,
                image.handle(),
                src_x,
                src_y,
                dest_x,
                dest_y,
                width,
                height,
            )
        };
    }

    /// Copies another drawable onto the target.
    ///
    /// # Arguments
    ///
    /// * `src` - The drawable to copy
    /// * `src_x` - The x offset in the drawable to start copying from
    /// * `src_y` - The y offset in the drawable to start copying from
    /// * `dest_x` - The x offset in the drawable to start copying to
    /// * `dest_y` - The y offset in the drawable to start copying to
    /// * `width` - The width of the drawable to copy
    /// * `height` - The height of the drawable to copy
    #[allow(clippy::too_many_arguments)]
    pub fn copy_area<'b, D: XDrawable<'b>>(
        &self,
        src: &D,
        src_x: i32,
        src_y: i32,
        dest_x: i32,
        dest_y: i32,
        width: u32,
        height: u32,
    ) {
        unsafe {
            xlib_sys::XCopyArea(
                self.display.handle(),
                src.drawable_handle(),
                self.drawable.drawable_handle(),
                self.handle,
                src_x,
                src_y,
                width as _,
                height as _,
                dest_x as _,
                dest_y as _,
            )
        };
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
