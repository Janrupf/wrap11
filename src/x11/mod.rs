mod atom;
mod colormap;
mod cursor;
mod display;
mod drawable;
mod event;
mod gc;
mod pixmap;
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
pub use gc::*;
pub use pixmap::*;
pub use region::*;
pub use screen::*;
pub use visual::*;
pub use window::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum XLibError {
    #[error("failed to open display :{0}")]
    OpenDisplayFailed(String),
}
