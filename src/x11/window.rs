use crate::{
    xcomposite_sys, xfixes_sys, xlib_sys, XAtom, XColormap, XCursor, XDisplay, XDrawable, XPixmap,
    XScreen, XServerRegion, XVisual,
};
use std::ffi::CString;

use std::fmt::Debug;
use std::mem::MaybeUninit;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum WindowShapeKind {
    Bounding = 0,
    Clip = 1,
    Input = 2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum WindowClass {
    InputOnly = xlib_sys::InputOnly,
    InputOutput = xlib_sys::InputOutput,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum BackingWindowStore {
    NotUseful = xlib_sys::NotUseful,
    WhenMapped = xlib_sys::WhenMapped,
    Always = xlib_sys::Always,
}

#[derive(Debug, Default, Clone)]
pub struct SetWindowAttributes<'creation, 'a> {
    background_pixmap: Option<&'creation XPixmap<'a>>,
    background_pixel: Option<u64>,
    border_pixmap: Option<&'creation XPixmap<'a>>,
    border_pixel: Option<u64>,
    bit_gravity: Option<i32>,
    win_gravity: Option<i32>,
    backing_store: Option<BackingWindowStore>,
    backing_planes: Option<u64>,
    backing_pixels: Option<u64>,
    save_under: Option<bool>,
    event_mask: Option<WindowInputMask>,
    do_not_propagate_mask: Option<WindowInputMask>,
    override_redirect: Option<bool>,
    colormap: Option<&'creation XColormap<'a>>,
    cursor: Option<&'creation XCursor<'a>>,
}

impl<'creation, 'a> SetWindowAttributes<'creation, 'a> {
    /// Creates a new set of window attributes without any set.
    pub fn new() -> Self {
        SetWindowAttributes::default()
    }

    /// Sets the window background pixmap.
    pub fn background_pixmap(&mut self, pixmap: &'creation XPixmap<'a>) -> &mut Self {
        self.background_pixmap = Some(pixmap);
        self
    }

    /// Sets the window background pixel.
    pub fn background_pixel(&mut self, pixel: u64) -> &mut Self {
        self.background_pixel = Some(pixel);
        self
    }

    /// Sets the window border pixmap.
    pub fn border_pixmap(&mut self, pixmap: &'creation XPixmap<'a>) -> &mut Self {
        self.border_pixmap = Some(pixmap);
        self
    }

    /// Sets the window border pixel.
    pub fn border_pixel(&mut self, pixel: u64) -> &mut Self {
        self.border_pixel = Some(pixel);
        self
    }

    /// Sets the window bit gravity.
    pub fn bit_gravity(&mut self, gravity: i32) -> &mut Self {
        self.bit_gravity = Some(gravity);
        self
    }

    /// Sets the window gravity.
    pub fn window_gravity(&mut self, gravity: i32) -> &mut Self {
        self.win_gravity = Some(gravity);
        self
    }

    /// Sets the window backing store.
    pub fn backing_store(&mut self, backing_store: BackingWindowStore) -> &mut Self {
        self.backing_store = Some(backing_store);
        self
    }

    /// Sets the window backing planes.
    pub fn backing_planes(&mut self, planes: u64) -> &mut Self {
        self.backing_planes = Some(planes);
        self
    }

    /// Sets the window backing pixel.
    pub fn backing_pixels(&mut self, pixels: u64) -> &mut Self {
        self.backing_pixels = Some(pixels);
        self
    }

    /// Sets whether bits under should be saved.
    pub fn save_under(&mut self, save: bool) -> &mut Self {
        self.save_under = Some(save);
        self
    }

    /// Sets the window event mask.
    pub fn event_mask(&mut self, mask: WindowInputMask) -> &mut Self {
        self.event_mask = Some(mask);
        self
    }

    /// Sets the window do-not-propagate mask.
    pub fn do_not_propagate_mask(&mut self, mask: WindowInputMask) -> &mut Self {
        self.do_not_propagate_mask = Some(mask);
        self
    }

    /// Sets whether override redirect is enabled for this window.
    pub fn override_redirect(&mut self, override_redirect: bool) -> &mut Self {
        self.override_redirect = Some(override_redirect);
        self
    }

    /// Sets the window colormap.
    pub fn colormap(&mut self, colormap: &'creation XColormap<'a>) -> &mut Self {
        self.colormap = Some(colormap);
        self
    }

    /// Sets the window cursor.
    pub fn cursor(&mut self, cursor: &'creation XCursor<'a>) -> &mut Self {
        self.cursor = Some(cursor);
        self
    }

    /// Turns this struct into its native representation along with the associated value mask.
    pub fn into_native(self) -> (u64, xlib_sys::XSetWindowAttributes) {
        let mut mask = 0;
        let mut native = unsafe { std::mem::zeroed::<xlib_sys::XSetWindowAttributes>() };

        if let Some(pixmap) = self.background_pixmap {
            native.background_pixmap = pixmap.handle();
            mask |= xlib_sys::CWBackPixmap;
        }

        if let Some(pixel) = self.background_pixel {
            native.background_pixel = pixel;
            mask |= xlib_sys::CWBackPixel;
        }

        if let Some(pixmap) = self.border_pixmap {
            native.border_pixmap = pixmap.handle();
            mask |= xlib_sys::CWBorderPixmap;
        }

        if let Some(pixel) = self.border_pixel {
            native.border_pixel = pixel;
            mask |= xlib_sys::CWBorderPixel;
        }

        if let Some(gravity) = self.bit_gravity {
            native.bit_gravity = gravity;
            mask |= xlib_sys::CWBitGravity;
        }

        if let Some(gravity) = self.win_gravity {
            native.win_gravity = gravity;
            mask |= xlib_sys::CWWinGravity;
        }

        if let Some(store) = self.backing_store {
            native.backing_store = store as _;
            mask |= xlib_sys::CWBackingStore;
        }

        if let Some(planes) = self.backing_planes {
            native.backing_planes = planes;
            mask |= xlib_sys::CWBackingPlanes;
        }

        if let Some(pixel) = self.backing_pixels {
            native.backing_pixel = pixel;
            mask |= xlib_sys::CWBackingPixel;
        }

        if let Some(override_redirect) = self.override_redirect {
            native.override_redirect = override_redirect as _;
            mask |= xlib_sys::CWOverrideRedirect;
        }

        if let Some(save_under) = self.save_under {
            native.save_under = save_under as _;
            mask |= xlib_sys::CWSaveUnder;
        }

        if let Some(event_mask) = self.event_mask {
            native.event_mask = event_mask.bits;
            mask |= xlib_sys::CWEventMask;
        }

        if let Some(do_not_propagate_mask) = self.do_not_propagate_mask {
            native.do_not_propagate_mask = do_not_propagate_mask.bits;
            mask |= xlib_sys::CWDontPropagate;
        }

        if let Some(colormap) = self.colormap {
            native.colormap = colormap.handle();
            mask |= xlib_sys::CWColormap;
        }

        if let Some(cursor) = self.cursor {
            native.cursor = cursor.handle();
            mask |= xlib_sys::CWCursor;
        }

        (mask, native)
    }
}

/// Describes the possible format of a X11 window property.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum WindowPropertyDataFormat {
    /// One property element is 8 bits long
    Bit8,

    /// One property element is 16 bits long
    Bit16,

    /// One property element is 32 bits long
    Bit32,
}

impl WindowPropertyDataFormat {
    /// Attempts to convert the format from the X11 native representation.
    ///
    /// # Arguments
    ///
    /// * `format` - The native format, must be one of 8, 16 or 32
    pub fn from_native(format: i32) -> Option<Self> {
        match format {
            8 => Some(WindowPropertyDataFormat::Bit8),
            16 => Some(WindowPropertyDataFormat::Bit16),
            32 => Some(WindowPropertyDataFormat::Bit32),
            _ => None,
        }
    }

    /// Converts this format to the native representation.
    pub fn to_native(&self) -> i32 {
        match self {
            WindowPropertyDataFormat::Bit8 => 8,
            WindowPropertyDataFormat::Bit16 => 16,
            WindowPropertyDataFormat::Bit32 => 32,
        }
    }

    /// Returns the amount of bytes per property.
    pub fn byte_count(&self) -> usize {
        match self {
            WindowPropertyDataFormat::Bit8 => 1,
            WindowPropertyDataFormat::Bit16 => 2,
            WindowPropertyDataFormat::Bit32 => 4,
        }
    }

    /// Returns the amount of bytes for a property array.
    ///
    /// # Arguments
    ///
    /// * `length` - The length of the array
    pub fn byte_count_array(&self, length: usize) -> usize {
        self.byte_count() * length
    }
}

/// Represents data held by a window property.
#[derive(Debug)]
pub struct WindowPropertyData<'a> {
    format: WindowPropertyDataFormat,
    actual_type: XAtom<'a>,
    item_count: usize,
    data: *mut u8,
}

impl<'a> WindowPropertyData<'a> {
    /// Wraps native window property data.
    ///
    /// # Arguments
    ///
    /// * `format` - The format of the data
    /// * `actual_type` - The actual type of the data as reported by the X server
    /// * `item_count` - The amount of properties stored in the data
    /// * `data` - A pointer to the beginning of the stored data
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure all arguments are valid.
    pub unsafe fn new(
        format: WindowPropertyDataFormat,
        actual_type: XAtom<'a>,
        item_count: usize,
        data: *mut u8,
    ) -> Self {
        Self {
            format,
            actual_type,
            item_count,
            data,
        }
    }

    /// Retrieves the format of the property elements.
    pub fn format(&self) -> WindowPropertyDataFormat {
        self.format
    }

    /// Retrieves the type of the property elements as reported by the X server.
    pub fn ty(&self) -> XAtom<'a> {
        self.actual_type
    }

    /// Retrieves the amount of properties in the data.
    pub fn length(&self) -> usize {
        self.item_count
    }

    /// Retrieves the size of the entire data in bytes.
    pub fn byte_size(&self) -> usize {
        self.format.byte_count_array(self.item_count)
    }

    /// Interprets the data as a pointer of a specific type.
    ///
    /// # Panics
    ///
    /// If the size of the stored data is smaller than the size of the requested type.
    pub fn get_as_ptr<T>(&self) -> *const T {
        assert!(self.byte_size() < std::mem::size_of::<T>());

        self.data as _
    }

    /// Interprets the data as a mutable pointer of a specific type.
    ///
    /// # Panics
    ///
    /// If the size of the stored data is smaller than the size of the requested type.
    pub fn get_as_mut_ptr<T>(&self) -> *mut T {
        assert!(self.byte_size() < std::mem::size_of::<T>());

        self.data as _
    }

    /// Interprets the data as a reference of a specific type.
    ///
    /// # Panics
    ///
    /// If the size of the stored data is smaller than the size of the requested type.
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure that the underlying data is valid for the requested type.
    pub unsafe fn get_as_ref<T>(&self) -> &T {
        &*self.get_as_ptr::<T>()
    }

    /// Interprets the data as a mutable reference of a specific type.
    ///
    /// # Panics
    ///
    /// If the size of the stored data is smaller than the size of the requested type.
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure that the underlying data is valid for the requested type.
    pub unsafe fn get_as_mut_ref<T>(&mut self) -> &mut T {
        &mut *self.get_as_mut_ptr::<T>()
    }
}

impl<'a> Drop for WindowPropertyData<'a> {
    fn drop(&mut self) {
        unsafe { xlib_sys::XFree(self.data as _) };
    }
}

/// Describes how the change of a window property is performed.
#[derive(Debug)]
pub enum WindowPropertyChangeMode {
    Replace,
    Prepend,
    Append,
}

impl WindowPropertyChangeMode {
    /// Converts the change mode the native X11 representation.
    pub fn to_native(&self) -> i32 {
        match self {
            WindowPropertyChangeMode::Replace => xlib_sys::PropModeReplace,
            WindowPropertyChangeMode::Prepend => xlib_sys::PropModePrepend,
            WindowPropertyChangeMode::Append => xlib_sys::PropModeAppend,
        }
    }
}

/// Describes how a window handle is owned
#[derive(Debug)]
pub enum WindowHandleOwnership {
    /// The window handle is not owned at all
    Foreign,

    /// The window is our own handle
    Owned,

    /// The window handle is owned by us, but is the composite window
    OwnedCompositeOverlay,
}

bitflags::bitflags! {
    /// Determines which events are sent to the X11 client.
    pub struct WindowInputMask: i64 {
        /// No events are sent at all
        const NO_EVENT_MASK = xlib_sys::NoEventMask;

        /// Keyboard key press events are sent
        const KEY_PRESS = xlib_sys::KeyPressMask;

        /// Keyboard key release events are sent
        const KEY_RELEASE = xlib_sys::KeyReleaseMask;

        /// Mouse button press events are sent
        const BUTTON_PRESS = xlib_sys::ButtonPressMask;

        /// Mouse button release events are sent
        const BUTTON_RELEASE = xlib_sys::ButtonReleaseMask;

        /// Events when the mouse enters the window are sent
        const ENTER_WINDOW = xlib_sys::EnterWindowMask;

        /// Events when the mouse leaves the window are sent
        const LEAVE_WINDOW = xlib_sys::LeaveWindowMask;

        /// Events when the mouse pointer is moving are sent
        const POINTER_MOTION = xlib_sys::PointerMotionMask;

        /// Events when the mouse pointer is moving are sent, but without metadata
        const POINTER_MOTION_HINT = xlib_sys::PointerMotionHintMask;

        /// Events when the mouse pointer is moving and button 1 is pressed are sent
        const BUTTON_1_MOTION = xlib_sys::Button1MotionMask;

        /// Events when the mouse pointer is moving and button 2 is pressed are sent
        const BUTTON_2_MOTION = xlib_sys::Button2MotionMask;

        /// Events when the mouse pointer is moving and button 3 is pressed are sent
        const BUTTON_3_MOTION = xlib_sys::Button3MotionMask;

        /// Events when the mouse pointer is moving and button 4 is pressed are sent
        const BUTTON_4_MOTION = xlib_sys::Button4MotionMask;

        /// Events when the mouse pointer is moving and button 5 is pressed are sent
        const BUTTON_5_MOTION = xlib_sys::Button5MotionMask;

        /// Events when the mouse pointer is moving and any button is pressed are sent
        const BUTTON_MOTION = xlib_sys::ButtonMotionMask;

        /// Events when the keymap changes are sent
        const KEYMAP_STATE = xlib_sys::KeymapStateMask;

        /// Events when the window has been exposed are sent
        const EXPOSURE = xlib_sys::ExposureMask;

        /// Events when the window visibility changes are sent
        const VISIBILITY_CHANGE = xlib_sys::VisibilityChangeMask;

        /// Events are sent when the window hierarchy structure changes
        const STRUCTURE = xlib_sys::StructureNotifyMask;

        /// Events are sent when the window should be resized
        const RESIZE_REDIRECT = xlib_sys::ResizeRedirectMask;

        /// Events are sent when the window child hierarchy structure changes
        const SUBSTRUCTURE = xlib_sys::SubstructureNotifyMask;

        /// Events are sent when the window child hierarchy structure should be changed
        const SUBSTRUCTURE_REDIRECT = xlib_sys::SubstructureRedirectMask;

        /// Events are sent when the window focus changes
        const FOCUS_CHANGE = xlib_sys::FocusChangeMask;

        /// Events are sent when a window property changes
        const PROPERTY_CHANGE = xlib_sys::PropertyChangeMask;

        /// Events are sent when the colormap changes
        const COLORMAP_CHANGE = xlib_sys::ColormapChangeMask;

        /// Automatic grabs active with owner events
        const OWNER_GRAB_BUTTON = xlib_sys::OwnerGrabButtonMask;
    }
}

bitflags::bitflags! {
    /// Determines which cursor events are sent to the X11 client.
    pub struct CursorInputMask: i64 {
        /// The global cursor has changed.
        const CURSOR_NOTIFY = xfixes_sys::XFixesDisplayCursorNotifyMask;
    }
}

/// Represents a window on the X server.
#[derive(Debug)]
pub struct XWindow<'a> {
    handle: xlib_sys::Window,
    display: &'a XDisplay,
    ownership: WindowHandleOwnership,
}

impl<'a> XWindow<'a> {
    /// Wraps an existing window native X11 window handle.
    ///
    /// Depending on the ownership type, the window may or may not be destroyed when it goes
    /// out of scope.
    ///
    /// # Arguments
    ///
    /// * `handle` - The native X11 window to wrap
    /// * `display` - The X11 display the window belongs to
    /// * `ownership` - The ownership of the passed window handle
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure that all arguments are valid.
    pub unsafe fn new(
        handle: xlib_sys::Window,
        display: &'a XDisplay,
        ownership: WindowHandleOwnership,
    ) -> Self {
        Self {
            handle,
            display,
            ownership,
        }
    }

    /// Retrieves the underlying native X11 window handle.
    pub fn handle(&self) -> xlib_sys::Window {
        self.handle
    }

    /// Retrieves the attributes of the window.
    pub fn get_attributes(&self) -> XWindowAttributes<'a> {
        let mut raw = MaybeUninit::uninit();
        let raw = unsafe {
            xlib_sys::XGetWindowAttributes(self.display.handle(), self.handle, raw.as_mut_ptr());

            raw.assume_init()
        };

        unsafe {
            let screen = XScreen::new(raw.screen, self.display);
            let visual = XVisual::new(raw.visual);

            XWindowAttributes::new(raw, screen, visual)
        }
    }

    /// Clears the content area of the window.
    pub fn clear(&self) {
        unsafe { xlib_sys::XClearWindow(self.display.handle(), self.handle) };
    }

    /// Clears a part of the content area of the window.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate to start clearing at
    /// * `y` - The y coordinate to start clearing at
    /// * `width` - The width of the area to clear
    /// * `height` - The height of the area to clear
    /// * `exposures` - Whether an exposure event should be generated
    pub fn clear_area(&self, x: i32, y: i32, width: u32, height: u32, exposures: bool) {
        unsafe {
            xlib_sys::XClearArea(
                self.display.handle(),
                self.handle,
                x,
                y,
                width,
                height,
                exposures as _,
            )
        };
    }

    /// Maps the window to screen.
    pub fn map(&self) {
        unsafe { xlib_sys::XMapWindow(self.display.handle(), self.handle) };
    }

    /// Unmaps the window from screen
    pub fn unmap(&self) {
        unsafe { xlib_sys::XUnmapWindow(self.display.handle(), self.handle) };
    }

    /// Selects the input mask for the window
    pub fn select_input(&self, mask: WindowInputMask) {
        unsafe { xlib_sys::XSelectInput(self.display.handle(), self.handle, mask.bits) };
    }

    /// Selects the cursor input mask for the window
    pub fn select_cursor_input(&self, mask: CursorInputMask) {
        unsafe {
            xfixes_sys::XFixesSelectCursorInput(self.display.handle(), self.handle, mask.bits as _)
        }
    }

    /// Store the name of the window.
    ///
    /// This is usually what gets displayed as the window title.
    ///
    /// # Arguments
    ///
    /// * `name` - The new name to store
    ///
    /// # Panics
    ///
    /// If name contains a nul byte.
    pub fn store_name(&self, name: impl AsRef<str>) {
        let name = CString::new(name.as_ref()).unwrap();

        unsafe { xlib_sys::XStoreName(self.display.handle(), self.handle, name.as_ptr()) };
    }

    /// Attempts to retrieve a window property.
    ///
    /// This functions returns (if available) the read data and amount of remaining bytes.
    ///
    /// # Arguments
    ///
    /// * `property` - The X atom identifying the property
    /// * `offset` - The byte offset into the property to start reading at
    /// * `length` - The maximal amount of bytes to read
    /// * `delete` - Whether the property should be deleted upon retrieval
    /// * `ty` - The X atom identifying the expected type of the property
    pub fn get_property(
        &self,
        property: XAtom,
        offset: i64,
        length: i64,
        delete: bool,
        ty: XAtom,
    ) -> Option<(WindowPropertyData, usize)> {
        let mut actual_type = 0;
        let mut actual_format = 0;
        let mut item_count = 0;
        let mut remaining_bytes = 0;
        let mut data = std::ptr::null_mut();

        let delete = i32::from(delete);

        unsafe {
            xlib_sys::XGetWindowProperty(
                self.display.handle(),
                self.handle,
                property.handle(),
                offset,
                length,
                delete,
                ty.handle(),
                &mut actual_type,
                &mut actual_format,
                &mut item_count,
                &mut remaining_bytes,
                &mut data,
            )
        };

        WindowPropertyDataFormat::from_native(actual_format).map(|format| {
            let actual_type = unsafe { XAtom::new(actual_type, self.display) };
            let data =
                unsafe { WindowPropertyData::new(format, actual_type, item_count as _, data) };

            (data, remaining_bytes as _)
        })
    }

    /// Changes a property in 8 bit format,
    ///
    /// # Arguments
    ///
    /// * `property` - The X atom identifying the property
    /// * `ty` - The X atom identifying the property type
    /// * `mode` - How the property should be changed
    /// * `data` - The data to work with (interpretation depends on `mode`)
    pub fn change_property8(
        &self,
        property: XAtom,
        ty: XAtom,
        mode: WindowPropertyChangeMode,
        data: &[u8],
    ) {
        // XChangeProperty never writes to data, but it is not defined as const in C
        #[allow(mutable_transmutes)]
        let data = unsafe { std::mem::transmute::<_, &mut [u8]>(data) };
        let element_count = data.len();

        unsafe {
            self.change_property(
                property,
                ty,
                WindowPropertyDataFormat::Bit8,
                mode,
                data.as_mut_ptr(),
                element_count,
            )
        };
    }

    /// Changes a property in 16 bit format,
    ///
    /// # Arguments
    ///
    /// * `property` - The X atom identifying the property
    /// * `ty` - The X atom identifying the property type
    /// * `mode` - How the property should be changed
    /// * `data` - The data to work with (interpretation depends on `mode`)
    pub fn change_property16(
        &self,
        property: XAtom,
        ty: XAtom,
        mode: WindowPropertyChangeMode,
        data: &[i16],
    ) {
        // XChangeProperty never writes to data, but it is not defined as const in C
        #[allow(mutable_transmutes)]
        let data = unsafe { std::mem::transmute::<_, &mut [i16]>(data) };
        let element_count = data.len();

        unsafe {
            self.change_property(
                property,
                ty,
                WindowPropertyDataFormat::Bit16,
                mode,
                data.as_mut_ptr() as _,
                element_count,
            )
        };
    }

    /// Changes a property in 32 bit format,
    ///
    /// # Arguments
    ///
    /// * `property` - The X atom identifying the property
    /// * `ty` - The X atom identifying the property type
    /// * `mode` - How the property should be changed
    /// * `data` - The data to work with (interpretation depends on `mode`)
    pub fn change_property32(
        &self,
        property: XAtom,
        ty: XAtom,
        mode: WindowPropertyChangeMode,
        data: &[i32],
    ) {
        // XChangeProperty never writes to data, but it is not defined as const in C
        #[allow(mutable_transmutes)]
        let data = unsafe { std::mem::transmute::<_, &mut [i32]>(data) };
        let element_count = data.len();

        unsafe {
            self.change_property(
                property,
                ty,
                WindowPropertyDataFormat::Bit32,
                mode,
                data.as_mut_ptr() as _,
                element_count,
            )
        };
    }

    /// Changes a property,
    ///
    /// # Arguments
    ///
    /// * `property` - The X atom identifying the property
    /// * `ty` - The X atom identifying the property type
    /// * `format` - The format of the property
    /// * `mode` - How the property should be changed
    /// * `data` - The data to work with (interpretation depends on `mode`)
    /// * `element_count` - The amount of elements stored in `data`
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure all arguments are valid.
    pub unsafe fn change_property(
        &self,
        property: XAtom,
        ty: XAtom,
        format: WindowPropertyDataFormat,
        mode: WindowPropertyChangeMode,
        data: *mut u8,
        element_count: usize,
    ) {
        xlib_sys::XChangeProperty(
            self.display.handle(),
            self.handle,
            property.handle(),
            ty.handle(),
            format.to_native(),
            mode.to_native(),
            data,
            element_count as _,
        );
    }

    /// Deletes a property if it exists from the window.
    ///
    /// # Arguments
    ///
    /// * `property` - The X atom identifying the property
    pub fn delete_property(&self, property: XAtom) {
        unsafe { xlib_sys::XDeleteProperty(self.display.handle(), self.handle, property.handle()) };
    }

    /// Replaces the `WM_PROTOCOLS` property on the window.
    ///
    /// # Arguments
    ///
    /// * `protocols` - The protocols to set
    pub fn set_wm_protocols(&self, protocols: &[XAtom<'a>]) {
        let mut protocols = protocols.iter().map(|v| v.handle()).collect::<Vec<_>>();

        unsafe {
            xlib_sys::XSetWMProtocols(
                self.display.handle(),
                self.handle,
                protocols.as_mut_ptr(),
                protocols.len() as _,
            )
        };
    }

    /// Creates a new child window of this window.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate of the window, relative to the inside border of this window
    /// * `y` - The y coordinate of the window, relative to the inside border of this window
    /// * `width` - The width in pixels of the inside window area excluding the border
    /// * `height` - The height in pixels of the inside window area excluding the border
    /// * `border_width` - The width of the window border in pixels
    /// * `border` - The border pixel value of the window
    /// * `background` - The background pixel value of the window
    #[allow(clippy::too_many_arguments)]
    pub fn create_simple_child_window(
        &self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        border_width: u32,
        border: u64,
        background: u64,
    ) -> XWindow<'a> {
        unsafe {
            let window = xlib_sys::XCreateSimpleWindow(
                self.display.handle(),
                self.handle,
                x,
                y,
                width,
                height,
                border_width,
                border,
                background,
            );

            XWindow::new(window, self.display, WindowHandleOwnership::Owned)
        }
    }

    /// Creates a new child window of this window.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate of the window, relative to the inside border of this window
    /// * `y` - The y coordinate of the window, relative to the inside border of this window
    /// * `width` - The width in pixels of the inside window area excluding the border
    /// * `height` - The height in pixels of the inside window area excluding the border
    /// * `border_width` - The width of the window border in pixels
    /// * `depth` - The depth value of the window
    /// * `visual` - The visual type of the window
    /// * `attributes` - The attributes to set on the window
    #[allow(clippy::too_many_arguments)]
    pub fn create_child_window<'creation>(
        &'creation self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        border_width: u32,
        depth: i32,
        class: WindowClass,
        visual: &'creation XVisual<'a>,
        attributes: SetWindowAttributes<'creation, 'a>,
    ) -> XWindow<'a> {
        let (value_mask, mut attributes) = attributes.into_native();

        unsafe {
            let window = xlib_sys::XCreateWindow(
                self.display.handle(),
                self.handle,
                x,
                y,
                width,
                height,
                border_width,
                depth,
                class as _,
                visual.handle(),
                value_mask,
                &mut attributes,
            );

            XWindow::new(window, self.display, WindowHandleOwnership::Owned)
        }
    }

    /// Changes a region of this window.
    ///
    /// # Arguments
    ///
    /// * `shape_kind` - The kind of shape to adjust
    /// * `x_offset` - The x coordinate from where to start the region
    /// * `y_offset` - The y coordinate from where to start the region
    /// * `region` - The region to apply
    pub fn set_shape_region(
        &self,
        shape_kind: WindowShapeKind,
        x_offset: i32,
        y_offset: i32,
        region: &XServerRegion,
    ) {
        unsafe {
            xfixes_sys::XFixesSetWindowShapeRegion(
                self.display.handle(),
                self.handle,
                shape_kind as _,
                x_offset,
                y_offset,
                region.handle(),
            );
        }
    }
}

impl<'a> Drop for XWindow<'a> {
    fn drop(&mut self) {
        match self.ownership {
            WindowHandleOwnership::Foreign => {}
            WindowHandleOwnership::Owned => unsafe {
                xlib_sys::XDestroyWindow(self.display.handle(), self.handle);
            },
            WindowHandleOwnership::OwnedCompositeOverlay => unsafe {
                xcomposite_sys::XCompositeReleaseOverlayWindow(self.display.handle(), self.handle);
            },
        }
    }
}

impl<'a> XDrawable<'a> for XWindow<'a> {
    fn drawable_handle(&self) -> xlib_sys::Drawable {
        self.handle
    }

    fn display(&self) -> &'a XDisplay {
        self.display
    }
}

/// Properties of an X11 window.
#[derive(Debug)]
pub struct XWindowAttributes<'a> {
    #[allow(dead_code)]
    inner: xlib_sys::XWindowAttributes,
    screen: XScreen<'a>,
    visual: XVisual<'a>,
}

impl<'a> XWindowAttributes<'a> {
    /// Wraps native X11 window properties.
    ///
    /// # Arguments
    ///
    /// * `inner` - The native X11 window attributes data
    /// * `screen` - The screen depicted in the window attributes
    /// * `visual` - The visual depicted in the window attributes
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure all arguments are valid.
    pub unsafe fn new(
        inner: xlib_sys::XWindowAttributes,
        screen: XScreen<'a>,
        visual: XVisual<'a>,
    ) -> Self {
        Self {
            inner,
            screen,
            visual,
        }
    }

    /// Retrieves the screen of the window these attributes describe.
    pub fn screen(&self) -> &XScreen<'a> {
        &self.screen
    }

    /// Retrieves the visual of the window these attributes describe
    pub fn visual(&self) -> &XVisual<'a> {
        &self.visual
    }
}
