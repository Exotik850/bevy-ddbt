use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::InstructionError;

#[typetag::serde]
pub trait Action: Send + Sync {
    fn execute(&self, ctx: &mut InstructionContext);
}

#[typetag::serde]
pub trait Sensor: Send + Sync {
    fn matches(&self, ctx: &InstructionContext) -> bool;
}

#[derive(Debug)]
pub struct InstructionContext {
    // Add any context fields as needed
    pub blackboard: HashMap<String, Value>,
}

#[derive(Deserialize)]
pub struct Instruction {
    #[serde(default)]
    pub sensors: Vec<Box<dyn Sensor>>,
    #[serde(default)]
    pub actions: Vec<Box<dyn Action>>,
    #[serde(default)]
    pub children: Vec<Instruction>,
}

impl Instruction {
    /// Deserializes an Instruction using typetag's polymorphic deserialization.
    pub fn deserialize(data: &Value) -> Result<Self, InstructionError> {
        serde_json::from_value(data.clone())
            .map_err(|e| InstructionError::Deserialization { source: e })
    }

    fn execute(&self, ctx: &mut InstructionContext) -> bool {
        if !self.sensors.iter().all(|sensor| sensor.matches(ctx)) {
            return false;
        }
        for action in &self.actions {
            action.execute(ctx);
        }
        for child in &self.children {
            child.execute(ctx);
        }
        true
    }
}
