use std::fmt::{self, Display};


/// Error type for instruction deserialization.
#[derive(Debug)]
pub enum InstructionError {
    /// The instruction data was not an object.
    InvalidFormat,
    /// A sensor deserialization error occurred.
    Sensor {
        /// The underlying sensor error.
        source: SensorError,
    },
    /// An action deserialization error occurred.
    Action {
        /// The underlying action error.
        source: ActionError,
    },
    /// A child instruction deserialization error occurred.
    Child {
        /// The underlying instruction error.
        source: Box<InstructionError>,
    },
    /// General deserialization error.
    Deserialization {
        /// The underlying serde_json error.
        source: serde_json::Error,
    },
}


impl Display for InstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InstructionError::InvalidFormat => {
                write!(f, "Instruction data must be an object")
            }
            InstructionError::Sensor { source } => {
                write!(f, "Sensor error: {}", source)
            }
            InstructionError::Action { source } => {
                write!(f, "Action error: {}", source)
            }
            InstructionError::Child { source } => {
                write!(f, "Child instruction error: {}", source)
            }
            InstructionError::Deserialization { source } => {
                write!(f, "Deserialization error: {}", source)
            }
        }
    }
}

impl std::error::Error for InstructionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            InstructionError::InvalidFormat => None,
            InstructionError::Sensor { source } => Some(source),
            InstructionError::Action { source } => Some(source),
            InstructionError::Child { source } => Some(source.as_ref()),
            InstructionError::Deserialization { source } => Some(source),
        }
    }
}

impl From<SensorError> for InstructionError {
    fn from(err: SensorError) -> Self {
        InstructionError::Sensor { source: err }
    }
}

impl From<ActionError> for InstructionError {
    fn from(err: ActionError) -> Self {
        InstructionError::Action { source: err }
    }
}


/// Error type for action-related operations.
#[derive(Debug)]
pub enum ActionError {
    /// The action type was not found in the registry.
    UnknownAction(String),
    /// Deserialization of the action failed.
    Deserialization {
        /// The action name that failed to deserialize.
        action: String,
        /// The underlying simd_json error.
        source: serde_json::Error,
    },
}

impl Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActionError::UnknownAction(name) => {
                write!(f, "Unknown action type: {}", name)
            }
            ActionError::Deserialization { action, source } => {
                write!(f, "Failed to deserialize action '{}': {}", action, source)
            }
        }
    }
}

impl std::error::Error for ActionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ActionError::UnknownAction(_) => None,
            ActionError::Deserialization { source, .. } => Some(source),
        }
    }
}

/// Error type for sensor-related operations.
#[derive(Debug)]
pub enum SensorError {
    /// The sensor type was not found in the registry.
    UnknownSensor(String),
    /// Deserialization of the sensor failed.
    Deserialization {
        /// The sensor name that failed to deserialize.
        sensor: String,
        /// The underlying simd_json error.
        source: serde_json::Error,
    },
}

impl Display for SensorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SensorError::UnknownSensor(name) => {
                write!(f, "Unknown sensor type: {}", name)
            }
            SensorError::Deserialization { sensor, source } => {
                write!(f, "Failed to deserialize sensor '{}': {}", sensor, source)
            }
        }
    }
}

impl std::error::Error for SensorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SensorError::UnknownSensor(_) => None,
            SensorError::Deserialization { source, .. } => Some(source),
        }
    }
}
