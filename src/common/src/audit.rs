use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Audit {
    created_by: Uuid,
    modified_by: Option<Uuid>,
    created: DateTime<Utc>,
    modified: Option<DateTime<Utc>>,
}
