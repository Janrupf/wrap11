use crate::x11::input::XInputDevice;
use crate::{
    xfixes_sys, xinput2_sys, xlib_sys, ColormapHandleOwnership, ColormapState,
    WindowHandleOwnership, XAtom, XColormap, XDisplay, XWindow,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum VisibilityState {
    /// The window is fully visible
    Unobscured = xlib_sys::VisibilityUnobscured,

    /// The window is partially hidden
    PartiallyObscured = xlib_sys::VisibilityPartiallyObscured,

    /// The window is fully hidden
    FullyObscured = xlib_sys::VisibilityFullyObscured,
}

impl VisibilityState {
    /// Wraps an existing X11 visibility state.
    ///
    /// # Arguments
    ///
    /// * `detail` - The native X11 visibility state to wrap
    pub fn new(state: i32) -> Self {
        match state {
            xlib_sys::VisibilityUnobscured => Self::Unobscured,
            xlib_sys::VisibilityPartiallyObscured => Self::PartiallyObscured,
            xlib_sys::VisibilityFullyObscured => Self::FullyObscured,
            x => unreachable!("Invalid X visibility state: {}", x),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum MappingRequestType {
    Modifier = xlib_sys::MappingModifier,
    Keyboard = xlib_sys::MappingKeyboard,
    Pointer = xlib_sys::MappingPointer,
}

impl MappingRequestType {
    /// Wraps an existing X11 mapping request type.
    ///
    /// # Arguments
    ///
    /// * `detail` - The native X11 mapping request type to wrap
    pub fn new(request: i32) -> Self {
        match request {
            xlib_sys::MappingModifier => Self::Modifier,
            xlib_sys::MappingKeyboard => Self::Keyboard,
            xlib_sys::MappingPointer => Self::Pointer,
            x => unreachable!("Invalid X mapping request type: {}", x),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ClientMessageData {
    Bit8([i8; 20]),
    Bit16([i16; 10]),
    Bit32([i32; 5]),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum ConfigureDetail {
    Above = xlib_sys::Above,
    Below = xlib_sys::Below,
    TopIf = xlib_sys::TopIf,
    BottomIf = xlib_sys::BottomIf,
    Opposite = xlib_sys::Opposite,
}

impl ConfigureDetail {
    /// Wraps an existing X11 configure detail.
    ///
    /// # Arguments
    ///
    /// * `detail` - The native X11 configure detail to wrap
    pub fn new(detail: i32) -> Self {
        match detail {
            xlib_sys::Above => Self::Above,
            xlib_sys::Below => Self::Below,
            xlib_sys::TopIf => Self::TopIf,
            xlib_sys::BottomIf => Self::BottomIf,
            xlib_sys::Opposite => Self::Opposite,
            x => unreachable!("Invalid X configure detail: {}", x),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum CirculatePlace {
    /// The window should be placed on top
    Top = xlib_sys::PlaceOnTop,

    /// The window should be placed on the bottom
    Bottom = xlib_sys::PlaceOnBottom,
}

impl CirculatePlace {
    /// Wraps an existing X11 circulate place.
    ///
    /// # Arguments
    ///
    /// * `state` - The native X11 circulate place to wrap
    pub fn new(place: i32) -> Self {
        match place {
            xlib_sys::PlaceOnTop => Self::Top,
            xlib_sys::PlaceOnBottom => Self::Bottom,
            x => unreachable!("Invalid X circulate place: {}", x),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum PropertyState {
    /// The property has a new value
    NewValue = xlib_sys::PropertyNewValue,

    /// The property got deleted
    Delete = xlib_sys::PropertyDelete,
}

impl PropertyState {
    /// Wraps an existing X11 property state.
    ///
    /// # Arguments
    ///
    /// * `state` - The native X11 property state to wrap
    pub fn new(state: i32) -> Self {
        match state {
            xlib_sys::PropertyNewValue => Self::NewValue,
            xlib_sys::PropertyDelete => Self::Delete,
            x => unreachable!("Invalid X property state: {}", x),
        }
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct InputModifierMask: u32 {
        /// Mouse button 1 is down
        const BUTTON_1 = xlib_sys::Button1Mask;

        /// Mouse button 2 is down
        const BUTTON_2 = xlib_sys::Button2Mask;

        /// Mouse button 3 is down
        const BUTTON_3 = xlib_sys::Button3Mask;

        /// Mouse button 4 is down
        const BUTTON_4 = xlib_sys::Button4Mask;

        /// Mouse button 5 is down
        const BUTTON_5 = xlib_sys::Button5Mask;

        /// Shift key is down
        const SHIFT = xlib_sys::ShiftMask;

        /// Lock key is down
        const LOCK = xlib_sys::LockMask;

        /// Control key is down
        const CONTROL = xlib_sys::ControlMask;

        /// Mod 1 key is down
        const MOD_1 = xlib_sys::Mod1Mask;

        /// Mod 2 key is down
        const MOD_2 = xlib_sys::Mod2Mask;

        /// Mod 3 key is down
        const MOD_3 = xlib_sys::Mod3Mask;

        /// Mod 4 key is down
        const MOD_4 = xlib_sys::Mod4Mask;

        /// Mod 5 key is down
        const MOD_5 = xlib_sys::Mod5Mask;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum NotifyMode {
    /// Normal notification
    Normal = xlib_sys::NotifyNormal,

    /// Grabbed notification
    Grab = xlib_sys::NotifyGrab,

    /// Ungrabbed notification
    Ungrab = xlib_sys::NotifyUngrab,
}

impl NotifyMode {
    /// Wraps an existing X11 notify mode.
    ///
    /// # Arguments
    ///
    /// * `mode` - The native X11 notify mode to wrap
    pub fn new(mode: i32) -> Self {
        match mode {
            xlib_sys::NotifyNormal => Self::Normal,
            xlib_sys::NotifyGrab => Self::Grab,
            xlib_sys::NotifyUngrab => Self::Ungrab,
            x => unreachable!("Invalid X notify mode: {}", x),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum NotifyDetail {
    /// Notification from ancestor window
    Ancestor = xlib_sys::NotifyAncestor,

    /// Virtual notification
    Virtual = xlib_sys::NotifyVirtual,

    /// Notification from inferior window
    Inferior = xlib_sys::NotifyInferior,

    /// Notification from non linear related window
    Nonlinear = xlib_sys::NotifyNonlinear,

    /// Virtual notification form non linear related window
    NonlinearVirtual = xlib_sys::NotifyNonlinearVirtual,

    /// Notification from a pointer
    Pointer = xlib_sys::NotifyPointer,

    /// Notification from a root pointer
    PointerRoot = xlib_sys::NotifyPointerRoot,

    /// No detail given
    None = xlib_sys::NotifyDetailNone,
}

impl NotifyDetail {
    /// Wraps an existing X11 notify detail.
    ///
    /// # Arguments
    ///
    /// * `mode` - The native X11 notify detail to wrap
    pub fn new(detail: i32) -> Self {
        match detail {
            xlib_sys::NotifyAncestor => Self::Ancestor,
            xlib_sys::NotifyVirtual => Self::Virtual,
            xlib_sys::NotifyInferior => Self::Inferior,
            xlib_sys::NotifyNonlinear => Self::Nonlinear,
            xlib_sys::NotifyNonlinearVirtual => Self::NonlinearVirtual,
            xlib_sys::NotifyPointer => Self::Pointer,
            xlib_sys::NotifyPointerRoot => Self::PointerRoot,
            xlib_sys::NotifyDetailNone => Self::None,
            x => unreachable!("Invalid X notify detail: {}", x),
        }
    }
}

#[derive(Debug)]
pub struct XEvent<'a> {
    serial: u64,
    send_event: bool,
    window: XWindow<'a>,
    data: XEventData<'a>,
}

impl<'a> XEvent<'a> {
    /// Creates a new X event from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xlib_sys::XEvent, display: &'a XDisplay) -> Self {
        let (serial, send_event, window) = unsafe {
            let serial = event.any.serial;
            let send_event = event.any.send_event != 0;
            let window = XWindow::new(event.any.window, display, WindowHandleOwnership::Foreign);

            (serial, send_event, window)
        };

        let data = XEventData::new(event, display);

        Self {
            serial,
            send_event,
            window,
            data,
        }
    }

    /// Retrieves the event serial.
    pub fn serial(&self) -> u64 {
        self.serial
    }

    /// Determines whether this event was generated by a `SendEvent` request.
    pub fn is_from_send_event(&self) -> bool {
        self.send_event
    }

    /// Retrieves the window this event was generated for.
    ///
    /// X does not clearly define which window this is, other than "the most useful
    /// for toolkit dispatchers". Prefer using the windows contained in the events
    /// themselves, when available!
    pub fn window(&self) -> &XWindow<'a> {
        &self.window
    }

    /// Determines whether this event was sent by a send event call.
    pub fn send_event(&self) -> bool {
        self.send_event
    }

    /// Retrieves the underlying data of this event.
    pub fn data(&self) -> &XEventData<'a> {
        &self.data
    }
}

/// The payload of an event
#[derive(Debug)]
pub enum XEventData<'a> {
    /// The mouse cursor has moved.
    ///
    /// Only generated when the follow input masks are set:
    /// - [`WindowInputMask::POINTER_MOTION`][crate::WindowInputMask::POINTER_MOTION]
    /// - [`WindowInputMask::BUTTON_MOTION`][crate::WindowInputMask::BUTTON_MOTION]
    /// - [`WindowInputMask::BUTTON_1_MOTION`][crate::WindowInputMask::BUTTON_1_MOTION]
    /// - [`WindowInputMask::BUTTON_2_MOTION`][crate::WindowInputMask::BUTTON_2_MOTION]
    /// - [`WindowInputMask::BUTTON_3_MOTION`][crate::WindowInputMask::BUTTON_3_MOTION]
    /// - [`WindowInputMask::BUTTON_4_MOTION`][crate::WindowInputMask::BUTTON_4_MOTION]
    /// - [`WindowInputMask::BUTTON_5_MOTION`][crate::WindowInputMask::BUTTON_5_MOTION]
    Motion(XMotionEvent<'a>),

    /// A key has been pressed.
    ///
    /// Only generated when [`WindowInputMask::KEY_PRESS`][crate::WindowInputMask::KEY_PRESS]
    /// is set.
    KeyPress(XKeyEvent<'a>),

    /// A key has been released
    ///
    /// Only generated when [`WindowInputMask::KEY_RELEASE`][crate::WindowInputMask::KEY_RELEASE]
    /// is set.
    KeyRelease(XKeyEvent<'a>),

    /// A key has been pressed.
    ///
    /// Only generated when [`WindowInputMask::BUTTON_PRESS`][crate::WindowInputMask::BUTTON_PRESS]
    /// is set.    
    ButtonPress(XButtonEvent<'a>),

    /// A key has been released.
    ///
    /// Only generated when [`WindowInputMask::BUTTON_RELEASE`][crate::WindowInputMask::BUTTON_RELEASE]
    /// is set.
    ButtonRelease(XButtonEvent<'a>),

    /// A colormap has been changed.
    ///
    /// Only generated when [`WindowInputMask::COLORMAP_CHANGE`][crate::WindowInputMask::COLORMAP_CHANGE]
    /// is set.
    ColormapChange(XColormapEvent<'a>),

    /// The mouse has entered the window.
    ///
    /// Only generated when [`WindowInputMask::ENTER_WINDOW`][crate::WindowInputMask::ENTER_WINDOW]
    /// is set.
    EnterWindow(XCrossingEvent<'a>),

    /// The mouse has left the window.
    ///
    /// Only generated when [`WindowInputMask::LEAVE_WINDOW`][crate::WindowInputMask::LEAVE_WINDOW]
    /// is set.
    LeaveWindow(XCrossingEvent<'a>),

    /// The window has been exposed and content needs to be drawn.
    ///
    /// Only generated when [`WindowInputMask::EXPOSURE`][crate::WindowInputMask::EXPOSURE]
    /// is set.
    Expose(XExposeEvent),

    /// The window has been focused.
    ///
    /// Only generated when [`WindowInputMask::FOCUS_CHANGE`][crate::WindowInputMask::FOCUS_CHANGE]
    /// is set.
    FocusIn(XFocusChangeEvent),

    /// The window has been unfocused.
    ///
    /// Only generated when [`WindowInputMask::FOCUS_CHANGE`][crate::WindowInputMask::FOCUS_CHANGE]
    /// is set.
    FocusOut(XFocusChangeEvent),

    /// The keymap has changed.
    ///
    /// Only generated when [`WindowInputMask::KEYMAP_STATE`][crate::WindowInputMask::KEYMAP_STATE]
    /// is set.
    KeymapChange(XKeymapEvent),

    /// A window property has changed.
    ///
    /// Only generated when [`WindowInputMask::PROPERTY_CHANGE`][crate::WindowInputMask::PROPERTY_CHANGE]
    /// is set.
    PropertyChange(XPropertyEvent<'a>),

    /// The window should be resized.
    ///
    /// Only generated when [`WindowInputMask::RESIZE_REDIRECT`][crate::WindowInputMask::RESIZE_REDIRECT]
    /// is set.
    ResizeRequest(XResizeRequestEvent),

    /// The window has been circulated.
    ///
    /// Only generated when [`WindowInputMask::STRUCTURE`][crate::WindowInputMask::STRUCTURE]
    /// [`WindowInputMask::SUBSTRUCTURE`][crate::WindowInputMask::SUBSTRUCTURE] or is set.
    Circulate(XCirculateEvent<'a>),

    /// The window has been configured.
    ///
    /// Only generated when [`WindowInputMask::STRUCTURE`][crate::WindowInputMask::STRUCTURE]
    /// [`WindowInputMask::SUBSTRUCTURE`][crate::WindowInputMask::SUBSTRUCTURE] or is set.
    Configure(XConfigureEvent<'a>),

    /// The window has been destroyed.
    ///
    /// Only generated when [`WindowInputMask::STRUCTURE`][crate::WindowInputMask::STRUCTURE]
    /// [`WindowInputMask::SUBSTRUCTURE`][crate::WindowInputMask::SUBSTRUCTURE] or is set.
    Destroy(XDestroyWindowEvent<'a>),

    /// The window has been moved.
    ///
    /// Only generated when [`WindowInputMask::STRUCTURE`][crate::WindowInputMask::STRUCTURE]
    /// [`WindowInputMask::SUBSTRUCTURE`][crate::WindowInputMask::SUBSTRUCTURE] or is set.
    Gravity(XGravityEvent<'a>),

    /// The window has been mapped.
    ///
    /// Only generated when [`WindowInputMask::STRUCTURE`][crate::WindowInputMask::STRUCTURE]
    /// [`WindowInputMask::SUBSTRUCTURE`][crate::WindowInputMask::SUBSTRUCTURE] or is set.
    Map(XMapEvent<'a>),

    /// The window has been reparented.
    ///
    /// Only generated when [`WindowInputMask::STRUCTURE`][crate::WindowInputMask::STRUCTURE]
    /// [`WindowInputMask::SUBSTRUCTURE`][crate::WindowInputMask::SUBSTRUCTURE] or is set.
    Reparent(XReparentEvent<'a>),

    /// The window has been unmapped.
    ///
    /// Only generated when [`WindowInputMask::STRUCTURE`][crate::WindowInputMask::STRUCTURE]
    /// [`WindowInputMask::SUBSTRUCTURE`][crate::WindowInputMask::SUBSTRUCTURE] or is set.
    Unmap(XUnmapEvent<'a>),

    /// A child window has been requested to be circulated.
    ///
    /// Only generated when [`WindowInputMask::SUBSTRUCTURE_REDIRECT`][crate::WindowInputMask::SUBSTRUCTURE_REDIRECT]
    /// is set.
    CirculateRequest(XCirculateRequestEvent<'a>),

    /// A child window has been requested to be configured.
    ///
    /// Only generated when [`WindowInputMask::SUBSTRUCTURE_REDIRECT`][crate::WindowInputMask::SUBSTRUCTURE_REDIRECT]
    /// is set.
    ConfigureRequest(XConfigureRequestEvent<'a>),

    /// A child window has been requested to be mapped.
    ///
    /// Only generated when [`WindowInputMask::SUBSTRUCTURE_REDIRECT`][crate::WindowInputMask::SUBSTRUCTURE_REDIRECT]
    /// is set.
    MapRequest(XMapRequestEvent<'a>),

    /// A client message has been received.
    ///
    /// Always generated.
    ClientMessage(XClientMessageEvent<'a>),

    /// The window has been mapped.
    ///
    /// Always generated.
    Mapping(XMappingEvent),

    /// The selection has been cleared.
    ///
    /// Always generated.
    SelectionClear(XSelectionClearEvent<'a>),

    /// The selection has been changed.
    ///
    /// Always generated.
    Selection(XSelectionEvent<'a>),

    /// A client message has been requested to change.
    ///
    /// Always generated.
    SelectionRequest(XSelectionRequestEvent<'a>),

    /// The window visibility has changed.
    ///
    /// Only generated when [`WindowInputMask::VISIBILITY_CHANGE`][crate::WindowInputMask::VISIBILITY_CHANGE]
    /// is set.
    VisibilityChange(XVisibilityEvent),

    /// The global cursor has changed.
    ///
    /// Only generated when [`CursorInputMask::CURSOR_NOTIFY`][crate::CursorInputMask::CURSOR_NOTIFY]
    /// is set.
    CursorChanged(XDisplayCursorEvent<'a>),

    /// The XInput2 hierarchy has changed.
    ///
    /// Only generated when [`XInputEventMask::HIERARCHY_CHANGED`][crate::XInputEventMask::HIERARCHY_CHANGED]
    /// is set.
    XIHierarchyChanged(XIHierarchyEvent<'a>),

    /// An XInput2 device has changed.
    ///
    /// Only generated when [`XInputEventMask::DEVICE_CHANGED`][crate::XInputEventMask::DEVICE_CHANGED]
    /// is set.
    XIDeviceChanged(XIDeviceChangedEvent<'a>),

    /// A key has been pressed.
    ///
    /// Only generated when [`XInputEventMask::KEY_PRESS`][crate::XInputEventMask::KEY_PRESS]
    /// is set.
    XIKeyPressed(XIDeviceEvent<'a>),

    /// A key has been released.
    ///
    /// Only generated when [`XInputEventMask::KEY_RELEASE][crate::XInputEventMask::KEY_RELEASE]
    /// is set.
    XIKeyReleased(XIDeviceEvent<'a>),

    /// A button has been pressed.
    ///
    /// Only generated when [`XInputEventMask::BUTTON_PRESS][crate::XInputEventMask::BUTTON_PRESS]
    /// is set.
    XIButtonPressed(XIDeviceEvent<'a>),

    /// A button has been released.
    ///
    /// Only generated when [`XInputEventMask::BUTTON_RELEASE][crate::XInputEventMask::BUTTON_RELEASE]
    /// is set.
    XIButtonReleased(XIDeviceEvent<'a>),

    /// A touch has begun.
    ///
    /// Only generated when [`XInputEventMask::TOUCH_BEGIN][crate::XInputEventMask::TOUCH_BEGIN]
    /// is set.
    XITouchBegin(XIDeviceEvent<'a>),

    /// A touch has ended.
    ///
    /// Only generated when [`XInputEventMask::TOUCH_END][crate::XInputEventMask::TOUCH_END]
    /// is set.
    XITouchEnd(XIDeviceEvent<'a>),

    /// A touch has updated.
    ///
    /// Only generated when [`XInputEventMask::TOUCH_UPDATE][crate::XInputEventMask::TOUCH_UPDATE]
    /// is set.
    XITouchUpdate(XIDeviceEvent<'a>),

    /// A touch ownership has changed.
    ///
    /// Only generated when [`XInputEventMask::TOUCH_OWNERSHIP_CHANGED][crate::XInputEventMask::TOUCH_OWNERSHIP_CHANGED]
    /// is set.
    XITouchOwnershipChanged(XITouchOwnershipEvent<'a>),

    /// A pointer has moved.
    ///
    /// Only generated when [`XInputEventMask::MOTION][crate::XInputEventMask::MOTION]
    /// is set.
    XIMotion(XIDeviceEvent<'a>),

    /// A raw key has been pressed.
    ///
    /// Only generated when [`XInputEventMask::RAW_KEY_PRESS`][crate::XInputEventMask::RAW_KEY_PRESS]
    /// is set.
    XIRawKeyPressed(XIRawEvent<'a>),

    /// A raw key has been released.
    ///
    /// Only generated when [`XInputEventMask::RAW_KEY_RELEASE`][crate::XInputEventMask::RAW_KEY_RELEASE]
    /// is set.
    XIRawKeyReleased(XIRawEvent<'a>),

    /// A raw button has been pressed.
    ///
    /// Only generated when [`XInputEventMask::RAW_BUTTON_PRESS`][crate::XInputEventMask::RAW_BUTTON_PRESS]
    /// is set.
    XIRawButtonPressed(XIRawEvent<'a>),

    /// A raw button has been released.
    ///
    /// Only generated when [`XInputEventMask::RAW_BUTTON_RELEASE`][crate::XInputEventMask::RAW_BUTTON_RELEASE]
    /// is set.
    XIRawButtonReleased(XIRawEvent<'a>),

    /// A raw touch has begun.
    ///
    /// Only generated when [`XInputEventMask::RAW_TOUCH_BEGIN`][crate::XInputEventMask::RAW_TOUCH_BEGIN]
    /// is set.
    XIRawTouchBegin(XIRawEvent<'a>),

    /// A raw touch has ended.
    ///
    /// Only generated when [`XInputEventMask::RAW_TOUCH_END`][crate::XInputEventMask::RAW_TOUCH_END]
    /// is set.
    XIRawTouchEnd(XIRawEvent<'a>),

    /// A raw touch has been updated.
    ///
    /// Only generated when [`XInputEventMask::RAW_TOUCH_UPDATE`][crate::XInputEventMask::RAW_TOUCH_UPDATE]
    /// is set.
    XIRawTouchUpdated(XIRawEvent<'a>),

    /// A raw motion has occurred.
    ///
    /// Only generated when [`XInputEventMask::RAW_MOTION`][crate::XInputEventMask::RAW_MOTION]
    /// is set.
    XIRawMotion(XIRawEvent<'a>),

    /// A pointer has hit a barrier.
    ///
    /// Only generated when [`XInputEventMask::BARRIER_HIT`][crate::XInputEventMask::BARRIER_HIT]
    /// is set.
    XIBarrierHit(XIBarrierEvent<'a>),

    /// A pointer has left a barrier.
    ///
    /// Only generated when [`XInputEventMask::BARRIER_LEAVE`][crate::XInputEventMask::BARRIER_LEAVE]
    /// is set.
    XIBarrierLeft(XIBarrierEvent<'a>),

    /// A device has entered.
    ///
    /// Only generated when [`XInputEventMask::ENTER`][crate::XInputEventMask::ENTER]
    /// is set.
    XIEntered(XIFocusEvent<'a>),

    /// A device has left.
    ///
    /// Only generated when [`XInputEventMask::LEAVE`][crate::XInputEventMask::LEAVE]
    /// is set.
    XILeft(XIFocusEvent<'a>),

    /// Something has been focused.
    ///
    /// Only generated when [`XInputEventMask::FOCUS_IN`][crate::XInputEventMask::FOCUS_IN]
    /// is set.
    XIFocusIn(XIFocusEvent<'a>),

    /// Something has been unfocused.
    ///
    /// Only generated when [`XInputEventMask::FOCUS_OUT`][crate::XInputEventMask::FOCUS_OUT]
    /// is set.
    XIFocusOut(XIFocusEvent<'a>),

    /// A device property changed.
    ///
    /// Only generated when [`XInputEventMask::PROPERTY_CHANGE`][crate::XInputEventMask::PROPERTY_CHANGE]
    /// is set.
    XIPropertyChanged(XIPropertyEvent<'a>),

    /// An unknown event has occurred, this may be sent by X extension and can be handled
    /// using the raw structure if desired.
    Unknown(xlib_sys::XEvent),
}

impl<'a> XEventData<'a> {
    /// Converts the X event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xlib_sys::XEvent, display: &'a XDisplay) -> Self {
        match event.type_ {
            xlib_sys::MotionNotify => Self::Motion(XMotionEvent::new(event.motion, display)),
            xlib_sys::ButtonPress => Self::ButtonPress(XButtonEvent::new(event.button, display)),
            xlib_sys::ButtonRelease => {
                Self::ButtonRelease(XButtonEvent::new(event.button, display))
            }
            xlib_sys::ColormapNotify => {
                Self::ColormapChange(XColormapEvent::new(event.colormap, display))
            }
            xlib_sys::EnterNotify => {
                Self::EnterWindow(XCrossingEvent::new(event.crossing, display))
            }
            xlib_sys::LeaveNotify => {
                Self::LeaveWindow(XCrossingEvent::new(event.crossing, display))
            }
            xlib_sys::Expose => Self::Expose(XExposeEvent::new(event.expose)),
            xlib_sys::FocusIn => Self::FocusIn(XFocusChangeEvent::new(event.focus_change)),
            xlib_sys::FocusOut => Self::FocusOut(XFocusChangeEvent::new(event.focus_change)),
            xlib_sys::KeymapNotify => Self::KeymapChange(XKeymapEvent::new(event.keymap)),
            xlib_sys::KeyPress => Self::KeyPress(XKeyEvent::new(event.key, display)),
            xlib_sys::KeyRelease => Self::KeyRelease(XKeyEvent::new(event.key, display)),
            xlib_sys::PropertyNotify => {
                Self::PropertyChange(XPropertyEvent::new(event.property, display))
            }
            xlib_sys::ResizeRequest => {
                Self::ResizeRequest(XResizeRequestEvent::new(event.resize_request))
            }
            xlib_sys::CirculateNotify => {
                Self::Circulate(XCirculateEvent::new(event.circulate, display))
            }
            xlib_sys::ConfigureNotify => {
                Self::Configure(XConfigureEvent::new(event.configure, display))
            }
            xlib_sys::DestroyNotify => {
                Self::Destroy(XDestroyWindowEvent::new(event.destroy_window, display))
            }
            xlib_sys::GravityNotify => Self::Gravity(XGravityEvent::new(event.gravity, display)),
            xlib_sys::MapNotify => Self::Map(XMapEvent::new(event.map, display)),
            xlib_sys::ReparentNotify => {
                Self::Reparent(XReparentEvent::new(event.reparent, display))
            }
            xlib_sys::UnmapNotify => Self::Unmap(XUnmapEvent::new(event.unmap, display)),
            xlib_sys::CirculateRequest => Self::CirculateRequest(XCirculateRequestEvent::new(
                event.circulate_request,
                display,
            )),
            xlib_sys::ConfigureRequest => Self::ConfigureRequest(XConfigureRequestEvent::new(
                event.configure_request,
                display,
            )),
            xlib_sys::MapRequest => {
                Self::MapRequest(XMapRequestEvent::new(event.map_request, display))
            }
            xlib_sys::ClientMessage => {
                Self::ClientMessage(XClientMessageEvent::new(event.client_message, display))
            }
            xlib_sys::MappingNotify => Self::Mapping(XMappingEvent::new(event.mapping)),
            xlib_sys::SelectionClear => {
                Self::SelectionClear(XSelectionClearEvent::new(event.selection_clear, display))
            }
            xlib_sys::SelectionNotify => {
                Self::Selection(XSelectionEvent::new(event.selection, display))
            }
            xlib_sys::SelectionRequest => Self::SelectionRequest(XSelectionRequestEvent::new(
                event.selection_request,
                display,
            )),
            xlib_sys::VisibilityNotify => {
                Self::VisibilityChange(XVisibilityEvent::new(event.visibility))
            }
            xlib_sys::GenericEvent => Self::new_generic(event, display),
            x if x == display.xfixes_event_base() + xfixes_sys::XFixesCursorNotify => {
                Self::CursorChanged(XDisplayCursorEvent::new(
                    event.xfixes_cursor_notify,
                    display,
                ))
            }
            _ => Self::Unknown(event),
        }
    }

    /// Converts the generic X event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The generic X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    unsafe fn new_generic(event: xlib_sys::XEvent, display: &'a XDisplay) -> Self {
        debug_assert_eq!(event.type_, xlib_sys::GenericEvent);

        if event.generic_event_cookie.extension == display.xinput2_opcode() {
            Self::new_xinput2(event, display)
        } else {
            Self::Unknown(event)
        }
    }

    /// Converts the xinput2 X event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The input2 X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    unsafe fn new_xinput2(mut event: xlib_sys::XEvent, display: &'a XDisplay) -> Self {
        let event_cookie = &mut event.generic_event_cookie;

        xlib_sys::XGetEventData(display.handle(), event_cookie);
        let event = match event_cookie.evtype {
            xinput2_sys::XI_DeviceChanged => {
                let event = *(event_cookie.data as *mut _);
                let converted = XIDeviceChangedEvent::new(event, display);
                xlib_sys::XFreeEventData(display.handle(), event_cookie);

                Self::XIDeviceChanged(converted)
            }
            xinput2_sys::XI_KeyPress => {
                let event = *(event_cookie.data as *mut _);
                let converted = XIDeviceEvent::new(event, display);
                xlib_sys::XFreeEventData(display.handle(), event_cookie);

                Self::XIKeyPressed(converted)
            }
            xinput2_sys::XI_KeyRelease => {
                let event = *(event_cookie.data as *mut _);
                let converted = XIDeviceEvent::new(event, display);
                xlib_sys::XFreeEventData(display.handle(), event_cookie);

                Self::XIKeyReleased(converted)
            }
            xinput2_sys::XI_ButtonPress => {
                let event = *(event_cookie.data as *mut _);
                let converted = XIDeviceEvent::new(event, display);
                xlib_sys::XFreeEventData(display.handle(), event_cookie);

                Self::XIButtonPressed(converted)
            }
            xinput2_sys::XI_ButtonRelease => {
                let event = *(event_cookie.data as *mut _);
                let converted = XIDeviceEvent::new(event, display);
                xlib_sys::XFreeEventData(display.handle(), event_cookie);

                Self::XIButtonReleased(converted)
            }
            xinput2_sys::XI_TouchBegin => {
                let event = *(event_cookie.data as *mut _);
                let converted = XIDeviceEvent::new(event, display);
                xlib_sys::XFreeEventData(display.handle(), event_cookie);

                Self::XITouchBegin(converted)
            }
            xinput2_sys::XI_TouchEnd => {
                let event = *(event_cookie.data as *mut _);
                let converted = XIDeviceEvent::new(event, display);
                xlib_sys::XFreeEventData(display.handle(), event_cookie);

                Self::XITouchEnd(converted)
            }
            xinput2_sys::XI_TouchUpdate => {
                let event = *(event_cookie.data as *mut _);
                let converted = XIDeviceEvent::new(event, display);
                xlib_sys::XFreeEventData(display.handle(), event_cookie);

                Self::XITouchUpdate(converted)
            }
            xinput2_sys::XI_TouchOwnership => {
                let event = *(event_cookie.data as *mut _);
                let converted = XITouchOwnershipEvent::new(event, display);
                xlib_sys::XFreeEventData(display.handle(), event_cookie);

                Self::XITouchOwnershipChanged(converted)
            }
            xinput2_sys::XI_Motion => {
                let event = *(event_cookie.data as *mut _);
                let converted = XIDeviceEvent::new(event, display);
                xlib_sys::XFreeEventData(display.handle(), event_cookie);

                Self::XIMotion(converted)
            }
            xinput2_sys::XI_HierarchyChanged => {
                let event = *(event_cookie.data as *mut _);
                let converted = XIHierarchyEvent::new(event, display);
                xlib_sys::XFreeEventData(display.handle(), event_cookie);

                Self::XIHierarchyChanged(converted)
            }
            xinput2_sys::XI_RawKeyPress => {
                let event = *(event_cookie.data as *mut _);
                let converted = XIRawEvent::new(event, display);
                xlib_sys::XFreeEventData(display.handle(), event_cookie);

                Self::XIRawKeyPressed(converted)
            }
            xinput2_sys::XI_RawKeyRelease => {
                let event = *(event_cookie.data as *mut _);
                let converted = XIRawEvent::new(event, display);
                xlib_sys::XFreeEventData(display.handle(), event_cookie);

                Self::XIRawKeyReleased(converted)
            }
            xinput2_sys::XI_RawButtonPress => {
                let event = *(event_cookie.data as *mut _);
                let converted = XIRawEvent::new(event, display);
                xlib_sys::XFreeEventData(display.handle(), event_cookie);

                Self::XIRawButtonPressed(converted)
            }
            xinput2_sys::XI_RawButtonRelease => {
                let event = *(event_cookie.data as *mut _);
                let converted = XIRawEvent::new(event, display);
                xlib_sys::XFreeEventData(display.handle(), event_cookie);

                Self::XIRawButtonReleased(converted)
            }
            xinput2_sys::XI_RawTouchBegin => {
                let event = *(event_cookie.data as *mut _);
                let converted = XIRawEvent::new(event, display);
                xlib_sys::XFreeEventData(display.handle(), event_cookie);

                Self::XIRawTouchBegin(converted)
            }
            xinput2_sys::XI_RawTouchEnd => {
                let event = *(event_cookie.data as *mut _);
                let converted = XIRawEvent::new(event, display);
                xlib_sys::XFreeEventData(display.handle(), event_cookie);

                Self::XIRawTouchEnd(converted)
            }
            xinput2_sys::XI_RawTouchUpdate => {
                let event = *(event_cookie.data as *mut _);
                let converted = XIRawEvent::new(event, display);
                xlib_sys::XFreeEventData(display.handle(), event_cookie);

                Self::XIRawTouchUpdated(converted)
            }
            xinput2_sys::XI_RawMotion => {
                let event = *(event_cookie.data as *mut _);
                let converted = XIRawEvent::new(event, display);
                xlib_sys::XFreeEventData(display.handle(), event_cookie);

                Self::XIRawMotion(converted)
            }
            xinput2_sys::XI_BarrierHit => {
                let event = *(event_cookie.data as *mut _);
                let converted = XIBarrierEvent::new(event, display);
                xlib_sys::XFreeEventData(display.handle(), event_cookie);

                Self::XIBarrierHit(converted)
            }
            xinput2_sys::XI_BarrierLeave => {
                let event = *(event_cookie.data as *mut _);
                let converted = XIBarrierEvent::new(event, display);
                xlib_sys::XFreeEventData(display.handle(), event_cookie);

                Self::XIBarrierLeft(converted)
            }
            xinput2_sys::XI_Enter => {
                let event = *(event_cookie.data as *mut _);
                let converted = XIFocusEvent::new(event, display);
                xlib_sys::XFreeEventData(display.handle(), event_cookie);

                Self::XIEntered(converted)
            }
            xinput2_sys::XI_Leave => {
                let event = *(event_cookie.data as *mut _);
                let converted = XIFocusEvent::new(event, display);
                xlib_sys::XFreeEventData(display.handle(), event_cookie);

                Self::XILeft(converted)
            }
            xinput2_sys::XI_FocusIn => {
                let event = *(event_cookie.data as *mut _);
                let converted = XIFocusEvent::new(event, display);
                xlib_sys::XFreeEventData(display.handle(), event_cookie);

                Self::XIFocusIn(converted)
            }
            xinput2_sys::XI_FocusOut => {
                let event = *(event_cookie.data as *mut _);
                let converted = XIFocusEvent::new(event, display);
                xlib_sys::XFreeEventData(display.handle(), event_cookie);

                Self::XIFocusOut(converted)
            }
            xinput2_sys::XI_PropertyEvent => {
                let event = *(event_cookie.data as *mut _);
                let converted = XIPropertyEvent::new(event, display);
                xlib_sys::XFreeEventData(display.handle(), event_cookie);

                Self::XIPropertyChanged(converted)
            }
            _ => {
                xlib_sys::XFreeEventData(display.handle(), event_cookie);
                Self::Unknown(event)
            }
        };

        event
    }
}

#[derive(Debug)]
pub struct XMotionEvent<'a> {
    root: XWindow<'a>,
    subwindow: XWindow<'a>,
    time: u64,
    x: i32,
    y: i32,
    x_root: i32,
    y_root: i32,
    state: InputModifierMask,
    is_hint: bool,
    same_screen: bool,
}

impl<'a> XMotionEvent<'a> {
    /// Converts the X motion event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xlib_sys::XMotionEvent, display: &'a XDisplay) -> Self {
        Self {
            root: XWindow::new(event.root, display, WindowHandleOwnership::Foreign),
            subwindow: XWindow::new(event.subwindow, display, WindowHandleOwnership::Foreign),
            time: event.time,
            x: event.x,
            y: event.y,
            x_root: event.x_root,
            y_root: event.y_root,
            state: InputModifierMask::from_bits_retain(event.state),
            is_hint: (event.is_hint as i32) == xlib_sys::NotifyHint,
            same_screen: event.same_screen != 0,
        }
    }

    /// Retrieves the root window this event occurred on.
    pub fn root(&self) -> &XWindow<'a> {
        &self.root
    }

    /// Retrieves the subwindow this event occurred on.
    pub fn subwindow(&self) -> &XWindow<'a> {
        &self.subwindow
    }

    /// Retrieves the timestamp this event occurred at.
    pub fn time(&self) -> u64 {
        self.time
    }

    /// Retrieves the x position the cursor is at now relative to the window
    /// that generated the event.
    pub fn x(&self) -> i32 {
        self.x
    }

    /// Retrieves the y position the cursor is at now relative to the window
    /// that generated the event.
    pub fn y(&self) -> i32 {
        self.y
    }

    /// Retrieves the x position the cursor is at now relative to the root window.
    pub fn root_x(&self) -> i32 {
        self.x_root
    }

    /// Retrieves the y position the cursor is at now relative to the root window.
    pub fn root_y(&self) -> i32 {
        self.y_root
    }

    /// Retrieves the state the cursor was in when the event occurred.
    pub fn state(&self) -> InputModifierMask {
        self.state
    }

    /// Determines whether this event is a hint.
    pub fn is_hint(&self) -> bool {
        self.is_hint
    }

    /// Determines whether the window the event occurred in and the root window were
    /// on the same screen.
    pub fn on_same_screen(&self) -> bool {
        self.same_screen
    }
}

#[derive(Debug)]
pub struct XButtonEvent<'a> {
    root: XWindow<'a>,
    subwindow: XWindow<'a>,
    time: u64,
    x: i32,
    y: i32,
    x_root: i32,
    y_root: i32,
    state: InputModifierMask,
    button: u32,
    same_screen: bool,
}

impl<'a> XButtonEvent<'a> {
    /// Converts the X button event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xlib_sys::XButtonEvent, display: &'a XDisplay) -> Self {
        Self {
            root: XWindow::new(event.root, display, WindowHandleOwnership::Foreign),
            subwindow: XWindow::new(event.subwindow, display, WindowHandleOwnership::Foreign),
            time: event.time,
            x: event.x,
            y: event.y,
            x_root: event.x_root,
            y_root: event.y_root,
            state: InputModifierMask::from_bits_retain(event.state),
            button: event.button,
            same_screen: event.same_screen != 0,
        }
    }

    /// Retrieves the root window this event occurred on.
    pub fn root(&self) -> &XWindow<'a> {
        &self.root
    }

    /// Retrieves the subwindow this event occurred on.
    pub fn subwindow(&self) -> &XWindow<'a> {
        &self.subwindow
    }

    /// Retrieves the timestamp this event occurred at.
    pub fn time(&self) -> u64 {
        self.time
    }

    /// Retrieves the x position the cursor is at now relative to the window
    /// that generated the event.
    pub fn x(&self) -> i32 {
        self.x
    }

    /// Retrieves the y position the cursor is at now relative to the window
    /// that generated the event.
    pub fn y(&self) -> i32 {
        self.y
    }

    /// Retrieves the x position the cursor is at now relative to the root window.
    pub fn root_x(&self) -> i32 {
        self.x_root
    }

    /// Retrieves the y position the cursor is at now relative to the root window.
    pub fn root_y(&self) -> i32 {
        self.y_root
    }

    /// Retrieves the state the cursor was in when the event occurred.
    pub fn state(&self) -> InputModifierMask {
        self.state
    }

    /// Retrieves the number of the button that triggered the event.
    pub fn button(&self) -> u32 {
        self.button
    }

    /// Determines whether the window the event occurred in and the root window were
    /// on the same screen.
    pub fn on_same_screen(&self) -> bool {
        self.same_screen
    }
}

#[derive(Debug)]
pub struct XKeyEvent<'a> {
    root: XWindow<'a>,
    subwindow: XWindow<'a>,
    time: u64,
    x: i32,
    y: i32,
    x_root: i32,
    y_root: i32,
    state: InputModifierMask,
    keycode: u32,
    same_screen: bool,
}

impl<'a> XKeyEvent<'a> {
    /// Converts the X key event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xlib_sys::XKeyEvent, display: &'a XDisplay) -> Self {
        Self {
            root: XWindow::new(event.root, display, WindowHandleOwnership::Foreign),
            subwindow: XWindow::new(event.subwindow, display, WindowHandleOwnership::Foreign),
            time: event.time,
            x: event.x,
            y: event.y,
            x_root: event.x_root,
            y_root: event.y_root,
            state: InputModifierMask::from_bits_retain(event.state),
            keycode: event.keycode,
            same_screen: event.same_screen != 0,
        }
    }

    /// Retrieves the root window this event occurred on.
    pub fn root(&self) -> &XWindow<'a> {
        &self.root
    }

    /// Retrieves the subwindow this event occurred on.
    pub fn subwindow(&self) -> &XWindow<'a> {
        &self.subwindow
    }

    /// Retrieves the timestamp this event occurred at.
    pub fn time(&self) -> u64 {
        self.time
    }

    /// Retrieves the x position the cursor is at now relative to the window
    /// that generated the event.
    pub fn x(&self) -> i32 {
        self.x
    }

    /// Retrieves the y position the cursor is at now relative to the window
    /// that generated the event.
    pub fn y(&self) -> i32 {
        self.y
    }

    /// Retrieves the x position the cursor is at now relative to the root window.
    pub fn root_x(&self) -> i32 {
        self.x_root
    }

    /// Retrieves the y position the cursor is at now relative to the root window.
    pub fn root_y(&self) -> i32 {
        self.y_root
    }

    /// Retrieves the state the cursor was in when the event occurred.
    pub fn state(&self) -> InputModifierMask {
        self.state
    }

    /// Retrieves the keycode of the key that triggered the event.
    pub fn keycode(&self) -> u32 {
        self.keycode
    }

    /// Determines whether the window the event occurred in and the root window were
    /// on the same screen.
    pub fn on_same_screen(&self) -> bool {
        self.same_screen
    }
}

#[derive(Debug)]
pub struct XColormapEvent<'a> {
    colormap: XColormap<'a>,
    new: bool,
    state: ColormapState,
}

impl<'a> XColormapEvent<'a> {
    /// Converts the X colormap event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xlib_sys::XColormapEvent, display: &'a XDisplay) -> Self {
        Self {
            colormap: XColormap::new(event.colormap, display, ColormapHandleOwnership::Foreign),
            new: event.new != 0,
            state: ColormapState::new(event.state),
        }
    }

    /// Retrieves the colormap that triggered this event.
    pub fn colormap(&self) -> &XColormap<'a> {
        &self.colormap
    }

    /// Determines whether the color is a new colormap.
    pub fn is_new(&self) -> bool {
        self.new
    }

    /// Retrieves the state of the colormap.
    pub fn state(&self) -> ColormapState {
        self.state
    }
}

#[derive(Debug)]
pub struct XCrossingEvent<'a> {
    root: XWindow<'a>,
    subwindow: XWindow<'a>,
    time: u64,
    x: i32,
    y: i32,
    x_root: i32,
    y_root: i32,
    detail: NotifyDetail,
    same_screen: bool,
    focus: bool,
    state: InputModifierMask,
}

impl<'a> XCrossingEvent<'a> {
    /// Converts the X crossing event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xlib_sys::XCrossingEvent, display: &'a XDisplay) -> Self {
        Self {
            root: XWindow::new(event.root, display, WindowHandleOwnership::Foreign),
            subwindow: XWindow::new(event.subwindow, display, WindowHandleOwnership::Foreign),
            time: event.time,
            x: event.x,
            y: event.y,
            x_root: event.x_root,
            y_root: event.y_root,
            detail: NotifyDetail::new(event.detail),
            same_screen: event.same_screen != 0,
            focus: event.focus != 0,
            state: InputModifierMask::from_bits_retain(event.state),
        }
    }

    /// Retrieves the root window this event occurred on.
    pub fn root(&self) -> &XWindow<'a> {
        &self.root
    }

    /// Retrieves the subwindow this event occurred on.
    pub fn subwindow(&self) -> &XWindow<'a> {
        &self.subwindow
    }

    /// Retrieves the timestamp this event occurred at.
    pub fn time(&self) -> u64 {
        self.time
    }

    /// Retrieves the x position the cursor is at now relative to the window
    /// that generated the event.
    pub fn x(&self) -> i32 {
        self.x
    }

    /// Retrieves the y position the cursor is at now relative to the window
    /// that generated the event.
    pub fn y(&self) -> i32 {
        self.y
    }

    /// Retrieves the x position the cursor is at now relative to the root window.
    pub fn root_x(&self) -> i32 {
        self.x_root
    }

    /// Retrieves the y position the cursor is at now relative to the root window.
    pub fn root_y(&self) -> i32 {
        self.y_root
    }

    /// Retrieves the detail of this notification.
    pub fn detail(&self) -> NotifyDetail {
        self.detail
    }

    /// Determines whether the window has focus.
    pub fn has_focus(&self) -> bool {
        self.focus
    }

    /// Retrieves the state the cursor was in when the event occurred.
    pub fn state(&self) -> InputModifierMask {
        self.state
    }

    /// Determines whether the window the event occurred in and the root window were
    /// on the same screen.
    pub fn on_same_screen(&self) -> bool {
        self.same_screen
    }
}

#[derive(Debug)]
pub struct XExposeEvent {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    count: i32,
}

impl XExposeEvent {
    /// Converts the X expose event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    pub fn new(event: xlib_sys::XExposeEvent) -> Self {
        Self {
            x: event.x,
            y: event.y,
            width: event.width,
            height: event.height,
            count: event.count,
        }
    }

    /// Retrieves the x coordinate inside the window where the exposure happened.
    pub fn x(&self) -> i32 {
        self.x
    }

    /// Retrieves the y coordinate inside the window where the exposure happened.
    pub fn y(&self) -> i32 {
        self.y
    }

    /// Retrieves the width of the exposed rectangle.
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Retrieves the height of the exposed rectangle.
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Retrieves the amount of expose events following this event.
    ///
    /// When it is desired to always redraw the full window, applications should ignore
    /// any event with a counter greater than 0 and redraw the entire window when the
    /// count reaches 0.
    pub fn count(&self) -> i32 {
        self.count
    }
}

#[derive(Debug)]
pub struct XFocusChangeEvent {
    mode: NotifyMode,
    detail: NotifyDetail,
}

impl XFocusChangeEvent {
    /// Converts the X focus change event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xlib_sys::XFocusChangeEvent) -> Self {
        Self {
            mode: NotifyMode::new(event.mode),
            detail: NotifyDetail::new(event.detail),
        }
    }

    /// Retrieves the mode of this notification.
    pub fn mode(&self) -> NotifyMode {
        self.mode
    }

    /// Retrieves the detail of this notification.
    pub fn detail(&self) -> NotifyDetail {
        self.detail
    }
}

#[derive(Debug)]
pub struct XKeymapEvent {
    key_vector: [char; 32],
}

impl XKeymapEvent {
    /// Converts the X keymap event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    pub fn new(event: xlib_sys::XKeymapEvent) -> Self {
        let mut key_vector = ['\0'; 32];

        for (i, key) in event.key_vector.iter().enumerate() {
            key_vector[i] = (*key as u8) as _;
        }

        Self { key_vector }
    }

    /// Retrieves the new key vector of the keymap.
    pub fn key_vector(&self) -> [char; 32] {
        self.key_vector
    }
}

#[derive(Debug)]
pub struct XPropertyEvent<'a> {
    atom: XAtom<'a>,
    time: u64,
    state: PropertyState,
}

impl<'a> XPropertyEvent<'a> {
    /// Converts the X property event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xlib_sys::XPropertyEvent, display: &'a XDisplay) -> Self {
        Self {
            atom: XAtom::new(event.atom, display),
            time: event.time,
            state: PropertyState::new(event.state),
        }
    }

    /// Retrieves the atom identifying the property.
    pub fn atom(&self) -> XAtom<'a> {
        self.atom
    }

    /// Retrieves the timestamp the event occurred at
    pub fn time(&self) -> u64 {
        self.time
    }

    /// Retrieves the new state of the property.
    pub fn state(&self) -> PropertyState {
        self.state
    }
}

#[derive(Debug)]
pub struct XResizeRequestEvent {
    width: i32,
    height: i32,
}

impl XResizeRequestEvent {
    /// Converts the X resize request event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    pub fn new(event: xlib_sys::XResizeRequestEvent) -> Self {
        Self {
            width: event.width,
            height: event.height,
        }
    }

    /// Retrieves the requested width.
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Retrieves the requested height.
    pub fn height(&self) -> i32 {
        self.height
    }
}

#[derive(Debug)]
pub struct XCirculateEvent<'a> {
    window: XWindow<'a>,
    place: CirculatePlace,
}

impl<'a> XCirculateEvent<'a> {
    /// Converts the X circulate event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xlib_sys::XCirculateEvent, display: &'a XDisplay) -> Self {
        Self {
            window: XWindow::new(event.window, display, WindowHandleOwnership::Foreign),
            place: CirculatePlace::new(event.place),
        }
    }

    /// Retrieves the window which should be circulated.
    pub fn window(&self) -> &XWindow<'a> {
        &self.window
    }

    /// Retrieves the desired new place of the window.
    pub fn place(&self) -> CirculatePlace {
        self.place
    }
}

#[derive(Debug)]
pub struct XConfigureEvent<'a> {
    window: XWindow<'a>,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    border_width: i32,
    above: Option<XWindow<'a>>,
    override_redirect: bool,
}

impl<'a> XConfigureEvent<'a> {
    /// Converts the X configure event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xlib_sys::XConfigureEvent, display: &'a XDisplay) -> Self {
        Self {
            window: XWindow::new(event.window, display, WindowHandleOwnership::Foreign),
            x: event.x,
            y: event.y,
            width: event.width,
            height: event.height,
            border_width: event.border_width,
            above: if event.above == 0 {
                None
            } else {
                Some(XWindow::new(
                    event.above,
                    display,
                    WindowHandleOwnership::Foreign,
                ))
            },
            override_redirect: event.override_redirect != 0,
        }
    }

    /// Retrieves the window which was configured.
    pub fn window(&self) -> &XWindow<'a> {
        &self.window
    }

    /// Retrieves the new x coordinate of the window.
    pub fn x(&self) -> i32 {
        self.x
    }

    /// Retrieves the new y coordinate of the window.
    pub fn y(&self) -> i32 {
        self.y
    }

    /// Retrieves the new width of the window.
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Retrieves the new height of the window.
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Retrieves the new border width of the window.
    pub fn border_width(&self) -> i32 {
        self.border_width
    }

    /// Retrieves the sibling window for stacking operations, if any.
    ///
    /// This will be [`None`] if the window is at the bottom of the stack.
    pub fn above(&self) -> Option<&XWindow<'a>> {
        self.above.as_ref()
    }

    /// Determines whether override redirect is enabled for this window.
    pub fn override_redirect(&self) -> bool {
        self.override_redirect
    }
}

#[derive(Debug)]
pub struct XDestroyWindowEvent<'a> {
    window: XWindow<'a>,
}

impl<'a> XDestroyWindowEvent<'a> {
    /// Converts the X destroy window event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xlib_sys::XDestroyWindowEvent, display: &'a XDisplay) -> Self {
        Self {
            window: XWindow::new(event.window, display, WindowHandleOwnership::Foreign),
        }
    }

    /// Retrieves the window that has been destroyed.
    pub fn window(&self) -> &XWindow<'a> {
        &self.window
    }
}

#[derive(Debug)]
pub struct XGravityEvent<'a> {
    window: XWindow<'a>,
    x: i32,
    y: i32,
}

impl<'a> XGravityEvent<'a> {
    /// Converts the X gravity event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xlib_sys::XGravityEvent, display: &'a XDisplay) -> Self {
        Self {
            window: XWindow::new(event.window, display, WindowHandleOwnership::Foreign),
            x: event.x,
            y: event.y,
        }
    }

    /// Retrieves the window which position has changed.
    pub fn window(&self) -> &XWindow<'a> {
        &self.window
    }

    /// Retrieves the new x coordinate of the window.
    pub fn x(&self) -> i32 {
        self.x
    }

    /// Retrieves the new y coordinate of the window.
    pub fn y(&self) -> i32 {
        self.y
    }
}

#[derive(Debug)]
pub struct XMapEvent<'a> {
    window: XWindow<'a>,
    override_redirect: bool,
}

impl<'a> XMapEvent<'a> {
    /// Converts the X map event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xlib_sys::XMapEvent, display: &'a XDisplay) -> Self {
        Self {
            window: XWindow::new(event.window, display, WindowHandleOwnership::Foreign),
            override_redirect: event.override_redirect != 0,
        }
    }

    /// Retrieves the window which has been mapped.
    pub fn window(&self) -> &XWindow<'a> {
        &self.window
    }

    /// Determines whether override redirect is enabled for this window.
    pub fn override_redirect(&self) -> bool {
        self.override_redirect
    }
}

#[derive(Debug)]
pub struct XReparentEvent<'a> {
    window: XWindow<'a>,
    parent: XWindow<'a>,
    x: i32,
    y: i32,
    override_redirect: bool,
}

impl<'a> XReparentEvent<'a> {
    /// Converts the X reparent event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xlib_sys::XReparentEvent, display: &'a XDisplay) -> Self {
        Self {
            window: XWindow::new(event.window, display, WindowHandleOwnership::Foreign),
            parent: XWindow::new(event.parent, display, WindowHandleOwnership::Foreign),
            x: event.x,
            y: event.y,
            override_redirect: event.override_redirect != 0,
        }
    }

    /// Retrieves the window which has been reparented.
    pub fn window(&self) -> &XWindow<'a> {
        &self.window
    }

    /// Retrieves the new parent of the window.
    pub fn parent(&self) -> &XWindow<'a> {
        &self.parent
    }

    /// Retrieves the new x coordinate of the window relative to the parent.
    pub fn x(&self) -> i32 {
        self.x
    }

    /// Retrieves the new y coordinate of the window relative to the parent.
    pub fn y(&self) -> i32 {
        self.y
    }

    /// Determines whether override redirect is enabled for this window.
    pub fn override_redirect(&self) -> bool {
        self.override_redirect
    }
}

#[derive(Debug)]
pub struct XUnmapEvent<'a> {
    window: XWindow<'a>,
    from_configure: bool,
}

impl<'a> XUnmapEvent<'a> {
    /// Converts the X unmap event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xlib_sys::XUnmapEvent, display: &'a XDisplay) -> Self {
        Self {
            window: XWindow::new(event.window, display, WindowHandleOwnership::Foreign),
            from_configure: event.from_configure != 0,
        }
    }

    /// Retrieves the window which has been unmapped.
    pub fn window(&self) -> &XWindow<'a> {
        &self.window
    }

    /// Determines whether this window has been unmapped by a configure call.
    pub fn is_from_configure(&self) -> bool {
        self.from_configure
    }
}

#[derive(Debug)]
pub struct XCirculateRequestEvent<'a> {
    window: XWindow<'a>,
    place: CirculatePlace,
}

impl<'a> XCirculateRequestEvent<'a> {
    /// Converts the X circulate request event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xlib_sys::XCirculateRequestEvent, display: &'a XDisplay) -> Self {
        Self {
            window: XWindow::new(event.window, display, WindowHandleOwnership::Foreign),
            place: CirculatePlace::new(event.place),
        }
    }

    /// Retrieves the window which should be circulated.
    pub fn window(&self) -> &XWindow<'a> {
        &self.window
    }

    /// Retrieves the desired new circulation place.
    pub fn place(&self) -> CirculatePlace {
        self.place
    }
}

#[derive(Debug)]
pub struct XConfigureRequestEvent<'a> {
    window: XWindow<'a>,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    border_width: i32,
    above: XWindow<'a>,
    detail: ConfigureDetail,
    value_mask: u64,
}

impl<'a> XConfigureRequestEvent<'a> {
    /// Converts the X configure request event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xlib_sys::XConfigureRequestEvent, display: &'a XDisplay) -> Self {
        Self {
            window: XWindow::new(event.window, display, WindowHandleOwnership::Foreign),
            x: event.x,
            y: event.y,
            width: event.width,
            height: event.height,
            border_width: event.border_width,
            above: XWindow::new(event.above, display, WindowHandleOwnership::Foreign),
            detail: ConfigureDetail::new(event.detail),
            value_mask: event.value_mask,
        }
    }

    /// Retrieves the window which should be configured.
    pub fn window(&self) -> &XWindow<'a> {
        &self.window
    }

    /// Retrieves the new desired x position of the window.
    pub fn x(&self) -> i32 {
        self.x
    }

    /// Retrieves the new desired y coordinate of the window.
    pub fn y(&self) -> i32 {
        self.y
    }

    /// Retrieves the new desired width of the window.
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Retrieves the new desired height of the window.
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Retrieves the new desired border width of the window.
    pub fn border_width(&self) -> i32 {
        self.border_width
    }

    /// Retrieves the window which this window should be placed above.
    pub fn above(&self) -> &XWindow<'a> {
        &self.above
    }

    /// Determines the detail of this request.
    pub fn detail(&self) -> ConfigureDetail {
        self.detail
    }

    /// Determines the mask of the fields which were set in the initial request.
    ///
    /// All other value were filled in automatically.
    pub fn value_mask(&self) -> u64 {
        self.value_mask
    }
}

#[derive(Debug)]
pub struct XMapRequestEvent<'a> {
    window: XWindow<'a>,
}

impl<'a> XMapRequestEvent<'a> {
    /// Converts the X map request event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xlib_sys::XMapRequestEvent, display: &'a XDisplay) -> Self {
        Self {
            window: XWindow::new(event.window, display, WindowHandleOwnership::Foreign),
        }
    }

    /// Retrieves the window which should be mapped.
    pub fn window(&self) -> &XWindow<'a> {
        &self.window
    }
}

#[derive(Debug)]
pub struct XClientMessageEvent<'a> {
    message_type: XAtom<'a>,
    data: ClientMessageData,
}

impl<'a> XClientMessageEvent<'a> {
    /// Converts the X client message event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xlib_sys::XClientMessageEvent, display: &'a XDisplay) -> Self {
        let data = match event.format {
            8 => {
                let mut data = [0; 20];

                for i in 0..20 {
                    data[i] = event.data.get_byte(i) as i8;
                }

                ClientMessageData::Bit8(data)
            }
            16 => {
                let mut data = [0; 10];

                for i in 0..10 {
                    data[i] = event.data.get_short(i) as i16;
                }

                ClientMessageData::Bit16(data)
            }
            32 => {
                let mut data = [0; 5];

                for i in 0..5 {
                    data[i] = event.data.get_long(i) as i32;
                }

                ClientMessageData::Bit32(data)
            }
            x => unreachable!("Invalid X message data format: {}", x),
        };

        Self {
            message_type: XAtom::new(event.message_type, display),
            data,
        }
    }

    /// Retrieves the atom identifying the type of this message.
    ///
    /// This is an application defined value.
    pub fn message_type(&self) -> XAtom<'a> {
        self.message_type
    }

    /// Retrieves the data of this message.
    pub fn data(&self) -> ClientMessageData {
        self.data
    }
}

#[derive(Debug)]
pub struct XMappingEvent {
    request: MappingRequestType,
    first_keycode: i32,
    count: i32,
}

impl XMappingEvent {
    /// Converts the X mapping event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    pub fn new(event: xlib_sys::XMappingEvent) -> Self {
        Self {
            request: MappingRequestType::new(event.request),
            first_keycode: event.first_keycode,
            count: event.count,
        }
    }

    /// Determines the type of this request.
    pub fn request_type(&self) -> MappingRequestType {
        self.request
    }

    /// Determines the first keycode of the mapping request.
    pub fn first_keycode(&self) -> i32 {
        self.first_keycode
    }

    /// Determines the count of keycodes in the mapping request.
    pub fn count(&self) -> i32 {
        self.count
    }
}

#[derive(Debug)]
pub struct XSelectionClearEvent<'a> {
    selection: XAtom<'a>,
    time: u64,
}

impl<'a> XSelectionClearEvent<'a> {
    /// Converts the X selection clear event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xlib_sys::XSelectionClearEvent, display: &'a XDisplay) -> Self {
        Self {
            selection: XAtom::new(event.selection, display),
            time: event.time,
        }
    }

    /// Retrieves the selection that was cleared.
    pub fn selection(&self) -> XAtom<'a> {
        self.selection
    }

    /// Retrieves the timestamp at which this event occurred.
    pub fn time(&self) -> u64 {
        self.time
    }
}

#[derive(Debug)]
pub struct XSelectionEvent<'a> {
    selection: XAtom<'a>,
    target: XAtom<'a>,
    property: Option<XAtom<'a>>,
    time: u64,
}

impl<'a> XSelectionEvent<'a> {
    /// Converts the X selection event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xlib_sys::XSelectionEvent, display: &'a XDisplay) -> Self {
        Self {
            selection: XAtom::new(event.selection, display),
            target: XAtom::new(event.target, display),
            property: if event.property == 0 {
                None
            } else {
                Some(XAtom::new(event.property, display))
            },
            time: event.time,
        }
    }

    /// Retrieves the selection that changed.
    pub fn selection(&self) -> XAtom<'a> {
        self.selection
    }

    /// Retrieves the selection target.
    pub fn target(&self) -> XAtom<'a> {
        self.target
    }

    /// Retrieves the selection property, if any.
    pub fn property(&self) -> Option<XAtom<'a>> {
        self.property
    }

    /// Retrieves the timestamp at which this event occurred.
    pub fn time(&self) -> u64 {
        self.time
    }
}

#[derive(Debug)]
pub struct XSelectionRequestEvent<'a> {
    requestor: XWindow<'a>,
    selection: XAtom<'a>,
    target: XAtom<'a>,
    property: XAtom<'a>,
    time: u64,
}

impl<'a> XSelectionRequestEvent<'a> {
    /// Converts the X selection request event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xlib_sys::XSelectionRequestEvent, display: &'a XDisplay) -> Self {
        Self {
            requestor: XWindow::new(event.requestor, display, WindowHandleOwnership::Foreign),
            selection: XAtom::new(event.selection, display),
            target: XAtom::new(event.target, display),
            property: XAtom::new(event.property, display),
            time: event.time,
        }
    }

    /// Retrieves the window which issued this request.
    pub fn requestor(&self) -> &XWindow<'a> {
        &self.requestor
    }

    /// Retrieves the selection to be changed.
    pub fn selection(&self) -> XAtom<'a> {
        self.selection
    }

    /// Retrieves the target of the selection.
    pub fn target(&self) -> XAtom<'a> {
        self.target
    }

    /// Retrieves the property of the selection.
    pub fn property(&self) -> XAtom<'a> {
        self.property
    }

    /// Retrieves the timestamp this event occurred at.
    pub fn time(&self) -> u64 {
        self.time
    }
}

#[derive(Debug)]
pub struct XVisibilityEvent {
    state: VisibilityState,
}

impl XVisibilityEvent {
    /// Converts the X visibility event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    pub fn new(event: xlib_sys::XVisibilityEvent) -> Self {
        Self {
            state: VisibilityState::new(event.state),
        }
    }

    /// Retrieves the new visibility of the window.
    pub fn state(&self) -> VisibilityState {
        self.state
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum XDisplayCursorEventSubtype {
    CursorNotify,
}

impl XDisplayCursorEventSubtype {
    /// Wraps an existing X11 cursor event subtype.
    ///
    /// # Arguments
    ///
    /// * `subtype` - The native X11 cursor even subtype to wrap
    pub fn new(subtype: i32) -> Self {
        match subtype {
            xfixes_sys::XFixesDisplayCursorNotify => Self::CursorNotify,
            x => unreachable!("Invalid X cursor event subtype: {}", x),
        }
    }
}

#[derive(Debug)]
pub struct XDisplayCursorEvent<'a> {
    subtype: XDisplayCursorEventSubtype,
    cursor_serial: u64,
    timestamp: u64,
    cursor_name: XAtom<'a>,
}

impl<'a> XDisplayCursorEvent<'a> {
    /// Converts the X cursor notify event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xfixes_sys::XFixesCursorNotifyEvent, display: &'a XDisplay) -> Self {
        Self {
            subtype: XDisplayCursorEventSubtype::new(event.subtype),
            cursor_serial: event.cursor_serial as _,
            timestamp: event.timestamp as _,
            cursor_name: XAtom::new(event.cursor_name, display),
        }
    }

    /// Retrieves the subtype of the event.
    pub fn subtype(&self) -> XDisplayCursorEventSubtype {
        self.subtype
    }

    /// Retrieves the serial of the cursor.
    pub fn cursor_serial(&self) -> u64 {
        self.cursor_serial
    }

    /// Retrieves the timestamp this event occurred at.
    pub fn time(&self) -> u64 {
        self.timestamp
    }

    /// Retrieves the name of the cursor that changed.
    pub fn cursor_name(&self) -> XAtom<'a> {
        self.cursor_name
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct XIHierarchyChangeFlags: i32 {
        /// A master device has been added.
        const MASTER_ADDED = xinput2_sys::XIMasterAdded;

        /// A master device has been removed.
        const MASTER_REMOVED = xinput2_sys::XIMasterRemoved;

        /// A slave device has been added.
        const SLAVE_ADDED = xinput2_sys::XISlaveAdded;

        /// A slave device has been removed.
        const SLAVE_REMOVED = xinput2_sys::XISlaveRemoved;

        /// A slave device has been attached.
        const SLAVE_ATTACHED = xinput2_sys::XISlaveAttached;

        /// A slave device has been detached.
        const SLAVE_DETACHED = xinput2_sys::XISlaveDetached;

        /// A device has been enabled.
        const DEVICE_ENABLED = xinput2_sys::XIDeviceEnabled;

        /// A device has been disabled.
        const DEVICE_DISABLED = xinput2_sys::XIDeviceDisabled;
    }
}

#[derive(Debug)]
pub struct XIHierarchyInfo<'a> {
    device: XInputDevice<'a>,
    attachment: i32,
    usage: i32,
    enabled: bool,
    flags: XIHierarchyChangeFlags,
}

impl<'a> XIHierarchyInfo<'a> {
    /// Wraps an existing XInput2 hierarchy info.
    ///
    /// # Arguments
    ///
    /// * `handle` - The XInput2 native info
    /// * `display` - The display the info belongs to
    pub fn new(handle: xinput2_sys::XIHierarchyInfo, display: &'a XDisplay) -> Self {
        Self {
            device: XInputDevice::from_id(handle.deviceid, display),
            attachment: handle.attachment,
            usage: handle._use,
            enabled: handle.enabled != 0,
            flags: XIHierarchyChangeFlags::from_bits_retain(handle.flags),
        }
    }

    /// Retrieves the device this info applies to.
    pub fn device(&self) -> &XInputDevice<'a> {
        &self.device
    }

    /// Retrieves the attachment of this info
    pub fn attachment(&self) -> i32 {
        self.attachment
    }

    /// Retrieves the usage of this info
    pub fn usage(&self) -> i32 {
        self.usage
    }

    /// Determines whether the device is enabled.
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// Retrieves the flags of this info.
    pub fn flags(&self) -> XIHierarchyChangeFlags {
        self.flags
    }
}

#[derive(Debug, Clone)]
pub struct XIButtonState {
    mask: Vec<u8>,
}

impl XIButtonState {
    /// Creates a new XInput2 button state.
    ///
    /// # Arguments
    ///
    /// * `mask` - The mask of the state
    pub fn new(mask: Vec<u8>) -> Self {
        Self { mask }
    }

    /// Retrieves the mask of this state
    pub fn mask(&self) -> &[u8] {
        &self.mask
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum XIValuatorMode {
    Absolute = xinput2_sys::XIModeAbsolute,
    Relative = xinput2_sys::XIModeRelative,
}

impl XIValuatorMode {
    /// Wraps an existing XInput2 valuator mode.
    ///
    /// # Arguments
    ///
    /// * `mode` - The XInput2 valuator mode
    ///
    /// # Panics
    ///
    /// If the given valuator mode is not valid.
    pub fn new(mode: i32) -> Self {
        match mode {
            xinput2_sys::XIModeAbsolute => Self::Absolute,
            xinput2_sys::XIModeRelative => Self::Relative,
            x => unreachable!("Unknown XI valuator mode {}", x),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum XIScrollType {
    Vertical = xinput2_sys::XIScrollTypeVertical,
    Horizontal = xinput2_sys::XIScrollTypeHorizontal,
}

impl XIScrollType {
    /// Wraps an existing XInput2 scroll type.
    ///
    /// # Arguments
    ///
    /// * `ty` - The XInput2 scroll type
    ///
    /// # Panics
    ///
    /// If the given scroll type is not valid.
    pub fn new(ty: i32) -> Self {
        match ty {
            xinput2_sys::XIScrollTypeVertical => Self::Vertical,
            xinput2_sys::XIScrollTypeHorizontal => Self::Horizontal,
            x => unreachable!("Unknown XI scroll type {}", x),
        }
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct XIScrollFlags: i32 {
        /// No legacy events will be emulated.
        const NO_EMULATION = xinput2_sys::XIScrollFlagNoEmulation;

        /// For legacy events, this is the preferred axis.
        const PREFERRED = xinput2_sys::XIScrollFlagPreferred;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum XITouchMode {
    Direct = xinput2_sys::XIDirectTouch,
    Dependent = xinput2_sys::XIDependentTouch,
}

impl XITouchMode {
    /// Wraps an existing XInput2 touch mode.
    ///
    /// # Arguments
    ///
    /// * `mode` - The XInput2 touch mode
    ///
    /// # Panics
    ///
    /// If the given touch mode is not valid.
    pub fn new(mode: i32) -> Self {
        match mode {
            xinput2_sys::XIDirectTouch => Self::Direct,
            xinput2_sys::XIDependentTouch => Self::Dependent,
            x => unreachable!("Unknown XI touch mode {}", x),
        }
    }
}

#[derive(Debug, Clone)]
pub enum XIClassInfo<'a> {
    Button {
        source: XInputDevice<'a>,
        labels: Vec<XAtom<'a>>,
        state: XIButtonState,
    },
    Key {
        source: XInputDevice<'a>,
        key_codes: Vec<i32>,
    },
    Valuator {
        source: XInputDevice<'a>,
        number: i32,
        label: Option<XAtom<'a>>,
        min: f64,
        max: f64,
        value: f64,
        resolution: i32,
        mode: XIValuatorMode,
    },
    Scroll {
        source: XInputDevice<'a>,
        number: i32,
        ty: XIScrollType,
        increment: f64,
        flags: XIScrollFlags,
    },
    Touch {
        source: XInputDevice<'a>,
        mode: XITouchMode,
        num_touches: u32,
    },
}

impl<'a> XIClassInfo<'a> {
    /// Wraps an existing XInput2 class info.
    ///
    /// # Arguments
    ///
    /// * `handle` - The XInput2 native info
    /// * `display` - The display the info belongs to
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure all arguments are valid.
    pub unsafe fn new(handle: *const xinput2_sys::XIAnyClassInfo, display: &'a XDisplay) -> Self {
        let handle = &*handle;

        match handle._type {
            xinput2_sys::XIKeyClass => {
                let handle: &xinput2_sys::XIKeyClassInfo = std::mem::transmute(handle);

                let key_codes =
                    std::slice::from_raw_parts(handle.keycodes, handle.num_keycodes as _);

                Self::Key {
                    source: XInputDevice::from_id(handle.sourceid, display),
                    key_codes: key_codes.to_vec(),
                }
            }
            xinput2_sys::XIButtonClass => {
                let handle: &xinput2_sys::XIButtonClassInfo = std::mem::transmute(handle);

                let labels = std::slice::from_raw_parts(handle.labels, handle.num_buttons as _);
                let state =
                    std::slice::from_raw_parts(handle.state.mask, handle.state.mask_len as _);

                Self::Button {
                    source: XInputDevice::from_id(handle.sourceid, display),
                    labels: labels.iter().map(|a| XAtom::new(*a, display)).collect(),
                    state: XIButtonState::new(state.to_vec()),
                }
            }
            xinput2_sys::XIValuatorClass => {
                let handle: &xinput2_sys::XIValuatorClassInfo = std::mem::transmute(handle);

                Self::Valuator {
                    source: XInputDevice::from_id(handle.sourceid, display),
                    number: handle.number,
                    label: if handle.label == 0 {
                        None
                    } else {
                        Some(XAtom::new(handle.label, display))
                    },
                    min: handle.min,
                    max: handle.max,
                    value: handle.value,
                    resolution: handle.resolution,
                    mode: XIValuatorMode::new(handle.mode),
                }
            }
            xinput2_sys::XIScrollClass => {
                let handle: &xinput2_sys::XIScrollClassInfo = std::mem::transmute(handle);

                Self::Scroll {
                    source: XInputDevice::from_id(handle.sourceid, display),
                    number: handle.number,
                    ty: XIScrollType::new(handle.scroll_type),
                    increment: handle.increment,
                    flags: XIScrollFlags::from_bits_retain(handle.flags),
                }
            }
            xinput2_sys::XITouchClass => {
                let handle: &xinput2_sys::XITouchClassInfo = std::mem::transmute(handle);

                Self::Touch {
                    source: XInputDevice::from_id(handle.sourceid, display),
                    mode: XITouchMode::new(handle.mode),
                    num_touches: handle.num_touches as _,
                }
            }
            x => unreachable!("Unknown XI class type {}", x),
        }
    }
}

#[derive(Debug)]
pub struct XIHierarchyEvent<'a> {
    time: u64,
    flags: XIHierarchyChangeFlags,
    info: Vec<XIHierarchyInfo<'a>>,
}

impl<'a> XIHierarchyEvent<'a> {
    /// Converts the XInput2 hierarchy event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xinput2_sys::XIHierarchyEvent, display: &'a XDisplay) -> Self {
        let info = std::slice::from_raw_parts(event.info, event.num_info as _);

        Self {
            time: event.time,
            flags: XIHierarchyChangeFlags::from_bits_retain(event.flags),
            info: info
                .iter()
                .map(|i| XIHierarchyInfo::new(*i, display))
                .collect(),
        }
    }

    /// Retrieves the timestamp this event occurred at.
    pub fn time(&self) -> u64 {
        self.time
    }

    /// Retrieves the flags of this event.
    pub fn flags(&self) -> XIHierarchyChangeFlags {
        self.flags
    }

    /// Retrieves additional hierarchy info of this event.
    pub fn info(&self) -> &[XIHierarchyInfo] {
        &self.info
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum XIDeviceChangeReason {
    SlaveSwitch = xinput2_sys::XISlaveSwitch,
    DeviceChange = xinput2_sys::XIDeviceChange,
}

impl XIDeviceChangeReason {
    /// Wraps an existing XInput2 device change reason.
    ///
    /// # Arguments
    ///
    /// * `reason` - The XInput2 device change reason
    ///
    /// # Panics
    ///
    /// If the given device change reason is not valid.
    pub fn new(reason: i32) -> Self {
        match reason {
            xinput2_sys::XISlaveSwitch => Self::SlaveSwitch,
            xinput2_sys::XIDeviceChange => Self::DeviceChange,
            x => unreachable!("Unknown XI device change reason {}", x),
        }
    }
}

#[derive(Debug)]
pub struct XIDeviceChangedEvent<'a> {
    time: u64,
    device: XInputDevice<'a>,
    source: XInputDevice<'a>,
    reason: XIDeviceChangeReason,
    classes: Vec<XIClassInfo<'a>>,
}

impl<'a> XIDeviceChangedEvent<'a> {
    /// Converts the XInput2 device changed event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event belongs to
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xinput2_sys::XIDeviceChangedEvent, display: &'a XDisplay) -> Self {
        let classes = std::slice::from_raw_parts(event.classes, event.num_classes as _);

        Self {
            time: event.time,
            device: XInputDevice::from_id(event.deviceid, display),
            source: XInputDevice::from_id(event.sourceid, display),
            reason: XIDeviceChangeReason::new(event.reason),
            classes: classes
                .iter()
                .map(|c| XIClassInfo::new(*c, display))
                .collect(),
        }
    }

    /// Retrieves the timestamp this event occurred at.
    pub fn time(&self) -> u64 {
        self.time
    }

    /// Retrieves the device that changed.
    pub fn device(&self) -> &XInputDevice<'a> {
        &self.device
    }

    /// Retrieves the source of the new classes.
    pub fn source(&self) -> &XInputDevice<'a> {
        &self.source
    }

    /// Retrieves the reason the device changed.
    pub fn reason(&self) -> XIDeviceChangeReason {
        self.reason
    }

    /// Retrieves the new classes of the device
    pub fn classes(&self) -> &[XIClassInfo] {
        &self.classes
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct XIDeviceEventFlags: i32 {
        /// Key event is repeated.
        const KEY_REPEAT = xinput2_sys::XIKeyRepeat;

        /// Pointer is emulated legacy event.
        const POINTER_EMULATED = xinput2_sys::XIPointerEmulated;

        /// Touch end event is pending.
        const TOUCH_PENDING_END = xinput2_sys::XITouchPendingEnd;

        /// Touch events are emulated as pointer events.
        const TOUCH_EMULATING_POINTER = xinput2_sys::XITouchEmulatingPointer;
    }
}

#[derive(Debug, Clone)]
pub struct XIValuatorState {
    mask: Vec<u8>,
    values: Vec<f64>,
}

impl XIValuatorState {
    /// Creates a new XInput2 valuator state.
    ///
    /// # Arguments
    ///
    /// * `mask` - The mask of the state
    /// * `values` - The values of the state
    pub fn new(mask: Vec<u8>, values: Vec<f64>) -> Self {
        Self { mask, values }
    }

    /// Retrieves the mask of this state
    pub fn mask(&self) -> &[u8] {
        &self.mask
    }

    /// Retrieves the values of this state.
    pub fn values(&self) -> &[f64] {
        &self.values
    }
}

#[derive(Debug)]
pub struct XIModifierState {
    base: u32,
    latched: u32,
    locked: u32,
    effective: u32,
}

impl XIModifierState {
    /// Wraps an existing XInput2 modifier state.
    ///
    /// # Arguments
    ///
    /// * `state` - The XInput2 modifier state
    pub fn new(state: xinput2_sys::XIModifierState) -> Self {
        Self {
            base: state.base as _,
            latched: state.latched as _,
            locked: state.locked as _,
            effective: state.effective as _,
        }
    }

    /// Retrieves the currently pressed modifiers.
    pub fn base(&self) -> u32 {
        self.base
    }

    /// Retrieves the latched modifiers.
    pub fn latched(&self) -> u32 {
        self.latched
    }

    /// Retrieves the locked modifiers.
    pub fn locked(&self) -> u32 {
        self.locked
    }

    /// Retrieves the effectively active modifiers.
    pub fn effective(&self) -> u32 {
        self.effective
    }
}

#[derive(Debug)]
pub struct XIDeviceEvent<'a> {
    time: u64,
    device: XInputDevice<'a>,
    source: XInputDevice<'a>,
    detail: i32,
    root: XWindow<'a>,
    event: XWindow<'a>,
    child: XWindow<'a>,
    root_x: f64,
    root_y: f64,
    event_x: f64,
    event_y: f64,
    flags: XIDeviceEventFlags,
    buttons: XIButtonState,
    valuators: XIValuatorState,
    modifiers: XIModifierState,
    group: XIModifierState,
}

impl<'a> XIDeviceEvent<'a> {
    /// Converts the XInput2 device event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event belongs to
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xinput2_sys::XIDeviceEvent, display: &'a XDisplay) -> Self {
        let buttons = std::slice::from_raw_parts(event.buttons.mask, event.buttons.mask_len as _);

        let valuator_mask =
            std::slice::from_raw_parts(event.valuators.mask, event.valuators.mask_len as _);

        let mut current_valuator_value = event.valuators.values;

        let mut valuator_values = Vec::new();

        for i in 0..(valuator_mask.len() * 8) {
            if xinput2_sys::XIMaskIsSet(valuator_mask, i as _) {
                valuator_values.push(*current_valuator_value);
                current_valuator_value = current_valuator_value.add(1);
            }
        }

        let valuators = XIValuatorState::new(valuator_mask.to_vec(), valuator_values);

        Self {
            time: event.time,
            device: XInputDevice::from_id(event.deviceid, display),
            source: XInputDevice::from_id(event.sourceid, display),
            detail: event.detail,
            root: XWindow::new(event.root, display, WindowHandleOwnership::Foreign),
            event: XWindow::new(event.event, display, WindowHandleOwnership::Foreign),
            child: XWindow::new(event.child, display, WindowHandleOwnership::Foreign),
            root_x: event.root_x,
            root_y: event.root_y,
            event_x: event.event_x,
            event_y: event.event_y,
            flags: XIDeviceEventFlags::from_bits_retain(event.flags),
            buttons: XIButtonState::new(buttons.to_vec()),
            valuators,
            modifiers: XIModifierState::new(event.mods),
            group: XIModifierState::new(event.group),
        }
    }

    /// Retrieves the timestamp this event occurred at.
    pub fn time(&self) -> u64 {
        self.time
    }

    /// Retrieves the device that changed.
    pub fn device(&self) -> &XInputDevice<'a> {
        &self.device
    }

    /// Retrieves the source of the new classes.
    pub fn source(&self) -> &XInputDevice<'a> {
        &self.source
    }

    /// Retrieves the detail of this event.
    pub fn detail(&self) -> i32 {
        self.detail
    }

    /// Retrieves the root window this event occurred in.
    pub fn root(&self) -> &XWindow<'a> {
        &self.root
    }

    /// Retrieves the window this event occurred in.
    pub fn event(&self) -> &XWindow<'a> {
        &self.event
    }

    /// Retrieves the window this event targeted.
    pub fn child(&self) -> &XWindow<'a> {
        &self.child
    }

    /// Retrieves the x coordinate in the root window where this event occurred.
    pub fn root_x(&self) -> f64 {
        self.root_x
    }

    /// Retrieves the y coordinate in the root window where this event occurred.
    pub fn root_y(&self) -> f64 {
        self.root_y
    }

    /// Retrieves the x coordinate in the event window where this event occurred.
    pub fn event_x(&self) -> f64 {
        self.event_x
    }

    /// Retrieves the y coordinate in the event window where this event occurred.
    pub fn event_y(&self) -> f64 {
        self.event_y
    }

    /// Retrieves the flags of this event.
    pub fn flags(&self) -> XIDeviceEventFlags {
        self.flags
    }

    /// Retrieves the buttons that triggered this event.
    pub fn buttons(&self) -> &XIButtonState {
        &self.buttons
    }

    /// Retrieves the valuators that were active during this event.
    pub fn valuators(&self) -> &XIValuatorState {
        &self.valuators
    }

    /// Retrieves the modifiers that were active during this event.
    pub fn modifiers(&self) -> &XIModifierState {
        &self.modifiers
    }

    /// Retrieves the group modifiers that were active during this event.
    pub fn group(&self) -> &XIModifierState {
        &self.group
    }
}

#[derive(Debug)]
pub struct XIRawEvent<'a> {
    time: u64,
    device: XInputDevice<'a>,
    source: XInputDevice<'a>,
    detail: i32,
    flags: XIDeviceEventFlags,
    valuators: XIValuatorState,
    raw_values: Vec<f64>,
}

impl<'a> XIRawEvent<'a> {
    /// Converts the XInput2 raw event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event occurred on
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xinput2_sys::XIRawEvent, display: &'a XDisplay) -> Self {
        let valuator_mask =
            std::slice::from_raw_parts(event.valuators.mask, event.valuators.mask_len as _);

        let mut current_valuator_value = event.valuators.values;
        let mut current_raw_value = event.raw_values;

        let mut valuator_values = Vec::new();
        let mut raw_values = Vec::new();

        for i in 0..(valuator_mask.len() * 8) {
            if xinput2_sys::XIMaskIsSet(valuator_mask, i as _) {
                valuator_values.push(*current_valuator_value);
                raw_values.push(*current_raw_value);

                current_valuator_value = current_valuator_value.add(1);
                current_raw_value = current_raw_value.add(1);
            }
        }

        let valuators = XIValuatorState::new(valuator_mask.to_vec(), valuator_values);

        Self {
            time: event.time,
            device: XInputDevice::from_id(event.deviceid, display),
            source: XInputDevice::from_id(event.sourceid, display),
            detail: event.detail,
            flags: XIDeviceEventFlags::from_bits_retain(event.flags),
            valuators,
            raw_values,
        }
    }

    /// Retrieves the timestamp this event occurred at.
    pub fn time(&self) -> u64 {
        self.time
    }

    /// Retrieves the device that changed.
    pub fn device(&self) -> &XInputDevice<'a> {
        &self.device
    }

    /// Retrieves the source of the original event.
    pub fn source(&self) -> &XInputDevice<'a> {
        &self.source
    }

    /// Retrieves the detail of the event.
    pub fn detail(&self) -> i32 {
        self.detail
    }

    /// Retrieves the flags of the event.
    pub fn flags(&self) -> XIDeviceEventFlags {
        self.flags
    }

    /// Retrieves the valuators of the event.
    pub fn valuators(&self) -> &XIValuatorState {
        &self.valuators
    }

    /// Retrieves the raw values of the event.
    pub fn raw_values(&self) -> &[f64] {
        &self.raw_values
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct XITouchOwnershipEventFlags: i32 {}
}

#[derive(Debug)]
pub struct XITouchOwnershipEvent<'a> {
    time: u64,
    device: XInputDevice<'a>,
    source: XInputDevice<'a>,
    touch_id: u32,
    root: XWindow<'a>,
    event: XWindow<'a>,
    child: XWindow<'a>,
    flags: XITouchOwnershipEventFlags,
}

impl<'a> XITouchOwnershipEvent<'a> {
    /// Converts the XInput2 raw event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event belongs to
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xinput2_sys::XITouchOwnershipEvent, display: &'a XDisplay) -> Self {
        Self {
            time: event.time,
            device: XInputDevice::from_id(event.deviceid, display),
            source: XInputDevice::from_id(event.sourceid, display),
            touch_id: event.touchid,
            root: XWindow::new(event.root, display, WindowHandleOwnership::Foreign),
            event: XWindow::new(event.event, display, WindowHandleOwnership::Foreign),
            child: XWindow::new(event.child, display, WindowHandleOwnership::Foreign),
            flags: XITouchOwnershipEventFlags::from_bits_retain(event.flags),
        }
    }

    /// Retrieves the timestamp this event occurred at.
    pub fn time(&self) -> u64 {
        self.time
    }

    /// Retrieves the device that changed.
    pub fn device(&self) -> &XInputDevice<'a> {
        &self.device
    }

    /// Retrieves the source of the original event.
    pub fn source(&self) -> &XInputDevice<'a> {
        &self.source
    }

    /// Retrieves the touch id of the event.
    pub fn touch_id(&self) -> u32 {
        self.touch_id
    }

    /// Retrieves the root window this event occurred in.
    pub fn root(&self) -> &XWindow<'a> {
        &self.root
    }

    /// Retrieves the window this event occurred in.
    pub fn event(&self) -> &XWindow<'a> {
        &self.event
    }

    /// Retrieves the window this event targeted.
    pub fn child(&self) -> &XWindow<'a> {
        &self.child
    }

    /// Retrieves the flags of this event.
    pub fn flags(&self) -> XITouchOwnershipEventFlags {
        self.flags
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct XIBarrierEventFlags: i32 {
        /// The pointer has been released from the barrier.
        const POINTER_RELEASED = xinput2_sys::XIBarrierPointerReleased;

        /// The device is grabbed.
        const DEVICE_IS_GRABBED = xinput2_sys::XIBarrierDeviceIsGrabbed;
    }
}

#[derive(Debug)]
pub struct XIBarrierEvent<'a> {
    time: u64,
    device: XInputDevice<'a>,
    source: XInputDevice<'a>,
    event: XWindow<'a>,
    root: XWindow<'a>,
    root_x: f64,
    root_y: f64,
    dx: f64,
    dy: f64,
    dtime: i32,
    flags: XIBarrierEventFlags,
    barrier: xlib_sys::XID,
    event_id: u32,
}

impl<'a> XIBarrierEvent<'a> {
    /// Converts the XInput2 barrier event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event belongs to
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xinput2_sys::XIBarrierEvent, display: &'a XDisplay) -> Self {
        Self {
            time: event.time,
            device: XInputDevice::from_id(event.deviceid, display),
            source: XInputDevice::from_id(event.sourceid, display),
            event: XWindow::new(event.event, display, WindowHandleOwnership::Foreign),
            root: XWindow::new(event.root, display, WindowHandleOwnership::Foreign),
            root_x: event.root_x,
            root_y: event.root_y,
            dx: event.dx,
            dy: event.dy,
            dtime: event.dtime,
            flags: XIBarrierEventFlags::from_bits_retain(event.flags),
            barrier: event.barrier,
            event_id: event.eventid,
        }
    }

    /// Retrieves the timestamp this event occurred at.
    pub fn time(&self) -> u64 {
        self.time
    }

    /// Retrieves the device that changed.
    pub fn device(&self) -> &XInputDevice<'a> {
        &self.device
    }

    /// Retrieves the source of the new classes.
    pub fn source(&self) -> &XInputDevice<'a> {
        &self.source
    }

    /// Retrieves the root window this event occurred in.
    pub fn root(&self) -> &XWindow<'a> {
        &self.root
    }

    /// Retrieves the window this event occurred in.
    pub fn event(&self) -> &XWindow<'a> {
        &self.event
    }

    /// Retrieves the x coordinate in the root window where this event occurred.
    pub fn root_x(&self) -> f64 {
        self.root_x
    }

    /// Retrieves the y coordinate in the root window where this event occurred.
    pub fn root_y(&self) -> f64 {
        self.root_y
    }

    /// Retrieves the x delta of this event.
    pub fn dx(&self) -> f64 {
        self.dx
    }

    /// Retrieves the y delta of this event.
    pub fn dy(&self) -> f64 {
        self.dy
    }

    /// Retrieves the delta time of this event.
    pub fn dtime(&self) -> i32 {
        self.dtime
    }

    /// Retrieves the flags of this event.
    pub fn flags(&self) -> XIBarrierEventFlags {
        self.flags
    }

    /// Retrieves the barrier id which triggered this event.
    pub fn barrier(&self) -> xlib_sys::XID {
        self.barrier
    }

    /// Retrieves the barrier event id of this event
    pub fn event_id(&self) -> u32 {
        self.event_id
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum XIFocusEventMode {
    Normal = xinput2_sys::XINotifyNormal,
    Grab = xinput2_sys::XINotifyGrab,
    Ungrab = xinput2_sys::XINotifyUngrab,
}

impl XIFocusEventMode {
    /// Wraps an existing XInput2 focus event mode.
    ///
    /// # Arguments
    ///
    /// * `mode` - The XInput2 focus event mode to wrap
    ///
    /// # Panics
    ///
    /// If the given focus event mode is not valid.
    pub fn new(mode: i32) -> Self {
        match mode {
            xinput2_sys::XINotifyNormal => Self::Normal,
            xinput2_sys::XINotifyGrab => Self::Grab,
            xinput2_sys::XINotifyUngrab => Self::Ungrab,
            x => unreachable!("Unknown XI focus event mode {}", x),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum XIFocusEventDetail {
    Ancestor = xinput2_sys::XINotifyAncestor,
    Virtual = xinput2_sys::XINotifyVirtual,
    Inferior = xinput2_sys::XINotifyInferior,
    Nonlinear = xinput2_sys::XINotifyNonlinear,
    NonlinearVirtual = xinput2_sys::XINotifyNonlinearVirtual,
    Pointer = xinput2_sys::XINotifyPointer,
    PointerRoot = xinput2_sys::XINotifyPointerRoot,
    None = xinput2_sys::XINotifyDetailNone,
}

impl XIFocusEventDetail {
    /// Wraps an existing XInput2 focus event detail.
    ///
    /// # Arguments
    ///
    /// * `detail` - The XInput2 focus event detail to wrap
    ///
    /// # Panics
    ///
    /// If the given focus event detail is not valid.
    pub fn new(detail: i32) -> Self {
        match detail {
            xinput2_sys::XINotifyAncestor => Self::Ancestor,
            xinput2_sys::XINotifyVirtual => Self::Virtual,
            xinput2_sys::XINotifyInferior => Self::Inferior,
            xinput2_sys::XINotifyNonlinear => Self::Nonlinear,
            xinput2_sys::XINotifyNonlinearVirtual => Self::NonlinearVirtual,
            xinput2_sys::XINotifyPointer => Self::Pointer,
            xinput2_sys::XINotifyPointerRoot => Self::PointerRoot,
            xinput2_sys::XINotifyDetailNone => Self::None,
            x => unreachable!("Unknown XI focus event detail {}", x),
        }
    }
}

#[derive(Debug)]
pub struct XIFocusEvent<'a> {
    time: u64,
    device: XInputDevice<'a>,
    source: XInputDevice<'a>,
    detail: XIFocusEventDetail,
    root: XWindow<'a>,
    event: XWindow<'a>,
    child: XWindow<'a>,
    root_x: f64,
    root_y: f64,
    event_x: f64,
    event_y: f64,
    mode: XIFocusEventMode,
    focus: bool,
    same_screen: bool,
    buttons: XIButtonState,
    modifiers: XIModifierState,
    group: XIModifierState,
}

impl<'a> XIFocusEvent<'a> {
    /// Converts the XInput2 focus event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event belongs to
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xinput2_sys::XIEnterEvent, display: &'a XDisplay) -> Self {
        let buttons = std::slice::from_raw_parts(event.buttons.mask, event.buttons.mask_len as _);

        Self {
            time: event.time,
            device: XInputDevice::from_id(event.deviceid, display),
            source: XInputDevice::from_id(event.sourceid, display),
            detail: XIFocusEventDetail::new(event.detail),
            root: XWindow::new(event.root, display, WindowHandleOwnership::Foreign),
            event: XWindow::new(event.event, display, WindowHandleOwnership::Foreign),
            child: XWindow::new(event.child, display, WindowHandleOwnership::Foreign),
            root_x: event.root_x,
            root_y: event.root_y,
            event_x: event.event_x,
            event_y: event.event_y,
            mode: XIFocusEventMode::new(event.mode),
            focus: event.focus != 0,
            same_screen: event.same_screen != 0,
            buttons: XIButtonState::new(buttons.to_vec()),
            modifiers: XIModifierState::new(event.mods),
            group: XIModifierState::new(event.group),
        }
    }

    /// Retrieves the timestamp this event occurred at.
    pub fn time(&self) -> u64 {
        self.time
    }

    /// Retrieves the device that changed.
    pub fn device(&self) -> &XInputDevice<'a> {
        &self.device
    }

    /// Retrieves the source of the new classes.
    pub fn source(&self) -> &XInputDevice<'a> {
        &self.source
    }

    /// Retrieves the detail of this event.
    pub fn detail(&self) -> XIFocusEventDetail {
        self.detail
    }

    /// Retrieves the root window this event occurred in.
    pub fn root(&self) -> &XWindow<'a> {
        &self.root
    }

    /// Retrieves the window this event occurred in.
    pub fn event(&self) -> &XWindow<'a> {
        &self.event
    }

    /// Retrieves the window this event targeted.
    pub fn child(&self) -> &XWindow<'a> {
        &self.child
    }

    /// Retrieves the x coordinate in the root window where this event occurred.
    pub fn root_x(&self) -> f64 {
        self.root_x
    }

    /// Retrieves the y coordinate in the root window where this event occurred.
    pub fn root_y(&self) -> f64 {
        self.root_y
    }

    /// Retrieves the x coordinate in the event window where this event occurred.
    pub fn event_x(&self) -> f64 {
        self.event_x
    }

    /// Retrieves the y coordinate in the event window where this event occurred.
    pub fn event_y(&self) -> f64 {
        self.event_y
    }

    /// Retrieves the mode of this event.
    pub fn mode(&self) -> XIFocusEventMode {
        self.mode
    }

    /// Determines whether the focus is now active.
    pub fn focus(&self) -> bool {
        self.focus
    }

    /// Determines whether the event occurred on the same screen.
    pub fn same_screen(&self) -> bool {
        self.same_screen
    }

    /// Retrieves the buttons that triggered this event.
    pub fn buttons(&self) -> &XIButtonState {
        &self.buttons
    }

    /// Retrieves the modifiers that were active during this event.
    pub fn modifiers(&self) -> &XIModifierState {
        &self.modifiers
    }

    /// Retrieves the group modifiers that were active during this event.
    pub fn group(&self) -> &XIModifierState {
        &self.group
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum XIPropertyEventChange {
    Created = xinput2_sys::XIPropertyCreated,
    Deleted = xinput2_sys::XIPropertyDeleted,
    Modified = xinput2_sys::XIPropertyModified,
}

impl XIPropertyEventChange {
    /// Wraps an existing XInput2 property event change.
    ///
    /// # Arguments
    ///
    /// * `change` - The XInput2 property event change to wrap
    ///
    /// # Panics
    ///
    /// If the given property event change is not valid.
    pub fn new(change: i32) -> Self {
        match change {
            xinput2_sys::XIPropertyCreated => Self::Created,
            xinput2_sys::XIPropertyDeleted => Self::Deleted,
            xinput2_sys::XIPropertyModified => Self::Modified,
            x => unreachable!("Unknown XI property event change {}", x),
        }
    }
}

#[derive(Debug)]
pub struct XIPropertyEvent<'a> {
    time: u64,
    device: XInputDevice<'a>,
    property: XAtom<'a>,
    what: XIPropertyEventChange,
}

impl<'a> XIPropertyEvent<'a> {
    /// Converts the XInput2 property event data from its native representation.
    ///
    /// # Arguments
    ///
    /// * `event` - The X native event
    /// * `display` - The display the event belongs to
    ///
    /// # Safety
    ///
    /// The caller must ensure all arguments are valid.
    pub unsafe fn new(event: xinput2_sys::XIPropertyEvent, display: &'a XDisplay) -> Self {
        Self {
            time: event.time,
            device: XInputDevice::from_id(event.deviceid, display),
            property: XAtom::new(event.property, display),
            what: XIPropertyEventChange::new(event.what),
        }
    }

    /// Retrieves the timestamp this event occurred at.
    pub fn time(&self) -> u64 {
        self.time
    }

    /// Retrieves the device that changed.
    pub fn device(&self) -> &XInputDevice<'a> {
        &self.device
    }

    /// Retrieves the property that changed.
    pub fn property(&self) -> &XAtom<'a> {
        &self.property
    }

    /// Retrieves what happened.
    pub fn what(&self) -> XIPropertyEventChange {
        self.what
    }
}
