use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    Form,
};
use serde::Deserialize;
use sqlx::{
    types::time::{OffsetDateTime, PrimitiveDateTime},
    PgPool,
};
use validator::Validate;

use crate::{
    common::{http, HtmlTemplate},
    model::movie_repo::{self, Movie, Status},
};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

#[derive(Template)]
#[template(path = "list.html")]
struct MovieListTemplate {}

#[derive(Template)]
#[template(path = "list_row.html")]
struct MovieRowTemplate {
    pub list: Vec<MovieListDetail>,
}

#[derive(Template)]
#[template(path = "edit.html")]
struct EditTemplate {
    action: String,
    message: Vec<String>,
    movie: EditModel,
}

#[derive(Deserialize)]
pub struct ListModel {
    pub name: String,
}

#[derive(Deserialize, Validate)]
pub struct EditModel {
    pub id: String,
    #[validate(
        length(min = 1, message = "電影名稱必填"),
        length(max = 10, message = "電影名稱勿超過10個字")
    )]
    pub name: String,

    pub status: String,
    #[validate(
        length(min = 1, message = "電影介紹必填"),
        length(max = 100, message = "電影介紹勿超過100個字")
    )]
    pub description: String,
}

struct MovieListDetail {
    pub id: i32,
    pub name: String,
    pub status: String,
}

pub async fn index(State(_pool): State<PgPool>) -> impl IntoResponse {
    let index = IndexTemplate {};
    return HtmlTemplate(index);
}

pub async fn list(State(_pool): State<PgPool>) -> impl IntoResponse {
    let list = MovieListTemplate {};
    return HtmlTemplate(list);
}

pub async fn all_list(State(pool): State<PgPool>) -> impl IntoResponse {
    let list = movie_repo::find_all(pool).await;
    let list = list
        .into_iter()
        .map(|dt| MovieListDetail {
            id: dt.id.unwrap(),
            name: dt.name.unwrap_or(String::from("")),
            status: Status::to_status_string(dt.status.unwrap_or(String::from(""))),
        })
        .collect();
    let list = MovieRowTemplate { list };
    return HtmlTemplate(list);
}

pub async fn search(
    State(pool): State<PgPool>,
    Form(list_model): Form<ListModel>,
) -> impl IntoResponse {
    let list = movie_repo::find_by_name(pool, list_model.name).await;
    let list = list
        .into_iter()
        .map(|dt| MovieListDetail {
            id: dt.id.unwrap(),
            name: dt.name.unwrap_or(String::from("")),
            status: Status::to_status_string(dt.status.unwrap_or(String::from(""))),
        })
        .collect();
    let list = MovieRowTemplate { list };
    return HtmlTemplate(list);
}

pub async fn add() -> impl IntoResponse {
    let movie = EditModel {
        id: "".to_string(),
        name: "".to_string(),
        status: "0".to_string(),
        description: "".to_string(),
    };
    let edit = EditTemplate {
        action: "A".to_string(),
        message: Vec::new(),
        movie,
    };
    return HtmlTemplate(edit);
}

pub async fn insert(State(pool): State<PgPool>, Form(edit_model): Form<EditModel>) -> Response {
    match validate_edit_model(&edit_model, &"".to_string(), "A".to_string()) {
        Some(res) => return res,
        None => (),
    }

    let now_odt = OffsetDateTime::now_utc();
    let now_pdt = PrimitiveDateTime::new(now_odt.date(), now_odt.time());
    let movie = Movie {
        id: None,
        name: Some(edit_model.name),
        status: Some(edit_model.status),
        description: Some(edit_model.description),
        created_at: Some(now_pdt),
        updated_at: None,
    };
    movie_repo::insert(pool, movie).await;
    return Redirect::to("/list").into_response();
}

pub async fn update(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
    Form(edit_model): Form<EditModel>,
) -> Response {
    match validate_edit_model(&edit_model, &id, "E".to_string()) {
        Some(res) => return res,
        None => (),
    }

    match id.parse::<i32>() {
        Ok(id) => {
            let now_odt = OffsetDateTime::now_utc();
            let now_pdt = PrimitiveDateTime::new(now_odt.date(), now_odt.time());
            let movie = Movie {
                id: Some(id),
                name: Some(edit_model.name),
                status: Some(edit_model.status),
                description: Some(edit_model.description),
                created_at: None,
                updated_at: Some(now_pdt),
            };
            movie_repo::update(pool, movie).await;
            return Redirect::to("/list").into_response();
        }
        Err(_) => HtmlTemplate(http::PageErrorTemplate {}).into_response(),
    }
}

pub async fn delete(State(pool): State<PgPool>, Path(id): Path<String>) -> impl IntoResponse {
    match id.parse::<i32>() {
        Ok(id) => {
            if movie_repo::delete(pool, id).await > 0 {
                StatusCode::OK
            } else {
                StatusCode::NOT_FOUND
            }
        }
        Err(_) => StatusCode::NOT_FOUND,
    }
}

pub async fn edit(State(pool): State<PgPool>, Path(id): Path<String>) -> Response {
    match id.parse::<i32>() {
        Ok(id) => {
            let movie = movie_repo::find_by_id(pool, id).await;
            match movie {
                Some(movie) => {
                    let movie = EditModel {
                        id: movie.id.unwrap().to_string(),
                        name: movie.name.unwrap(),
                        status: movie.status.unwrap(),
                        description: movie.description.unwrap(),
                    };
                    let edit = EditTemplate {
                        action: "E".to_string(),
                        movie,
                        message: Vec::new(),
                    };
                    return HtmlTemplate(edit).into_response();
                }
                None => HtmlTemplate(http::Page404Template {}).into_response(),
            }
        }
        Err(_) => HtmlTemplate(http::Page404Template {}).into_response(),
    }
}

pub fn validate_edit_model(
    edit_model: &EditModel,
    id: &String,
    action: String,
) -> Option<Response> {
    match edit_model.validate() {
        Ok(_) => None,
        Err(e) => {
            let mut message = Vec::new();
            e.field_errors()
                .into_iter()
                .flat_map(|error| error.1)
                .for_each(|error| {
                    message.push(error.message.as_ref().unwrap().to_string());
                });

            let movie = EditModel {
                id: id.clone(),
                name: edit_model.name.clone(),
                status: edit_model.status.clone(),
                description: edit_model.description.clone(),
            };
            let edit = EditTemplate {
                action: action,
                movie,
                message,
            };

            return Some(HtmlTemplate(edit).into_response());
        }
    }
}
