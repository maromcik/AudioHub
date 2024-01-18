use askama::Template;
use crate::database::models::audiobook::AudiobookDetail;

#[derive(Template)]
#[template(path = "library.html")]
pub struct LibraryPageTemplate {
    pub audiobooks: Vec<AudiobookDetail>,
}

#[derive(Template)]
#[template(path = "library-content.html")]
pub struct LibraryContentTemplate {
    pub audiobooks: Vec<AudiobookDetail>,
}