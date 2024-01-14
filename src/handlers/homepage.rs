use crate::database::common::{DbCreate, DbReadMany, DbReadOne};
use crate::database::models::user::UserGetById;
use crate::database::repositories::user::repository::UserRepository;
use crate::error::AppError;
use crate::handlers::utilities::parse_user_id;
use crate::templates::index::IndexTemplate;
use actix_identity::Identity;
use actix_web::{get, web, HttpResponse};
use askama::Template;
use crate::database::models::audiobook::AudiobookSearch;
use crate::database::repositories::audiobook::repository::AudiobookRepository;


#[get("/")]
pub async fn index(
    identity: Option<Identity>,
    user_repo: web::Data<UserRepository>,
    book_repo: web::Data<AudiobookRepository>,
) -> Result<HttpResponse, AppError> {
    let books = book_repo
        .read_many(&AudiobookSearch::new(None, None, None,
                                         None, None, None,
                                         None, None, None,
                                         None, None)).await?;

    let template = match identity {
        None => IndexTemplate {
            username: "None".to_string(),
            logged_in: false,
            audiobooks: books,
        },
        Some(u) => {
            let user = user_repo
                .read_one(&UserGetById::new(&parse_user_id(u)?))
                .await?;
            IndexTemplate {
                username: user.name,
                logged_in: true,
                audiobooks: books,
            }
        }
    };
    let body = template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
