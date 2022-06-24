use crate::domain::card::Card;
use common::audit::Audit;
use std::iter::Map;
use uuid::Uuid;

pub(crate) enum Phase {
    Think,
    Group,
    Vote,
    Discuss,
}

pub(crate) enum Column {
    Start,
    Stop,
    Continue,
}

pub(crate) struct Board {
    id: Uuid,
    team_id: Uuid,
    lanes: Map<Column, Vec<Card>>,
    audit: Audit,
}
