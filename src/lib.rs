#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate failure;

pub mod api;
pub mod errors;

pub use api::GeekClient;
