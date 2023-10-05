pub mod export;
pub mod targets;
pub mod prelude;

pub use prelude::Burr;
// these re-exports are necessary for the proc macro to work without requiring the user to include them as dependencies
#[doc(hidden)]
pub use burrtype_internal::ir;
#[doc(hidden)]
pub use syn;
#[doc(hidden)]
pub use quote;
#[doc(hidden)]
pub use linkme;

#[linkme::distributed_slice]
pub static TYPES: [fn() -> ir::IrItem] = [..];