use std::collections::HashMap;
use serde_json::Value;


#[derive(Debug)]
pub struct InstructionContext {
    // Add any context fields as needed
    pub blackboard: HashMap<String, Value>,
}

#[typetag::serde]
pub trait Action: Send + Sync {
    fn execute(&self, ctx: &mut InstructionContext);
}

#[typetag::serde]
pub trait Sensor: Send + Sync {
    fn matches(&self, ctx: &InstructionContext) -> bool;
}
