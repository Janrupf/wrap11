use crate::{xlib_sys, XEvent, XFont};
use crate::{XAtom, XLibError, XScreen};
use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum QueuedMode {
    Already = 0,
    AfterReading = 1,
    AfterFlush = 2,
}

/// The heart of an X11 connection.
///
/// In the context of XLib this represents a connection to the X11 server.
#[derive(Debug)]
pub struct XDisplay {
    handle: *mut xlib_sys::Display,
}

impl XDisplay {
    /// Attempts to open the connection to the X11 server using the default display.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the display to open
    pub fn open(name: Option<&str>) -> Result<Self, XLibError> {
        let c_name = name.map(|name| CString::new(name).unwrap());

        let handle = unsafe {
            xlib_sys::XOpenDisplay(
                c_name
                    .map(|name| name.as_ptr())
                    .unwrap_or(std::ptr::null_mut()),
            )
        };
        if handle.is_null() {
            let attempted_name =
                Self::display_name(name).unwrap_or_else(|| String::from("<unknown>"));

            return Err(XLibError::OpenDisplayFailed(attempted_name));
        }

        Ok(XDisplay { handle })
    }

    /// Retrieves the name of the display that [`xlib_sys::XOpenDisplay`] would attempt to use.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the display to look up
    pub fn display_name(name: Option<&str>) -> Option<String> {
        let name = name.map(|name| CString::new(name).unwrap());

        let used_name = unsafe {
            xlib_sys::XDisplayName(
                name.map(|name| name.as_ptr())
                    .unwrap_or(std::ptr::null_mut()),
            )
        };

        if used_name.is_null() {
            None
        } else {
            let used_name = unsafe { CStr::from_ptr(used_name).to_string_lossy().into_owned() };
            Some(used_name)
        }
    }

    /// Retrieves the underlying X11 native platform pointer.
    pub fn handle(&self) -> *mut xlib_sys::Display {
        self.handle
    }

    /// Retrieves the connection number of this display.
    ///
    /// On POSIX systems this will be a file descriptor pointing to the socket.
    pub fn connection_number(&self) -> i32 {
        unsafe { xlib_sys::XConnectionNumber(self.handle) }
    }

    /// Retrieves the default screen of the X11 display.
    pub fn default_screen(&self) -> XScreen {
        unsafe { XScreen::new(xlib_sys::XDefaultScreenOfDisplay(self.handle), self) }
    }

    /// Retrieves the amounts of events received from the X server but not processed yet
    pub fn pending_events(&self) -> u32 {
        (unsafe { xlib_sys::XPending(self.handle) }) as u32
    }

    /// Retrieves the amount of events received from X server depending on the selected mode.
    ///
    /// # Arguments
    ///
    /// * `mode` - How to check the queue
    pub fn queued_events(&self, mode: QueuedMode) -> u32 {
        (unsafe { xlib_sys::XEventsQueued(self.handle, mode as _) }) as u32
    }

    /// Waits for the next event to arrive on the display.
    pub fn next_event(&self) -> XEvent {
        unsafe {
            let mut event = MaybeUninit::uninit();
            xlib_sys::XNextEvent(self.handle, event.as_mut_ptr());
            let event = event.assume_init();

            XEvent::new(event, self)
        }
    }

    /// Synchronizes the X11 command queue and flushes all commands.
    ///
    /// This function will call the error handlers for any outstanding errors.
    ///
    /// # Arguments
    ///
    /// * `discard` - If `true`, outstanding commands will be discarded instead of flushed
    pub fn sync(&self, discard: bool) {
        unsafe { xlib_sys::XSync(self.handle, discard.into()) };
    }

    /// Flushes all commands from the queue.
    ///
    /// Other than [`sync`] this function does not read incoming events.
    pub fn flush(&self) {
        unsafe { xlib_sys::XFlush(self.handle) };
    }

    /// Attempts to retrieve an existing X11 atom from the display.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the atom to retrieve
    ///
    /// # Panics
    ///
    /// If the name contains a nul character.
    pub fn get_atom(&self, name: impl AsRef<str>) -> Option<XAtom> {
        let name = CString::new(name.as_ref()).unwrap();
        let atom = unsafe { xlib_sys::XInternAtom(self.handle, name.as_ptr(), 1) };

        if atom == 0 {
            None
        } else {
            Some(unsafe { XAtom::new(atom, self) })
        }
    }

    /// Attempts to retrieve an X11 atom from the display, creating it if it doesn't exist yet.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the atom to retrieve or create
    ///
    /// # Panics
    ///
    /// If the name contains a nul character.
    pub fn get_or_create_atom(&self, name: impl AsRef<str>) -> XAtom {
        let name = CString::new(name.as_ref()).unwrap();
        let atom = unsafe { xlib_sys::XInternAtom(self.handle, name.as_ptr(), 0) };

        debug_assert!(atom != 0);
        unsafe { XAtom::new(atom, self) }
    }

    /// Attempts to load and query an X11 font.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the font to load
    ///
    /// # Panics
    ///
    /// If the name contains a nul character.
    pub fn load_query_font(&self, name: impl AsRef<str>) -> Option<XFont> {
        let name = CString::new(name.as_ref()).unwrap();

        let font = unsafe { xlib_sys::XLoadQueryFont(self.handle, name.as_ptr()) };
        if font.is_null() {
            None
        } else {
            Some(unsafe { XFont::new(font, true, self) })
        }
    }

    /// Attempts to find the default font for the display.
    ///
    /// # Arguments
    ///
    /// * `program` - The name of the program to use when looking up defaults
    ///
    /// # Panics
    ///
    /// If the program name contains a nul character.
    pub fn find_default_font(&self, program: impl AsRef<str>) -> Option<XFont> {
        XFont::find_default(program, self)
    }

    /// Retrieves a default from the display.
    ///
    /// # Arguments
    ///
    /// * `program` - The name of the program requesting the default
    /// * `name` - The name of the default to retrieve
    ///
    /// # Panics
    ///
    /// If the name or program contains a nul character or if the default is not UTF-8.
    pub fn get_default(
        &self,
        program: impl AsRef<str>,
        name: impl AsRef<str>,
    ) -> Option<&'static str> {
        let program = CString::new(program.as_ref()).unwrap();
        let name = CString::new(name.as_ref()).unwrap();

        let val = unsafe { xlib_sys::XGetDefault(self.handle, program.as_ptr(), name.as_ptr()) };

        if val.is_null() {
            None
        } else {
            let cstr = unsafe { CStr::from_ptr(val) };
            Some(cstr.to_str().unwrap())
        }
    }
}

impl Drop for XDisplay {
    fn drop(&mut self) {
        unsafe { xlib_sys::XCloseDisplay(self.handle) };
    }
}

#[cfg(feature = "connection-poll")]
mod io {
    use crate::XDisplay;
    use mio::event::Source;
    use mio::unix::SourceFd;
    use mio::{Interest, Registry, Token};
    use std::os::unix::io::{AsRawFd, RawFd};

    impl AsRawFd for XDisplay {
        fn as_raw_fd(&self) -> RawFd {
            self.connection_number() as _
        }
    }

    impl Source for XDisplay {
        fn register(
            &mut self,
            registry: &Registry,
            token: Token,
            interests: Interest,
        ) -> std::io::Result<()> {
            registry.register(&mut SourceFd(&self.as_raw_fd()), token, interests)
        }

        fn reregister(
            &mut self,
            registry: &Registry,
            token: Token,
            interests: Interest,
        ) -> std::io::Result<()> {
            registry.reregister(&mut SourceFd(&self.as_raw_fd()), token, interests)
        }

        fn deregister(&mut self, registry: &Registry) -> std::io::Result<()> {
            registry.deregister(&mut SourceFd(&self.as_raw_fd()))
        }
    }
}
