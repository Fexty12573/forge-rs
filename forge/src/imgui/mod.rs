pub mod color;
pub mod context;
pub mod drag;
pub mod draw;
pub mod font;
pub mod input;
pub mod item;
pub mod layout;
pub mod menu;
pub mod plot;
pub mod popup;
pub mod slider;
pub mod stack;
pub mod table;
pub mod tooltip;
pub mod tree;
pub mod widgets;
pub mod window;

pub use color::*;
pub use context::*;
pub use drag::*;
pub use draw::*;
pub use font::*;
pub use input::*;
pub use item::*;
pub use layout::*;
pub use menu::*;
pub use plot::*;
pub use popup::*;
pub use slider::*;
pub use stack::*;
pub use table::*;
pub use tooltip::*;
pub use tree::*;
pub use widgets::*;
pub use window::*;

pub enum Id {
    Int(u32),
    Str(&'static str),
}
