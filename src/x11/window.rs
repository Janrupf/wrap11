use crate::{
    xcomposite_sys, xfixes_sys, xinput2_sys, xlib_sys, XAtom, XColormap, XCursor, XDisplay,
    XDrawable, XPixmap, XPropertyHolder, XScreen, XServerRegion, XVisual,
};
use std::ffi::{CStr, CString};

use crate::x11::input::XInputDevice;
use crate::x11::property::{XPropertyChangeMode, XPropertyData, XPropertyDataFormat};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
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
            native.event_mask = event_mask.bits();
            mask |= xlib_sys::CWEventMask;
        }

        if let Some(do_not_propagate_mask) = self.do_not_propagate_mask {
            native.do_not_propagate_mask = do_not_propagate_mask.bits();
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

bitflags::bitflags! {
    /// Determines which XInput2 events are sent to the X11 client.
    pub struct XInputEventMask: i32 {
        /// Events are sent when a device has changed.
        const DEVICE_CHANGED = xinput2_sys::XI_DeviceChangedMask;

        /// Events are sent when a key has been pressed.
        const KEY_PRESS = xinput2_sys::XI_KeyPressMask;

        /// Events are sent when a key has been released.
        const KEY_RELEASE = xinput2_sys::XI_KeyReleaseMask;

        /// Events are sent when a button has been pressed.
        const BUTTON_PRESS = xinput2_sys::XI_ButtonPressMask;

        /// Events are sent when a button has been released.
        const BUTTON_RELEASE = xinput2_sys::XI_ButtonReleaseMask;

        /// Events are sent when an input device moved the cursor.
        const MOTION = xinput2_sys::XI_MotionMask;

        /// Events are sent when the cursor entered.
        const ENTER = xinput2_sys::XI_EnterMask;

        /// Events are sent when the cursor left.
        const LEAVE = xinput2_sys::XI_LeaveMask;

        /// Events are sent when something has been focused.
        const FOCUS_IN = xinput2_sys::XI_FocusInMask;

        /// Events are sent when something has been unfocused.
        const FOCUS_OUT = xinput2_sys::XI_FocusOutMask;

        /// Events are sent when the hierarchy changed.
        const HIERARCHY_CHANGED = xinput2_sys::XI_HierarchyChangedMask;

        /// Events are sent when a property changed.
        const PROPERTY_CHANGE = xinput2_sys::XI_PropertyEventMask;

        /// Events are sent when a raw input device pressed a key.
        const RAW_KEY_PRESS = xinput2_sys::XI_RawKeyPressMask;

        /// Events are sent when a raw input device released a key.
        const RAW_KEY_RELEASE = xinput2_sys::XI_RawKeyReleaseMask;

        /// Events are sent when a raw input device pressed a button.
        const RAW_BUTTON_PRESS = xinput2_sys::XI_RawButtonPress;

        /// Events are sent when a raw input device released a button.
        const RAW_BUTTON_RELEASE = xinput2_sys::XI_RawButtonRelease;

        /// Events are sent when a raw input device moved the cursor.
        const RAW_MOTION = xinput2_sys::XI_RawMotionMask;

        /// Events are sent when a touch started.
        const TOUCH_BEGIN = xinput2_sys::XI_TouchBeginMask;

        /// Events are sent when a touch ends.
        const TOUCH_END = xinput2_sys::XI_TouchEndMask;

        /// Events are sent when a touch ownership changed.
        const TOUCH_OWNERSHIP_CHANGED = xinput2_sys::XI_TouchOwnershipChangedMask;

        /// Events are sent when a touch updated.
        const TOUCH_UPDATE = xinput2_sys::XI_TouchUpdateMask;

        /// Events are sent when a raw input device started a touch.
        const RAW_TOUCH_BEGIN = xinput2_sys::XI_RawTouchBeginMask;

        /// Events are sent when a raw input device ended a touch.
        const RAW_TOUCH_END = xinput2_sys::XI_RawTouchEndMask;

        /// Events are sent when a raw input device updated a touch.
        const RAW_TOUCH_UPDATE = xinput2_sys::XI_RawTouchUpdateMask;

        /// Events are sent when a barrier has been hit.
        const BARRIER_HIT = xinput2_sys::XI_BarrierHitMask;

        /// Events are sent when a barrier has been left.
        const BARRIER_LEAVE = xinput2_sys::XI_BarrierLeaveMask;
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

    /// Moves the window to the specified position.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate to move the window to
    /// * `y` - The y coordinate to move the window to
    pub fn move_to(&self, x: i32, y: i32) {
        unsafe { xlib_sys::XMoveWindow(self.display.handle(), self.handle, x, y) };
    }

    /// Selects the input mask for the window
    pub fn select_input(&self, mask: WindowInputMask) {
        unsafe { xlib_sys::XSelectInput(self.display.handle(), self.handle, mask.bits()) };
    }

    /// Selects the cursor input mask for the window
    pub fn select_cursor_input(&self, mask: CursorInputMask) {
        unsafe {
            xfixes_sys::XFixesSelectCursorInput(
                self.display.handle(),
                self.handle,
                mask.bits() as _,
            )
        }
    }

    /// Selects the XInput mask for the window
    pub fn select_xinput_events(&self, mask: Vec<(XInputDevice, XInputEventMask)>) {
        let mut event_mask_bytes = Vec::with_capacity(mask.len());
        let mut raw_event_masks = Vec::with_capacity(mask.len());

        for (device, mask) in mask {
            let numeric_mask = mask.bits();
            event_mask_bytes.push(numeric_mask);

            let numeric_mask_ptr =
                unsafe { event_mask_bytes.as_ptr().add(event_mask_bytes.len() - 1) };

            raw_event_masks.push(xinput2_sys::XIEventMask {
                deviceid: device.id(),
                mask: numeric_mask_ptr as _,
                mask_len: std::mem::size_of::<i32>() as _,
            });
        }

        unsafe {
            xinput2_sys::XISelectEvents(
                self.display.handle(),
                self.handle,
                raw_event_masks.as_mut_ptr(),
                raw_event_masks.len() as _,
            )
        };
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

    /// Retrieves information about the position of the window in the tree.
    pub fn query_tree(&self) -> XWindowTreeInfo<'a> {
        let mut root = 0;
        let mut parent = 0;

        let children = unsafe {
            let mut children = std::ptr::null_mut();
            let mut children_len = 0;

            xlib_sys::XQueryTree(
                self.display.handle(),
                self.handle,
                &mut root,
                &mut parent,
                &mut children,
                &mut children_len,
            );

            if children_len == 0 || children.is_null() {
                Vec::<XWindow<'a>>::new()
            } else {
                let out = std::slice::from_raw_parts(children, children_len as _)
                    .iter()
                    .map(|&w| XWindow::new(w, self.display, WindowHandleOwnership::Foreign))
                    .collect();

                xlib_sys::XFree(children as _);

                out
            }
        };

        let root = unsafe { XWindow::new(root, self.display, WindowHandleOwnership::Foreign) };
        let parent = unsafe { XWindow::new(parent, self.display, WindowHandleOwnership::Foreign) };

        XWindowTreeInfo::new(root, parent, children)
    }

    /// Retrieves the window name (this is usually what is displayed as its title).
    pub fn fetch_name(&self) -> Option<String> {
        let mut name_out = std::ptr::null_mut();
        let status =
            unsafe { xlib_sys::XFetchName(self.display.handle(), self.handle, &mut name_out) };

        if status != 0 {
            let name = unsafe { CStr::from_ptr(name_out) }
                .to_string_lossy()
                .to_string();
            unsafe { xlib_sys::XFree(name_out as _) };

            Some(name)
        } else {
            None
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

    /// Changes the parent window of this window.
    ///
    /// # Arguments
    /// * `new_parent` - The new parent window of this window
    /// * `x` - The x position inside the new parent
    /// * `y` - The y position inside the new parent
    pub fn reparent(&self, new_parent: &XWindow, x: i32, y: i32) {
        unsafe {
            xlib_sys::XReparentWindow(self.display.handle(), self.handle, new_parent.handle, x, y)
        };
    }

    /// Clones this window into a foreign window handle.
    pub fn foreign_clone(&self) -> XWindow<'a> {
        unsafe { XWindow::new(self.handle, self.display, WindowHandleOwnership::Foreign) }
    }
}

impl<'a> XPropertyHolder for XWindow<'a> {
    fn get_property(
        &self,
        property: XAtom,
        offset: i64,
        length: i64,
        delete: bool,
        ty: XAtom,
    ) -> Option<(XPropertyData, usize)> {
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

        XPropertyDataFormat::from_native(actual_format).map(|format| {
            let actual_type = unsafe { XAtom::new(actual_type, self.display) };
            let data = unsafe { XPropertyData::new(format, actual_type, item_count as _, data) };

            (data, remaining_bytes as _)
        })
    }

    unsafe fn change_property_unsafe(
        &self,
        property: XAtom,
        ty: XAtom,
        format: XPropertyDataFormat,
        mode: XPropertyChangeMode,
        data: *mut u8,
        element_count: usize,
    ) {
        xlib_sys::XChangeProperty(
            self.display.handle(),
            self.handle,
            property.handle(),
            ty.handle(),
            format.to_native(),
            mode as _,
            data,
            element_count as _,
        );
    }

    fn delete_property(&self, property: XAtom) {
        unsafe { xlib_sys::XDeleteProperty(self.display.handle(), self.handle, property.handle()) };
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

impl<'a> PartialEq for XWindow<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle && self.display.handle() == other.display.handle()
    }
}

impl<'a> Eq for XWindow<'a> {}

impl<'a> Hash for XWindow<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.handle.hash(state);
        self.display.handle().hash(state);
    }
}

/// The tree around an X11 window.
#[derive(Debug)]
pub struct XWindowTreeInfo<'a> {
    root: XWindow<'a>,
    parent: XWindow<'a>,
    children: Vec<XWindow<'a>>,
}

impl<'a> XWindowTreeInfo<'a> {
    /// Collects information about the tree around an X11 window.
    ///
    /// # Arguments
    ///
    /// * `root` - The root window
    /// * `parent` - The parent window
    /// * `children` - All child windows
    pub fn new(root: XWindow<'a>, parent: XWindow<'a>, children: Vec<XWindow<'a>>) -> Self {
        Self {
            root,
            parent,
            children,
        }
    }

    /// Retrieves the root window
    pub fn root(&self) -> &XWindow<'a> {
        &self.root
    }

    /// Retrieves the parent
    pub fn parent(&self) -> &XWindow<'a> {
        &self.parent
    }

    /// Retrieves the child windows
    pub fn children(&self) -> &[XWindow<'a>] {
        &self.children
    }

    /// Discards all information except the root window
    pub fn into_root(self) -> XWindow<'a> {
        self.root
    }

    /// Discards all information except the parent window
    pub fn into_parent(self) -> XWindow<'a> {
        self.parent
    }

    /// Discards all information except the child windows
    pub fn into_children(self) -> Vec<XWindow<'a>> {
        self.children
    }

    /// Splits the information bundle apart into root, parent and child windows.
    pub fn split(self) -> (XWindow<'a>, XWindow<'a>, Vec<XWindow<'a>>) {
        (self.root, self.parent, self.children)
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
