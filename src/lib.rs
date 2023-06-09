//! This library is an X11 Rust wrapper which provides a mostly safe interface for XLib.
//!
//! Please note that the available functionality has been tailored to fit Snowland and as such
//! this wrapper does not reflect all of X11. A lot of error checks are also missing (due to X11
//! bad error handling mechanism).

mod ext;
mod glx;
mod x11;

pub use self::x11::*;
pub use glx::*;

pub use ::x11::glx as glx_sys;
pub use ::x11::glx::arb as glx_arb_sys;
pub use ::x11::keysym as xkeysym_sys;
pub use ::x11::xcomposite as xcomposite_sys;
pub use ::x11::xfixes as xfixes_sys;
pub use ::x11::xinput2 as xinput2_sys;
pub use ::x11::xlib as xlib_sys;
pub use ::x11::xrandr as xrandr_sys;
pub use ::x11::xtest as xtest_sys;
