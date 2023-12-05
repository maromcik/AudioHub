use chrono::Duration;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct AudiobookUser {
    pub user_id: Uuid,
    pub audiobook_id: Uuid,
    pub playback_chapter_id: Uuid,
    pub playback_position_in_chapter: Option<Duration>,
}
