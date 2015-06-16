//! Structures related to seats and inputs.

pub use self::keyboard::{Keyboard, KeyboardId};
pub use self::pointer::{Pointer, PointerId};
pub use self::seat::Seat;

pub use self::keyboard::KeymapFormat;
pub use self::keyboard::KeyState;
pub use self::pointer::ScrollAxis;
pub use self::pointer::ButtonState;

mod keyboard;
mod pointer;
mod seat;