pub mod export;
pub mod targets;
pub mod prelude;
pub mod type_registration;

pub use prelude::burr;
// these re-exports are necessary for the proc macro to work without requiring the user to include them as dependencies
#[doc(hidden)]
pub use bevy_reflect::{Reflect, GetTypeRegistration, TypeRegistration};
#[doc(hidden)]
pub use syn;
#[doc(hidden)]
pub use quote;
#[doc(hidden)]
pub use linkme;

#[linkme::distributed_slice]
pub static TYPES: [fn() -> TypeRegistration] = [..];