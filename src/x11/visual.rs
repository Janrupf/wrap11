use crate::xlib_sys;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

/// X11 visual.
pub struct XVisual<'a> {
    handle: *mut xlib_sys::Visual,
    _data: PhantomData<&'a ()>,
}

impl<'a> XVisual<'a> {
    /// Wraps an existing native X11 visual.
    ///
    /// # Arguments
    ///
    /// * `handle` - The underlying native pointer
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure all arguments are valid.
    pub unsafe fn new(handle: *mut xlib_sys::Visual) -> Self {
        Self {
            handle,
            _data: PhantomData,
        }
    }

    /// Retrieves the underlying native X11 visual handle.
    pub fn handle(&self) -> *mut xlib_sys::Visual {
        self.handle
    }

    /// Retrieves the X11 id of the visual.
    pub fn id(&self) -> xlib_sys::VisualID {
        unsafe { xlib_sys::XVisualIDFromVisual(self.handle) }
    }
}

impl<'a> Debug for XVisual<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Note that this check is just to aid debugging - the safety contract of this type
        // guarantees that the handle is never null!
        let handle = if self.handle.is_null() {
            None
        } else {
            Some(unsafe { &*self.handle })
        };

        f.debug_struct("XVisual")
            .field("handle", &handle)
            .field("_data", &self._data)
            .finish()
    }
}

/// X11 information about a visual.
pub struct XVisualInfo<'a> {
    handle: xlib_sys::XVisualInfo,
    visual: XVisual<'a>,
    _data: PhantomData<&'a ()>,
}

impl<'a> XVisualInfo<'a> {
    /// Wraps an existing native X11 visual pointer and its belonging visual.
    ///
    /// # Arguments
    ///
    /// * `handle` - The native X11 visual pointer to wrap
    /// * `visual` - The visual the visual info belongs to
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure all arguments are valid.
    pub unsafe fn new(handle: xlib_sys::XVisualInfo, visual: XVisual<'a>) -> Self {
        Self {
            handle,
            visual,
            _data: PhantomData,
        }
    }

    /// Retrieves the bit-depth of the visual.
    pub fn depth(&self) -> i32 {
        self.handle.depth
    }

    /// Retrieves the visual this information belongs to.
    pub fn visual(&self) -> &XVisual {
        &self.visual
    }

    /// Retrieves the native underlying X11 pointer.
    pub fn handle(&self) -> xlib_sys::XVisualInfo {
        self.handle
    }

    /// Extracts the visual from this info, consuming the info.
    pub fn into_visual(self) -> XVisual<'a> {
        self.visual
    }
}

impl<'a> Debug for XVisualInfo<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("XVisualInfo")
            .field("handle", &self.handle)
            .field("_data", &self._data)
            .finish()
    }
}

impl<'a> PartialEq<Self> for XVisualInfo<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl<'a> Eq for XVisualInfo<'a> {}
