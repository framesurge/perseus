use crate::tags::db;
use crate::tags::models::CreateTags;
use std::io;

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};
use io::ErrorKind::NotFound;

#[utoipa::path(
    context_path = "/tags",
    responses(
        (status = 200, description = "Tag lists", body = [Tags]),
    )
)]
#[get("/")]
pub async fn tags(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::tags_list(&client).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

/// Create new Todo to shared in-memory storage.
///
/// Post a new `Todo` in request body as json to store it. Api will return
/// created `Todo` on success or `ErrorResponse::Conflict` if todo with same id already exists.
///
/// One could call the api with.
/// ```text
/// curl localhost:8080/todo -d '{"id": 1, "value": "Buy movie ticket", "checked": false}'
/// ```
#[utoipa::path(
    context_path = "/tags",
    request_body = CreateTags,
    responses(
        (status = 201, description = "Category Successfully added", body = Tags),
        (status = 409, description = "Category with id already exists", body = ServiceError)
    )
)]
#[post("/")]
pub async fn add_tags(
    local_object: web::Json<CreateTags>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::tags_add(&client, local_object.clone()).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

/// Get Category by given todo id.
///
/// Return found `Category` with status 200 or 404 not found if `Category` is not found from db.
#[utoipa::path(
    context_path = "/tags",
    responses(
        (status = 200, description = "get tag", body = Tags),
        (status = 404, description = "tag not found by id", body = ServiceError)
    ),
    params(
        ("id", description = "Unique tag Id")
    )
)]
#[get("/{id}")]
pub async fn get_tags(id_tags: web::Path<(i32,)>, db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::tags_id(&client, id_tags.0).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

/// Delete tag by given path variable id.
///
/// This endpoint needs `api_key` authentication in order to call. Api key can be found from README.md.
///
/// Api will delete todo from shared in-memory storage by the provided id and return success 200.
/// If storage does not contain `Todo` with given id 404 not found will be returned.
#[utoipa::path(
        context_path = "/tags",

    responses(
        (status = 200, description = "tag deleted successfully"),
        (status = 401, description = "Unauthorized to delete tag", body = ServiceError),
        (status = 404, description = "tag not found by id", body = ServiceError)
    ),
    params(
        ("id", description = "Unique storage id of tag")
    ),
    security(
        ("api_key" = [])
    )
)]
#[delete("/{id}")]
pub async fn delete_tags(tags_id: web::Path<(i32,)>, db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::tags_delete(&client, tags_id.0).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

/// Update Todo with given id.
///
/// This endpoint supports optional authentication.
///
/// Tries to update `Todo` by given id as path variable. If todo is found by id values are
/// updated according `TodoUpdateRequest` and updated `Todo` is returned with status 200.
/// If todo is not found then 404 not found is returned.
#[utoipa::path(
    context_path = "/tags",
    request_body = TodoUpdateRequest,
    responses(
        (status = 200, description = "Category updated successfully", body = CreateCategory),
        (status = 404, description = "Category not found by id", body = ServiceError)
    ),
    params(
        ("id", description = "Unique storage id of Category")
    ),
    security(
        (),
        ("api_key" = [])
    )
)]
#[patch("/{id}")]
pub async fn update_tags(
    id_tags: web::Path<(i32,)>,
    local_object: web::Json<CreateTags>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::tags_update(&client, id_tags.0, local_object.clone()).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(tags);
    cfg.service(add_tags);
    cfg.service(update_tags);
    cfg.service(get_tags);
    cfg.service(delete_tags);
}

// #[delete("/{id}")]
// pub async fn delete_author(id_author: web::Path<(i32,)>,  db_pool: web::Data<Pool>) -> impl Responder {
//     let res = format!("{:?},", id_author.0);
//     println!("{:#?}", res);
//     res
// }
