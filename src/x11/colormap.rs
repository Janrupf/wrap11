use crate::{xlib_sys, XDisplay};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum ColormapAllocation {
    /// No entries are allocated initially
    None = xlib_sys::AllocNone,

    /// All entries are allocated initially
    All = xlib_sys::AllocAll,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum ColormapState {
    /// The colormap is installed on the X server
    Installed = xlib_sys::ColormapInstalled,

    /// The colormap is not installed on the X server
    Uninstalled = xlib_sys::ColormapUninstalled,
}

impl ColormapState {
    /// Wraps an existing X11 colormap state.
    ///
    /// # Arguments
    ///
    /// * `state` - The native X11 state to wrap
    pub fn new(state: i32) -> Self {
        match state {
            xlib_sys::ColormapInstalled => Self::Installed,
            xlib_sys::ColormapUninstalled => Self::Uninstalled,
            x => unreachable!("Invalid colormap state: {}", x),
        }
    }
}

/// Describes how a colormap handle is owned
#[derive(Debug)]
pub enum ColormapHandleOwnership {
    /// The colormap handle is not owned at all
    Foreign,

    /// The colormap is our own handle
    Owned,
}

#[derive(Debug)]
pub struct XColormap<'a> {
    handle: xlib_sys::Colormap,
    display: &'a XDisplay,
    ownership: ColormapHandleOwnership,
}

impl<'a> XColormap<'a> {
    /// Wraps an existing X11 colormap handle.
    ///
    /// # Arguments
    ///
    /// * `handle` - The native X11 color to wrap
    /// * `display` - The X11 display the colormap belongs to
    /// * `ownership` - The ownership of the passed colormap handle
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure that all arguments are valid.
    pub unsafe fn new(
        handle: xlib_sys::Colormap,
        display: &'a XDisplay,
        ownership: ColormapHandleOwnership,
    ) -> Self {
        Self {
            handle,
            display,
            ownership,
        }
    }

    /// Retrieves the underlying native X11 colormap handle.
    pub fn handle(&self) -> xlib_sys::Colormap {
        self.handle
    }
}

impl<'a> Drop for XColormap<'a> {
    fn drop(&mut self) {
        if matches!(self.ownership, ColormapHandleOwnership::Owned) {
            unsafe { xlib_sys::XFreeColormap(self.display.handle(), self.handle) };
        }
    }
}
