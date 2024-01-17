use crate::database::models::Id;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq, Clone)]
pub struct Rating {
    pub id: Id,
    pub audiobook_id: Id,
    pub user_id: Id,
    pub rating: i16,
    pub review: Option<String>,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Default)]
pub struct RatingSearch {
    pub audiobook_id: Option<Id>,
    pub user_id: Option<Id>,
    pub min_rating: Option<i16>,
    pub max_rating: Option<i16>,
    pub review: Option<String>,
}

impl RatingSearch {
    #[must_use]
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        audiobook_id: Option<Id>,
        user_id: Option<Id>,
        min_rating: Option<i16>,
        max_rating: Option<i16>,
        review: Option<&str>,
    ) -> Self {
        let change_to_owned = |value: &str| Some(value.to_owned());
        Self {
            audiobook_id,
            user_id,
            min_rating,
            max_rating,
            review: review.and_then(change_to_owned),
        }
    }
}


#[derive(Debug, Clone, Deserialize)]
pub struct RatingsGetByBookId {
    pub audiobook_id: Id,
}

impl RatingsGetByBookId {
    pub fn new(id: Id) -> Self {
        Self {
            audiobook_id: id,
        }
    }
}


#[derive(Debug, Clone)]
pub struct RatingCreate {
    pub audiobook_id: Id,
    pub user_id: Id,
    pub rating: i16,
    pub review: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RatingUpdate {
    pub id: Id,
    pub rating: i16,
    pub review: Option<String>,
}

pub struct RatingGetById {
    pub id: Id,
}
