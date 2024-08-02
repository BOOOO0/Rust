use warp::http::StatusCode;
use crate::models::User;
use crate::user_queries::{get_all_users, find_user_by_id, create_user, update_user_by_id, delete_user_by_id};

pub async fn get_users(pool: sqlx::MySqlPool) -> Result<impl warp::Reply, warp::Rejection> {
    match get_all_users(&pool).await {
        Ok(users) => Ok(warp::reply::json(&users)),
        Err(_) => Err(warp::reject::not_found()),
    }
}

pub async fn get_user_by_ids(id: i32, pool: sqlx::MySqlPool) -> Result<impl warp::Reply, warp::Rejection> {
    match find_user_by_id(id, &pool).await {
        Ok(Some(user)) => Ok(warp::reply::json(&user)),
        Ok(None) => Err(warp::reject::not_found()),
        Err(_) => Err(warp::reject::not_found()),
    }
}

pub async fn create_users(new_user: User, pool: sqlx::MySqlPool) -> Result<impl warp::Reply, warp::Rejection> {
    match create_user(&new_user, &pool).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err(warp::reject::not_found()),
    }
}

pub async fn update_users(id: i32, updated_user: User, pool: sqlx::MySqlPool) -> Result<impl warp::Reply, warp::Rejection> {
    match update_user_by_id(id, &updated_user, &pool).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(warp::reject::not_found()),
    }
}

pub async fn delete_users(id: i32, pool: sqlx::MySqlPool) -> Result<impl warp::Reply, warp::Rejection> {
    match delete_user_by_id(id, &pool).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(warp::reject::not_found()),
    }
}

pub async fn health_check() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status("OK", warp::http::StatusCode::OK))
}