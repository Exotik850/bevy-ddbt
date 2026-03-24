use bevy_ddbt::prelude::*;
use serde_json::json;
use std::collections::HashMap;

fn main() {
    // Define a "Not: False" sensor with a no-op action
    // The instruction structure:
    // - sensors: [not: { sensor: { false: {} } }]
    // - actions: [noop: {}]
    let instruction_json = json!({
        "sensors": [
            {
                "not": {
                  "false": {}
                }
            }
        ],
        "actions": [
            {
                "noop": {}
            }
        ]
    });

    println!("Instruction JSON: {}", instruction_json);

    // Deserialize the instruction using typetag's polymorphic deserialization
    match Instruction::deserialize(&instruction_json) {
        Ok(instruction) => {
            println!("\n✓ Successfully deserialized instruction!");
            println!("  - Sensors: {}", instruction.sensors.len());
            println!("  - Actions: {}", instruction.actions.len());

            // Create a context for execution
            let mut ctx = InstructionContext {
                blackboard: HashMap::new(),
            };

            // Test the sensor matching
            println!("\n--- Testing Sensor Matching ---");
            let all_match = instruction
                .sensors
                .iter()
                .all(|sensor| sensor.matches(&ctx));
            println!("Sensor matches: {}", all_match);

            // Let's test each sensor individually
            for (i, sensor) in instruction.sensors.iter().enumerate() {
                let matches = sensor.matches(&ctx);
                println!("  Sensor {} matches: {}", i, matches);
            }

            // Execute actions if sensors match
            println!("\n--- Executing Actions ---");
            if all_match {
                for (i, action) in instruction.actions.iter().enumerate() {
                    println!("Executing action {}...", i);
                    action.execute(&mut ctx);
                    println!("  Action {} completed (no-op)", i);
                }
                println!("\n✓ All actions executed successfully!");
            } else {
                println!("Sensors did not match, skipping actions");
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to deserialize instruction: {:?}", e);
            std::process::exit(1);
        }
    }

    // Now let's also test serialization back to JSON
    println!("\n--- Testing Serialization ---");
    // Note: The Instruction struct doesn't currently have serialization support,
    // but we can demonstrate the JSON structure we started with
    println!("Original JSON structure can be re-serialized:");
    let re_serialized = serde_json::to_string_pretty(&instruction_json).unwrap();
    println!("{}", re_serialized);
}
