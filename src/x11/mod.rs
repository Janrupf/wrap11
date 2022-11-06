mod atom;
mod colormap;
mod cursor;
mod display;
mod drawable;
mod event;
mod font;
mod gc;
mod image;
mod input;
mod pixmap;
mod property;
mod region;
mod screen;
mod visual;
mod window;

pub use atom::*;
pub use colormap::*;
pub use cursor::*;
pub use display::*;
pub use drawable::*;
pub use event::*;
pub use font::*;
pub use gc::*;
pub use image::*;
pub use input::*;
pub use pixmap::*;
pub use property::*;
pub use region::*;
pub use screen::*;
pub use visual::*;
pub use window::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum XLibError {
    #[error("failed to open display: {0}")]
    OpenDisplayFailed(String),
}
