#[macro_use]
extern crate serde_derive;

#[cfg(feature = "sys")]
pub use rtdlib_sys::Tdlib;

mod codec;
pub mod types;
pub mod errors;
