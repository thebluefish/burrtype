#[path = "ir/enum.rs"]
mod r#enum;
#[path = "ir/item.rs"]
mod item;
#[path = "ir/mod.rs"]
mod r#mod;
#[path = "ir/struct.rs"]
mod r#struct;

pub use r#enum::*;
pub use item::*;
pub use r#mod::*;
pub use r#struct::*;
