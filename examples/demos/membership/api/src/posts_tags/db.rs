use crate::posts_tags::{CreatePostsTags, PostsTags};
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

// CORE CRUD

//TODO configure .env for db shema name

// Decide wether to return id or return all fields from insert sql query . if return ID, insert id in function argument.
// shift id in db tables to the top so we can skip it when not needed

pub async fn posts_tags_add(
    client: &Client,
    selfobj: CreatePostsTags,
) -> Result<CreatePostsTags, io::Error> {
    let statement = client
        .prepare(
            "INSERT INTO public.posts_tags
   (post_id, tag_id)
    VALUES ($0, $1) RETURNING post_id, tag_id",
        )
        .await
        .unwrap();

    client
        .query(&statement, &[&selfobj.post_id, &selfobj.tag_id])
        .await
        .expect("Error creating posts_tags")
        .iter()
        .map(|row| CreatePostsTags::from_row_ref(row).unwrap())
        .collect::<Vec<CreatePostsTags>>()
        .pop()
        .ok_or(io::Error::new(
            io::ErrorKind::Other,
            "Error creating posts_tags tables",
        ))
}

// TODO populate fields

pub async fn posts_tags_list(client: &Client) -> Result<Vec<PostsTags>, io::Error> {
    let statement = client
        .prepare("select * from public.posts_tags order by id desc")
        .await
        .unwrap();

    let posts_tags_list = client
        .query(&statement, &[])
        .await
        .expect("Error getting author lists")
        .iter()
        .map(|row| PostsTags::from_row_ref(row).unwrap())
        .collect::<Vec<PostsTags>>();

    Ok(posts_tags_list)
}

pub async fn posts_tags_id(client: &Client, id_posts_tags: i32) -> Result<PostsTags, io::Error> {
    let statement = client
        .prepare("select * from public.posts_tags where id = $1")
        .await
        .unwrap();

    let maybe_posts_tags = client
        .query_opt(&statement, &[&id_posts_tags])
        .await
        .expect("Error adding posts_tags ")
        .map(|row| PostsTags::from_row_ref(&row).unwrap());

    match maybe_posts_tags {
        Some(posts_tags) => Ok(posts_tags),
        None => Err(io::Error::new(io::ErrorKind::NotFound, "Not found")),
    }
}

//TODO take into account ID position

pub async fn posts_tags_update(
    client: &Client,
    id: i32,
    mdl: CreatePostsTags,
) -> Result<(), io::Error> {
    let statement = client
        .prepare("update public.posts_tags set (post_id, tag_id) = ($0, $1) where id = $3")
        .await
        .unwrap();

    let result = client
        .execute(&statement, &[&mdl.post_id, &mdl.tag_id, &id])
        .await
        .expect("Error getting todo lists");

    match result {
        ref updated if *updated == 1 => Ok(()),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Failed to check list")),
    }
}

pub async fn posts_tags_delete(client: &Client, posts_tags_id: i32) -> Result<(), io::Error> {
    let statement = client
        .prepare("DELETE FROM public.posts_tags WHERE id = $1")
        .await
        .unwrap();

    client.execute(&statement, &[&posts_tags_id]).await.unwrap();
    Ok(())
}

// END OF CORE CRUD
