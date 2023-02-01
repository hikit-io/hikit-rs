#[cfg(all(any(feature = "http-api", feature = "grpc-api", feature = "client"), not(target_arch = "wasm32")))]
pub mod model;

#[cfg(all(feature = "client", target_arch = "wasm32"))]
pub mod model_wasm;

#[cfg(all(feature = "client", target_arch = "wasm32"))]
pub use model_wasm as model;

pub mod common_model;

#[cfg(all(not(feature = "client")))]
pub mod persis_model;

#[cfg(any(feature = "http-api", feature = "grpc-api"))]
pub mod service;

#[cfg(any(feature = "http-api", feature = "grpc-api"))]
pub use service::*;

#[cfg(all(feature = "mongo", not(feature = "client")))]
pub mod mongo;

#[cfg(test)]
mod tests {}
