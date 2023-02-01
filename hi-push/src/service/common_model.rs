use serde_repr::{Deserialize_repr, Serialize_repr};

/**
 *
 * Data transfer object
 */
#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy, Default, Eq, PartialEq)]
#[repr(u8)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen::prelude::wasm_bindgen)]
pub enum Code {
    #[default]
    Ok,
    Err,
    System,
    Unknown,
}
