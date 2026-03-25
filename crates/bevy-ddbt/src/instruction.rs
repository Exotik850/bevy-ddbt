use std::{collections::HashMap, ops::ControlFlow};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::errors::InstructionError;




#[derive(Deserialize, Default, Serialize)]
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
        let mut instruction = Instruction::default();
        if let Some(sensors) = data.get("sensors") {
            instruction.sensors = serde_json::from_value(sensors.clone())
                .map_err(|e| InstructionError::Deserialization { source: e })?;
        }
        if let Some(actions) = data.get("actions") {
            instruction.actions = serde_json::from_value(actions.clone())
                .map_err(|e| InstructionError::Deserialization { source: e })?;
        }
        if let Some(children) = data.get("children") {
            instruction.children = serde_json::from_value(children.clone())
                .map_err(|e| InstructionError::Deserialization { source: e })?;
        }
        Ok(instruction)
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
