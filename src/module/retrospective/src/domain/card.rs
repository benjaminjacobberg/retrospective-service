use uuid::Uuid;

use common::audit::Audit;

pub(crate) struct Card {
    id: Uuid,
    user_id: Uuid,
    content: String,
    votes: i16,
    audit: Audit,
}
