use crate::posts::db;
use crate::posts::models::CreatePost;
use std::io;

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};
use io::ErrorKind::NotFound;

/// Get list of posts.
///
/// List postss from in-memory post store.
///
/// One could call the api endpoint with following curl.
/// ```text
/// curl localhost:8080/posts
/// ```
#[utoipa::path(
    context_path = "/posts",
    responses(
        (status = 200, description = "Post lists", body = [Post]),
    )
)]
#[get("/")]
pub async fn posts(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::post_list(&client).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

/// Create new Post to shared in-memory storage.
///
/// Post a new `Todo` in request body as json to store it. Api will return
/// created `Todo` on success or `ErrorResponse::Conflict` if todo with same id already exists.
///
/// One could call the api with.
/// ```text
/// curl localhost:8080/todo -d '{"id": 1, "value": "Buy movie ticket", "checked": false}'
/// ```
#[utoipa::path(
     context_path = "/posts",
    request_body = CreatePost,
    responses(
        (status = 201, description = "Category Successfully added", body = Post),
        (status = 409, description = "Category with id already exists", body = ServiceError)
    )
)]
#[post("/")]
pub async fn add_posts(
    local_object: web::Json<CreatePost>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::post_add(&client, local_object.clone()).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

/// Get Category by given todo id.
///
/// Return found `Category` with status 200 or 404 not found if `Category` is not found from db.
#[utoipa::path(
    context_path = "/posts",
    responses(
        (status = 200, description = "Post", body = Post),
        (status = 404, description = "Post not found by id", body = ServiceError)
    ),
    params(
        ("id", description = "Unique Post Id")
    )
)]
#[get("/{id}")]
pub async fn get_posts(id_posts: web::Path<(i32,)>, db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::post_id(&client, id_posts.0).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

/// Delete Post by given path variable id.
///
/// This endpoint needs `api_key` authentication in order to call. Api key can be found from README.md.
///
/// Api will delete todo from shared in-memory storage by the provided id and return success 200.
/// If storage does not contain `Todo` with given id 404 not found will be returned.
#[utoipa::path(
     context_path = "/posts",
    responses(
        (status = 200, description = "Post deleted successfully"),
        (status = 401, description = "Unauthorized to delete Post", body = ServiceError),
        (status = 404, description = "Post not found by id", body = ServiceError)
    ),
    params(
        ("id", description = "Unique storage id of Category")
    ),
    security(
        ("api_key" = [])
    )
)]
#[delete("/{id}")]
pub async fn delete_posts(posts_id: web::Path<(i32,)>, db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::post_delete(&client, posts_id.0).await;

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
     context_path = "/posts",
    request_body = CreatePost,
    responses(
        (status = 200, description = "Category updated successfully", body = Post),
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
pub async fn update_posts(
    id_posts: web::Path<(i32,)>,
    local_object: web::Json<CreatePost>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::post_update(&client, id_posts.0, local_object.clone()).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(posts);
    cfg.service(add_posts);
    cfg.service(update_posts);
    cfg.service(get_posts);
    cfg.service(delete_posts);
}

// #[delete("/{id}")]
// pub async fn delete_author(id_author: web::Path<(i32,)>,  db_pool: web::Data<Pool>) -> impl Responder {
//     let res = format!("{:?},", id_author.0);
//     println!("{:#?}", res);
//     res
// }
