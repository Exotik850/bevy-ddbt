use serde::{Deserialize, Serialize};

use crate::prelude::Sensor;

#[derive(Serialize, Deserialize)]
pub struct False {}

#[typetag::serde(name = "false")]
impl Sensor for False {
    fn matches(&self, _ctx: &crate::instruction::InstructionContext) -> bool {
        false
    }
}

#[derive(Serialize, Deserialize)]
pub struct True {}

#[typetag::serde(name = "true")]
impl Sensor for True {
    fn matches(&self, _ctx: &crate::instruction::InstructionContext) -> bool {
        true
    }
}

#[derive(Serialize, Deserialize)]
pub struct Not(pub Box<dyn Sensor>);

#[typetag::serde(name = "not")]
impl Sensor for Not {
    fn matches(&self, ctx: &crate::instruction::InstructionContext) -> bool {
        !self.0.matches(ctx)
    }
}

#[derive(Serialize, Deserialize)]
pub struct And(Vec<Box<dyn Sensor>>);

#[typetag::serde(name = "and")]
impl Sensor for And {
    fn matches(&self, ctx: &crate::instruction::InstructionContext) -> bool {
        self.0.iter().all(|sensor| sensor.matches(ctx))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Or(pub Vec<Box<dyn Sensor>>);

#[typetag::serde(name = "or")]
impl Sensor for Or {
    fn matches(&self, ctx: &crate::instruction::InstructionContext) -> bool {
        self.0.iter().any(|sensor| sensor.matches(ctx))
    }
}