use crate::{xlib_sys, XDisplay};
use std::mem::MaybeUninit;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct XFontId(pub(crate) xlib_sys::Font);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[repr(i32)]
pub enum XFontDirection {
    LeftToRight = xlib_sys::FontLeftToRight,
    RightToLeft = xlib_sys::FontRightToLeft,
}

impl XFontDirection {
    /// Wraps a native X11 front direction.
    ///
    /// # Arguments
    ///
    /// * `raw` - The X11 font direction to wrap
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure all arguments are valid.
    pub unsafe fn from_raw(raw: i32) -> Self {
        std::mem::transmute(raw)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct XChar {
    handle: xlib_sys::XCharStruct,
}

impl XChar {
    /// Wraps an existing X11 char.
    ///
    /// # Arguments
    ///
    /// * `handle` - The X11 char to wrap
    pub fn new(handle: xlib_sys::XCharStruct) -> Self {
        Self { handle }
    }

    /// Retrieves the origin to the left edge of the raster.
    pub fn left_bearing(&self) -> i16 {
        self.handle.lbearing
    }

    /// Retrieves the origin to the right edge of the raster.
    pub fn right_bearing(&self) -> i16 {
        self.handle.rbearing
    }

    /// Retrieves the advance to the next char's origin.
    pub fn width(&self) -> i16 {
        self.handle.width
    }

    /// Retrieves the baseline to top edge of the raster.
    pub fn ascent(&self) -> i16 {
        self.handle.ascent
    }

    /// Retrieves the baseline to bottom edge of the raster.
    pub fn descent(&self) -> i16 {
        self.handle.descent
    }

    /// Retrieves the attributes of the char (not predefined).
    pub fn attributes(&self) -> u16 {
        self.handle.attributes
    }
}

impl Eq for XChar {}

/// Shape of a text.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct XTextExtents {
    direction: XFontDirection,
    font_ascent: i32,
    font_descent: i32,
    overall: XChar,
}

impl XTextExtents {
    /// Wraps existing X11 text extents.
    ///
    /// # Arguments
    ///
    /// * `direction` - The direction of the text
    /// * `font_ascent` - The ascent of the font
    /// * `font_descent` - The descent of the font
    /// * `overall` - The overall extent of the text
    fn new(direction: XFontDirection, font_ascent: i32, font_descent: i32, overall: XChar) -> Self {
        Self {
            direction,
            font_ascent,
            font_descent,
            overall,
        }
    }

    /// Retrieves the direction of the text.
    pub fn direction(&self) -> XFontDirection {
        self.direction
    }

    /// Retrieves the ascent of the font.
    pub fn font_ascent(&self) -> i32 {
        self.font_ascent
    }

    /// Retrieves the descent of the font.
    pub fn font_descent(&self) -> i32 {
        self.font_descent
    }

    /// Retrieves the overall extent of the text.
    pub fn overall(&self) -> &XChar {
        &self.overall
    }
}

#[derive(Debug)]
pub struct XFont<'a> {
    handle: *mut xlib_sys::XFontStruct,
    owned: bool,
    display: &'a XDisplay,
}

impl<'a> XFont<'a> {
    const DEFAULT_FONT_NAMES: &'static [&'static str] = &["9x15", "8x13", "fixed"];

    /// Attempts to find the default font for the display.
    ///
    /// # Arguments
    ///
    /// * `program` - The name of the program to use when looking up defaults
    /// * `display` - The display the font should belong to
    ///
    /// # Panics
    ///
    /// If `program` contains a nul byte.
    pub fn find_default(program: impl AsRef<str>, display: &'a XDisplay) -> Option<Self> {
        let program = program.as_ref();

        let default_font_name = display
            .get_default(program, "font")
            .or_else(|| display.get_default(program, "Font"));

        let font = default_font_name.and_then(|name| display.load_query_font(name));
        if font.is_some() {
            return font;
        }

        for font_name in Self::DEFAULT_FONT_NAMES {
            let font = display.load_query_font(font_name);
            if font.is_some() {
                return font;
            }
        }

        None
    }

    /// Wraps an existing X11 font.
    ///
    /// # Arguments
    ///
    /// * `handle` - The X11 font to wrap
    /// * `owned` - Whether the X11 font is owned and will be freed when this is dropped
    /// * `display` - The display the font belongs to
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure all arguments are valid.
    pub unsafe fn new(
        handle: *mut xlib_sys::XFontStruct,
        owned: bool,
        display: &'a XDisplay,
    ) -> Self {
        Self {
            handle,
            owned,
            display,
        }
    }

    /// Retrieves the underlying native X11 font handle.
    pub fn handle(&self) -> *mut xlib_sys::XFontStruct {
        self.handle
    }

    /// Retrieves the id of the native X11 font.
    pub fn id(&self) -> XFontId {
        XFontId(unsafe { &*self.handle }.fid)
    }

    /// Calculates the extents of the given text.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to calculate the extents for
    pub fn text_extents(&self, text: impl AsRef<str>) -> XTextExtents {
        let text_bytes = text.as_ref().as_bytes();

        let mut direction = 0;
        let mut font_ascent = 0;
        let mut font_descent = 0;
        let mut overall = MaybeUninit::uninit();

        unsafe {
            xlib_sys::XTextExtents(
                self.handle,
                text_bytes.as_ptr() as _,
                text_bytes.len() as _,
                &mut direction,
                &mut font_ascent,
                &mut font_descent,
                overall.as_mut_ptr(),
            );
        }

        let direction = unsafe { XFontDirection::from_raw(direction) };
        let overall = unsafe { XChar::new(overall.assume_init()) };

        XTextExtents::new(direction, font_ascent, font_descent, overall)
    }
}

impl<'a> Drop for XFont<'a> {
    fn drop(&mut self) {
        if self.owned {
            unsafe { xlib_sys::XFreeFont(self.display.handle(), self.handle) };
        }
    }
}
