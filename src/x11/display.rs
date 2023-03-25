use crate::{
    xfixes_sys, xlib_sys, xtest_sys, XBitmapPadding, XCursorImage, XEvent, XFont, XImage,
    XImageFormat, XVisual, XWindow,
};
use crate::{XAtom, XLibError, XScreen};
use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use std::num::NonZeroUsize;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum QueuedMode {
    Already = 0,
    AfterReading = 1,
    AfterFlush = 2,
}

/// Describes how a display handle is owned.
#[derive(Debug, Eq, PartialEq)]
pub enum DisplayOwnership {
    /// The display handle is our own handle
    Owned,

    /// The display handle is owned by something else
    Foreign,
}

/// The heart of an X11 connection.
///
/// In the context of XLib this represents a connection to the X11 server.
#[derive(Debug)]
pub struct XDisplay {
    ownership: DisplayOwnership,
    handle: *mut xlib_sys::Display,
    xfixes_event_base: i32,
    xinput2_opcode: i32,
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

        Ok(unsafe { Self::from_ptr(handle, DisplayOwnership::Owned) })
    }

    /// Constructs an X11 display wrapper from an existing pointer.
    ///
    /// # Arguments
    ///
    /// * `handle` - The pointer to the X11 display to wrap
    ///
    /// # Safety
    ///
    /// The caller must ensure that the handle is a valid pointer and
    /// is not operated on after this method has been called.
    pub unsafe fn from_ptr(handle: *mut xlib_sys::Display, ownership: DisplayOwnership) -> Self {
        let mut xfixes_event_base = 0;
        let mut xfixes_error_base = 0;

        unsafe {
            xfixes_sys::XFixesQueryExtension(handle, &mut xfixes_event_base, &mut xfixes_error_base)
        };

        let mut xinput2_opcode = 0;
        let mut xinput2_event_base = 0;
        let mut xinput2_error_base = 0;

        unsafe {
            let name = CString::new("XInputExtension").unwrap();

            xlib_sys::XQueryExtension(
                handle,
                name.as_ptr(),
                &mut xinput2_opcode,
                &mut xinput2_event_base,
                &mut xinput2_error_base,
            );
        }

        XDisplay {
            ownership,
            handle,
            xfixes_event_base,
            xinput2_opcode,
        }
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

    /// Creates a new image.
    ///
    /// # Arguments
    ///
    /// * `visual` - The visual to use backing the image
    /// * `depth` - The depth of the image
    /// * `format` - The format of the image
    /// * `offset` - The offset in the data before the image starts
    /// * `data` - The image data
    /// * `width` - The width of the image
    /// * `height` - The height of the image
    /// * `bitmap_pad` - Bitmap padding describing the pixel padding
    /// * `bytes_per_line` - Explicit setting of bytes per line, auto calculated if [`None`]
    #[allow(clippy::too_many_arguments)]
    pub fn create_image<'a>(
        &'a self,
        visual: &XVisual,
        depth: u32,
        format: XImageFormat,
        offset: u32,
        data: &[u8],
        width: u32,
        height: u32,
        bitmap_pad: XBitmapPadding,
        bytes_per_line: Option<NonZeroUsize>,
    ) -> XImage<'a> {
        if let Some(bytes_per_line) = bytes_per_line {
            let expected_bytes = (height * bytes_per_line.get() as u32) + offset;
            assert_eq!(expected_bytes, data.len() as u32);
        } else {
            let bits = bitmap_pad as u32;

            let expected_bytes = (height * width * (bits / 8)) + offset;
            assert_eq!(expected_bytes, data.len() as u32);
        }

        let image = unsafe {
            let data_malloced = libc::malloc(data.len() as _);
            std::ptr::copy_nonoverlapping::<u8>(data.as_ptr(), data_malloced as _, data.len() as _);

            xlib_sys::XCreateImage(
                self.handle,
                visual.handle(),
                depth,
                format as _,
                offset as _,
                data_malloced as _, // will be freed by X11
                width as _,
                height as _,
                bitmap_pad as _,
                bytes_per_line.map(|v| v.get()).unwrap_or(0) as _,
            )
        };

        unsafe { XImage::new(image, self) }
    }

    /// Retrieves the current cursor image.
    pub fn get_cursor_image(&self) -> XCursorImage {
        unsafe { XCursorImage::new(xfixes_sys::XFixesGetCursorImage(self.handle)) }
    }

    /// Moves the mouse pointer to a specific position.
    ///
    /// # Arguments
    ///
    /// * `source_window` - If given, the pointer is only moved if it is currently in this window
    /// * `destination_window` - The window to move the pointer relative to
    /// * `source_rect` - (x, y, width, height) rectangle, if given the pointer is only moved if it
    ///                   currently is in [`source_window`] and this rectangle within
    /// * `dest_x` - X coordinate to move the pointer to relative to [`destination_window`]
    /// * `dest_y` - Y coordinate to move the pointer relative to [`destination_window`]
    pub fn warp_pointer(
        &self,
        source_window: Option<&XWindow>,
        destination_window: &XWindow,
        source_rect: Option<(usize, usize, usize, usize)>,
        dest_x: usize,
        dest_y: usize,
    ) {
        if source_rect.is_some() {
            assert!(
                source_window.is_some(),
                "Can only specify a source rect if a source window is given"
            );
        }

        let source_window = source_window.map(|w| w.handle()).unwrap_or(0);
        let (src_x, src_y, src_width, src_height) = source_rect.unwrap_or((0, 0, 0, 0));

        unsafe {
            xlib_sys::XWarpPointer(
                self.handle,
                source_window,
                destination_window.handle(),
                src_x as _,
                src_y as _,
                src_width as _,
                src_height as _,
                dest_x as _,
                dest_y as _,
            )
        };
    }

    /// Sends a fake key event.
    ///
    /// # Arguments
    ///
    /// `keycode` - The keycode to fake
    /// `press` - Whether the key is being released or pressed
    /// `delay` - How many milliseconds to wait before sending the event
    pub fn fake_key_event(&self, keycode: u32, press: bool, delay: u64) {
        unsafe { xtest_sys::XTestFakeKeyEvent(self.handle, keycode, press as _, delay) };
    }

    /// Sends a fake button event.
    ///
    /// # Arguments
    ///
    /// `button` - The button to fake
    /// `press` - Whether the key is being released or pressed
    /// `delay` - How many milliseconds to wait before sending the event
    pub fn fake_button_event(&self, button: u32, press: bool, delay: u64) {
        unsafe { xtest_sys::XTestFakeButtonEvent(self.handle, button, press as _, delay) };
    }

    /// Sends a fake motion event.
    ///
    /// # Arguments
    ///
    /// `screen_number` - The number of the screen the event should occur on, or [`None`] for current
    /// `x` - The X position the motion moved to
    /// `y` - The Y position the motion moved to
    /// `delay` - How many milliseconds to wait before sending the event
    /// `relative` - Whether X and Y are relative to the current position or absolute
    pub fn fake_motion_event(
        &self,
        screen_number: Option<u32>,
        x: i32,
        y: i32,
        delay: u64,
        relative: bool,
    ) {
        let screen_number = screen_number.map(|v| v as i32).unwrap_or(-1);

        if relative {
            unsafe {
                xtest_sys::XTestFakeRelativeMotionEvent(self.handle, screen_number, x, y, delay)
            };
        } else {
            unsafe { xtest_sys::XTestFakeMotionEvent(self.handle, screen_number, x, y, delay) };
        }
    }

    /// Retrieves the minimum and maximum number of keycodes supported.
    pub fn keycodes(&self) -> (u8, u8) {
        let mut min_supported = 0;
        let mut max_supported = 0;

        unsafe { xlib_sys::XDisplayKeycodes(self.handle, &mut min_supported, &mut max_supported) };

        debug_assert!((8..=255).contains(&min_supported));
        debug_assert!(max_supported >= min_supported && max_supported <= 255);

        (min_supported as _, max_supported as _)
    }

    /// Retrieves the event base id for xfixes events.
    pub fn xfixes_event_base(&self) -> i32 {
        self.xfixes_event_base
    }

    /// Retrieves the opcode for the xinput2 extension.
    pub fn xinput2_opcode(&self) -> i32 {
        self.xinput2_opcode
    }
}

impl Drop for XDisplay {
    fn drop(&mut self) {
        if self.ownership == DisplayOwnership::Owned {
            unsafe { xlib_sys::XCloseDisplay(self.handle) };
        }
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
