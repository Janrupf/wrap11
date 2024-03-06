use crate::ext::edid::MonitorDescriptor;
use crate::{
    xcomposite_sys, xlib_sys, xrandr_sys, ColormapAllocation, ColormapHandleOwnership,
    SetWindowAttributes, WindowClass, WindowHandleOwnership, XAtom, XColormap, XVisual,
    XVisualInfo,
};
use crate::{XDisplay, XWindow};
use std::io::Cursor;
use std::mem::MaybeUninit;
use std::slice;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum VisualClass {
    StaticGray = xlib_sys::StaticGray,
    GrayScale = xlib_sys::GrayScale,
    StaticColor = xlib_sys::StaticColor,
    PseudoColor = xlib_sys::PseudoColor,
    TrueColor = xlib_sys::TrueColor,
    DirectColor = xlib_sys::DirectColor,
}

/// XRandR info about a connected monitor.
#[derive(Debug)]
pub struct XRandRMonitorInfo<'a> {
    /// The X atom representing the connection name of the monitor.
    pub connection_name: XAtom<'a>,

    /// The physical name of the monitor.
    pub monitor_name: Option<String>,

    /// The serial of the monitor.
    pub monitor_serial: Option<u32>,

    /// Whether this monitor is the primary monitor.
    pub primary: bool,

    /// Whether this monitor is currently in automatic configuration mode.
    pub automatic: bool,

    /// The output number of physical connections from this virtual monitor.
    pub output_count: i32,

    /// The x coordinate at which the monitor starts.
    pub x: i32,

    /// The y coordinate at which the monitor starts.
    pub y: i32,

    /// The width of the monitor in pixels.
    pub width: i32,

    /// The height of the monitor in pixels.
    pub height: i32,

    /// The physical width of the monitor.
    pub physical_width: i32,

    /// The physical height of the monitor.
    pub physical_height: i32,
}

/// X11 screen.
///
/// Please note that while originally screens where meant to represent different heads (monitors)
/// on an X system, they rarely do anymore. Usually all monitors are combined as one huge screen
/// and the window manager takes care of assigning application windows to monitors.
///
/// Thus you can usually expect one X11 display to have one screen!
#[derive(Debug)]
pub struct XScreen<'a> {
    handle: *mut xlib_sys::Screen,
    display: &'a XDisplay,
}

impl<'a> XScreen<'a> {
    /// Wraps a native X11 screen.
    ///
    /// # Arguments
    ///
    /// * `handle` - The native platform X11 pointer of the screen
    /// * `display` - The display the screen belongs to (and often represents entirely)
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure all arguments are valid.
    pub unsafe fn new(handle: *mut xlib_sys::Screen, display: &'a XDisplay) -> Self {
        Self { handle, display }
    }

    /// Retrieves the display this screen belongs to.
    pub fn display(&self) -> &'a XDisplay {
        self.display
    }

    /// Retrieves the underlying platform native X11 pointer.
    pub fn handle(&self) -> *mut xlib_sys::Screen {
        self.handle
    }

    /// Retrieves the number of the screen, usually 0.
    pub fn number(&self) -> i32 {
        unsafe { xlib_sys::XScreenNumberOfScreen(self.handle) }
    }

    /// Retrieves the root window of the screen.
    ///
    /// The root window is the top level background window which spans the entire screen.
    pub fn root_window(&self) -> XWindow<'a> {
        unsafe {
            XWindow::new(
                (*self.handle).root,
                self.display,
                WindowHandleOwnership::Foreign,
            )
        }
    }

    /// Retrieves the composite window of the screen.
    ///
    /// The composite window is a window, which lies on top of all other windows
    /// but receives no input events.
    pub fn composite_window(&self) -> XWindow<'a> {
        unsafe {
            let window = xcomposite_sys::XCompositeGetOverlayWindow(
                self.display.handle(),
                (*self.handle).root,
            );
            XWindow::new(
                window,
                self.display,
                WindowHandleOwnership::OwnedCompositeOverlay,
            )
        }
    }

    /// Retrieves the default visual of the screen.
    pub fn default_visual(&self) -> XVisual<'a> {
        unsafe {
            let visual = xlib_sys::XDefaultVisual(self.display.handle(), self.number());
            XVisual::new(visual)
        }
    }

    /// Attempts to find a visual matching certain criteria for the screen.
    ///
    /// # Arguments
    ///
    /// * `depth` - The color depth in bits to look up
    /// * `class` - The class of the visual to look up
    pub fn match_visual(&self, depth: i32, class: VisualClass) -> Option<XVisualInfo<'a>> {
        let mut info_out = MaybeUninit::uninit();

        unsafe {
            if xlib_sys::XMatchVisualInfo(
                self.display.handle(),
                self.number(),
                depth,
                class as _,
                info_out.as_mut_ptr(),
            ) == 0
            {
                return None;
            }

            let info = info_out.assume_init();
            Some(XVisualInfo::new(info, XVisual::new(info.visual)))
        }
    }

    /// Creates a new colormap.
    ///
    /// # Arguments
    ///
    /// * `visual` - The visual to create the colormap for
    /// * `allocation` - The initial colormap allocation
    pub fn create_colormap(
        &self,
        visual: &XVisual<'a>,
        allocation: ColormapAllocation,
    ) -> XColormap<'a> {
        unsafe {
            let colormap = xlib_sys::XCreateColormap(
                self.display.handle(),
                self.root_window().handle(),
                visual.handle(),
                allocation as _,
            );

            XColormap::new(colormap, self.display, ColormapHandleOwnership::Owned)
        }
    }

    /// Creates a new simple window on this screen..
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate of the window
    /// * `y` - The y coordinate of the window
    /// * `width` - The width in pixels of the inside window area excluding the border
    /// * `height` - The height in pixels of the inside window area excluding the border
    /// * `border_width` - The width of the window border in pixels
    /// * `border` - The border pixel value of the window
    /// * `background` - The background pixel value of the window
    #[allow(clippy::too_many_arguments)]
    pub fn create_simple_window(
        &self,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        border_width: u32,
        border: u64,
        background: u64,
    ) -> XWindow<'a> {
        self.root_window().create_simple_child_window(
            x,
            y,
            width,
            height,
            border_width,
            border,
            background,
        )
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
    pub fn create_window<'creation>(
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
        self.root_window().create_child_window(
            x,
            y,
            width,
            height,
            border_width,
            depth,
            class,
            visual,
            attributes,
        )
    }

    /// Retrieves all monitors connected to this screen.
    pub fn get_monitors(&self) -> Vec<XRandRMonitorInfo<'a>> {
        let mut monitor_count = 0;
        let info = unsafe {
            xrandr_sys::XRRGetMonitors(
                self.display.handle(),
                (*self.handle).root,
                1,
                &mut monitor_count,
            )
        };

        if info.is_null() {
            return Vec::new();
        }

        let edid_atom = self.display.get_atom("EDID");

        let mut out = Vec::with_capacity(monitor_count as _);

        for i in 0..monitor_count {
            let info = unsafe { &*info.offset(i as _) };

            let edid = edid_atom.and_then(|edid_atom| {
                if info.noutput > 0 {
                    unsafe {
                        let mut actual_type = 0;
                        let mut actual_format = 0;
                        let mut item_count = 0;
                        let mut remaining_bytes = 0;
                        let mut data = std::ptr::null_mut();

                        xrandr_sys::XRRGetOutputProperty(
                            self.display.handle(),
                            *info.outputs,
                            edid_atom.handle(),
                            0,
                            100,
                            0,
                            0,
                            xlib_sys::AnyPropertyType as _,
                            &mut actual_type,
                            &mut actual_format,
                            &mut item_count,
                            &mut remaining_bytes,
                            &mut data,
                        );

                        let edid_data = slice::from_raw_parts(data as *const u8, item_count as _);

                        let edid = crate::ext::edid::parse(&mut Cursor::new(edid_data));

                        xlib_sys::XFree(data as _);

                        edid.ok()
                    }
                } else {
                    None
                }
            });

            let (name, serial) = match edid {
                None => (None, None),
                Some(edid) => {
                    let name = edid.descriptors.0.into_iter().find_map(|desc| {
                        if let MonitorDescriptor::MonitorName(name) = desc {
                            Some(name)
                        } else {
                            None
                        }
                    });

                    (name, Some(edid.product.serial_number))
                }
            };

            out.push(XRandRMonitorInfo {
                connection_name: unsafe { XAtom::new(info.name, self.display) },
                monitor_name: name,
                monitor_serial: serial,
                primary: info.primary != 0,
                automatic: info.automatic != 0,
                output_count: info.noutput,
                x: info.x,
                y: info.y,
                width: info.width,
                height: info.height,
                physical_width: info.mwidth,
                physical_height: info.mheight,
            })
        }

        unsafe {
            xlib_sys::XFree(info as _);
        }

        out
    }
}
