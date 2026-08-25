use crate::posts::{CreatePost, Post};
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

// CORE CRUD

//TODO configure .env for db shema name

// Decide wether to return id or return all fields from insert sql query . if return ID, insert id in function argument.
// shift id in db tables to the top so we can skip it when not needed

pub async fn post_add(client: &Client, selfobj: CreatePost) -> Result<Post, io::Error> {
    let statement = client
        .prepare(
            "INSERT INTO public.posts
   (title, slug, summary, content)
    VALUES ($0, $1, $2, $3) RETURNING slug, title, summary, content",
        )
        .await
        .unwrap();

    client
        .query(
            &statement,
            &[
                &selfobj.slug,
                &selfobj.title,
                &selfobj.summary,
                &selfobj.content,
            ],
        )
        .await
        .expect("Error creating post")
        .iter()
        .map(|row| Post::from_row_ref(row).unwrap())
        .collect::<Vec<Post>>()
        .pop()
        .ok_or(io::Error::new(
            io::ErrorKind::Other,
            "Error creating post tables",
        ))
}

// TODO populate fields

pub async fn post_list(client: &Client) -> Result<Vec<Post>, io::Error> {
    let statement = client
        .prepare("select * from public.posts order by id desc")
        .await
        .unwrap();

    let content_list = client
        .query(&statement, &[])
        .await
        .expect("Error getting author lists")
        .iter()
        .map(|row| Post::from_row_ref(row).unwrap())
        .collect::<Vec<Post>>();

    Ok(content_list)
}

pub async fn post_id(client: &Client, id_post: i32) -> Result<Post, io::Error> {
    let statement = client
        .prepare("select * from public.posts where id = $1")
        .await
        .unwrap();

    let maybe_post = client
        .query_opt(&statement, &[&id_post])
        .await
        .expect("Error fetching post ")
        .map(|row| Post::from_row_ref(&row).unwrap());

    match maybe_post {
        Some(post) => Ok(post),
        None => Err(io::Error::new(io::ErrorKind::NotFound, "Not found")),
    }
}

pub async fn post_search(client: &Client, post_search: String) -> Result<Vec<Post>, io::Error> {
    let statement = client
        .prepare("select * from public.posts where title LIKE %$1%")
        .await
        .unwrap();

    let maybe_content = client
        .query(&statement, &[&post_search])
        .await
        .expect("Error fetching content ")
        .iter()
        .map(|row| Post::from_row_ref(&row).unwrap())
        .collect::<Vec<Post>>();
    Ok(maybe_content)
}

//TODO take into account ID position

pub async fn post_update(client: &Client, id: i32, mdl: CreatePost) -> Result<(), io::Error> {
    let statement = client.prepare("update public.posts set (slug, title, summary, content) = ($1, $2, $3, $4) where id = $5").await.unwrap();

    let result = client
        .execute(
            &statement,
            &[&mdl.slug, &mdl.title, &mdl.summary, &mdl.content],
        )
        .await
        .expect("Error updating post");

    match result {
        ref updated if *updated == 1 => Ok(()),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Failed to check list")),
    }
}

pub async fn post_delete(client: &Client, post_id: i32) -> Result<(), io::Error> {
    let statement = client
        .prepare("DELETE FROM public.posts WHERE id = $1")
        .await
        .unwrap();

    client.execute(&statement, &[&post_id]).await.unwrap();
    Ok(())
}

// END OF CORE CRUD
