use crate::database::models::Id;
use chrono::{DateTime, Utc};

use sqlx::postgres::types::PgInterval;

#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct Audiobook {
    pub id: Id,
    // --------------
    pub name: String,
    pub author_id: Id,
    pub genre_id: Id,
    pub price_dollars: i32,
    pub price_cents: i32,
    pub length: PgInterval,
    pub file_path: String,
    pub stream_count: i64,
    pub overall_rating: i16,
    pub thumbnail: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct AudiobookSearch {
    pub name: Option<String>,
    pub author_id: Option<Id>,
    pub genre_id: Option<Id>,
    pub min_price_dollars: Option<i32>,
    pub max_price_dollars: Option<i32>,
    pub min_length: Option<PgInterval>,
    pub max_length: Option<PgInterval>,
    pub min_stream_count: Option<i64>,
    pub max_stream_count: Option<i64>,
    pub min_overall_rating: Option<i16>,
    pub max_overall_rating: Option<i16>,
}

#[derive(Debug, Clone)]
pub struct AudiobookCreate {
    pub name: String,
    pub author_id: Id,
    pub genre_id: Id,
    pub price_dollars: i32,
    pub price_cents: i32,
    pub length: PgInterval,
    pub file_path: String,
    pub stream_count: i64,
    pub overall_rating: i16,
    pub thumbnail: String,
    pub description: String,
}

impl AudiobookCreate {
    #[must_use]
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: &str,
        author_id: &Id,
        genre_id: &Id,
        price_dollars: &i32,
        price_cents: &i32,
        length: &PgInterval,
        file_path: &str,
        stream_count: &i64,
        overall_rating: &i16,
        thumbnail: &str,
        description: &str,
    ) -> Self {
        Self {
            name: name.to_owned(),
            author_id: *author_id,
            genre_id: *genre_id,
            price_dollars: *price_dollars,
            price_cents: *price_cents,
            length: length.clone(),
            file_path: file_path.to_owned(),
            stream_count: *stream_count,
            overall_rating: *overall_rating,
            thumbnail: thumbnail.to_owned(),
            description: description.to_owned(),
        }
    }
}

pub struct AudiobookUpdate {
    pub id: Id,
    pub name: Option<String>,
    pub author_id: Option<Id>,
    pub genre_id: Option<Id>,
    pub price_dollars: Option<i32>,
    pub price_cents: Option<i32>,
    pub length: Option<PgInterval>,
    pub file_path: Option<String>,
    pub stream_count: Option<i64>,
    pub overall_rating: Option<i16>,
    pub thumbnail: Option<String>,
    pub description: Option<String>,
}

impl AudiobookUpdate {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: &Id,
        name: Option<&str>,
        author_id: Option<&Id>,
        genre_id: Option<&Id>,
        price_dollars: Option<&i32>,
        price_cents: Option<&i32>,
        length: Option<&PgInterval>,
        file_path: Option<&str>,
        stream_count: Option<&i64>,
        overall_rating: Option<&i16>,
        thumbnail: Option<&str>,
        description: Option<&str>,
    ) -> Self {
        let change_to_owned = |value: &str| Some(value.to_owned());
        Self {
            id: *id,
            name: name.and_then(change_to_owned),
            author_id: author_id.copied(),
            genre_id: genre_id.copied(),
            price_dollars: price_dollars.copied(),
            price_cents: price_cents.copied(),
            length: length.cloned(),
            file_path: file_path.and_then(change_to_owned),
            stream_count: stream_count.copied(),
            overall_rating: overall_rating.copied(),
            thumbnail: thumbnail.and_then(change_to_owned),
            description: description.and_then(change_to_owned),
        }
    }

    #[must_use]
    pub const fn update_fields_none(&self) -> bool {
        self.name.is_none()
            && self.author_id.is_none()
            && self.genre_id.is_none()
            && self.price_dollars.is_none()
            && self.price_cents.is_none()
            && self.length.is_none()
            && self.file_path.is_none()
            && self.stream_count.is_none()
            && self.overall_rating.is_none()
            && self.description.is_none()
            && self.thumbnail.is_none()
    }
}

#[derive(Debug, Clone)]
pub struct AudiobookDelete {
    pub id: Id,
}

impl AudiobookDelete {
    #[must_use]
    #[inline]
    pub fn new(id: &Id) -> Self {
        Self { id: *id }
    }
}

pub struct AudiobookGetById {
    pub id: Id,
}

impl AudiobookGetById {
    #[must_use]
    #[inline]
    pub const fn new(id: &Id) -> Self {
        Self { id: *id }
    }
}
