mod instruction;

pub mod sensors;
use bevy_wasm_shared::prelude::*;

/// The version of the protocol. Automatically set from the `CARGO_PKG_XXX` environment variables.
pub const PROTOCOL_VERSION: Version = version!();

pub mod prelude {
    pub use crate::{
        instruction::{Action, Sensor, InstructionContext},
    };
}