use serde::{Deserialize, Serialize};

use crate::prelude::Action;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoOp {}

#[typetag::serde(name = "noop")]
impl Action for NoOp {
    fn execute(&self, _ctx: &mut crate::instruction::InstructionContext) {
        // No operation - does nothing
    }
}
