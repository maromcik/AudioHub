use std::convert::identity;
use crate::authorized;
use crate::database::models::Id;
use crate::error::AppError;
use actix_identity::Identity;
use actix_web::http::header::LOCATION;
use actix_web::{get, post, web, HttpResponse, HttpRequest, delete};
use askama::Template;
use serde::Deserialize;

use crate::database::models::rating::{ RatingCreate, RatingSearch, UserRatingDisplay};
use crate::database::repositories::rating::repository::RatingRepository;
use crate::forms::rating::RatingCreateForm;

use crate::handlers::utilities::parse_user_id;
use crate::templates::rating::{AudiobookRatingsTemplate, DeletedRatingTemplate, MyRatingTemplate, UserRatingTemplate};

#[get("/create/form")]
pub async fn create_rating_form(request: HttpRequest, identity: Option<Identity>) -> Result<HttpResponse, AppError> {
    authorized!(identity, request.path());
    todo!()
    // let template = ...
    // Ok(HttpResponse::Ok().content_type("text/html").body(template.render()?))
}

#[post("/audiobook/{book_id}")]
pub async fn create_rating(
    request: HttpRequest,
    identity: Option<Identity>,
    rating_repo: web::Data<RatingRepository>,
    path: web::Path<Id>,
    form: web::Form<RatingCreateForm>,
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity, request.path());
    let user_id = parse_user_id(identity)?;
    let audiobook_id = path.into_inner();
    let rating = rating_repo.create_or_update_displayed_rating(&RatingCreate {
            audiobook_id,
            user_id,
            rating: form.rating,
            review: form.review.to_owned(),
        })
        .await?;

    let template = MyRatingTemplate { rating };
    Ok(HttpResponse::Ok().content_type("text/html").body(template.render()?))
}
#[derive(Deserialize)]
struct OffsetQuery {
    offset: i32,
}

/// returns DISPLAYED_RATINGS_COUNT ratings transformed to html from query param offset, only returns reviews that
/// do not belong to the querying user
#[get("/audiobook/{id}")]
pub async fn get_ratings_by_audiobook(
    request: HttpRequest,
    identity: Option<Identity>,
    rating_repo: web::Data<RatingRepository>,
    path: web::Path<(Id,)>,
    query: web::Query<OffsetQuery>
) -> Result<HttpResponse, AppError> {
    let identity = authorized!(identity, request.path());
    let user_id = parse_user_id(identity)?;

    let search_params = RatingSearch::new(Some(path.into_inner().0),None,None,None,None,Some(query.offset));
    let ratings : Vec<UserRatingDisplay> = rating_repo
        .get_ratings_display(&search_params)
        .await?;
    let ratings : Vec<UserRatingDisplay> = ratings.into_iter()
        .filter(|rating| rating.user_id != user_id).collect();

    if ratings.len() == 0 {
        return Ok(HttpResponse::PreconditionFailed().finish());
    }
    let template = AudiobookRatingsTemplate {ratings,};
    Ok(HttpResponse::Ok().content_type("text/html").body(template.render()?))
}

#[delete("/audiobook/{id}")]
pub async fn remove_rating_for_audiobook(
    identity: Option<Identity>,
    rating_repo: web::Data<RatingRepository>,
    path: web::Path<(Id,)>,
    request: HttpRequest,
) -> Result<HttpResponse, AppError> {
    authorized!(identity, request.path());

    let _ = rating_repo.delete_rating_for_book(&path.into_inner().0).await?;

    let template = DeletedRatingTemplate {};
    Ok(HttpResponse::Ok().content_type("text/html").body(template.render()?))
}

#[get("/audiobook/{id}/my-rating")]
pub async fn get_my_rating(
    identity: Option<Identity>,
    rating_repo: web::Data<RatingRepository>,
    path: web::Path<(Id,)>,
    request: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let user = authorized!(identity, request.path());
    let user_id = parse_user_id(user)?;
    let search_params = RatingSearch::new(Some(path.into_inner().0),Some(user_id),None,None,None,Some(0));
    let ratings : Vec<UserRatingDisplay> = rating_repo
        .get_ratings_display(&search_params)
        .await?;

    if ratings.len() != 1 {
        return Ok(HttpResponse::PreconditionFailed().finish());
    }
    let template = MyRatingTemplate {rating: ratings[0].to_owned()};
    Ok(HttpResponse::Ok().content_type("text/html").body(template.render()?))
}
