use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Event<Data> {
    id: Uuid,
    aggregate_id: Uuid,
    aggregate_type: String,
    version: i128,
    data: Data,
    #[serde(rename = "type")]
    event_type: String,
    created: DateTime<Utc>,
}

trait EventConstructor<Data> {
    fn new(data: Data) -> Event<Data>;
}

trait EventAggregateConstructor<Data> {
    fn new(data: Data, aggregate_id: Uuid) -> Event<Data>;
}
