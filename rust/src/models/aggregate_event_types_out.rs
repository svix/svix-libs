// this file is @generated
use serde::{Deserialize, Serialize};

use super::{
    background_task_status::BackgroundTaskStatus, background_task_type::BackgroundTaskType,
};

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct AggregateEventTypesOut {
    /// The QueueBackgroundTask's ID.
    pub id: String,

    pub status: BackgroundTaskStatus,

    pub task: BackgroundTaskType,
}

impl AggregateEventTypesOut {
    pub fn new(id: String, status: BackgroundTaskStatus, task: BackgroundTaskType) -> Self {
        Self { id, status, task }
    }
}
