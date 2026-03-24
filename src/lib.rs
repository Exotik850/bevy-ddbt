mod instruction;
mod errors;
pub mod sensors;

pub mod prelude {
    pub use crate::{
        instruction::{Action, Sensor, Instruction, InstructionContext},
        errors::{InstructionError, SensorError, ActionError},
    };
}