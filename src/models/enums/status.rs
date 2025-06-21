use serde::{Deserialize, Serialize};

/// Status of a task or operation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Queued,
    Running,
    Finished,
    Terminated,
    Error,
}

impl Default for Status {
    fn default() -> Self {
        Self::Queued
    }
}
