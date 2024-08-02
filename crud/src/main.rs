use warp::Filter;
use sqlx::mysql::MySqlPoolOptions;
mod handlers;
mod models;
mod user_queries;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    let get_all = warp::path("get_users")
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(handlers::get_users);

    let create = warp::path("create_users")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool.clone()))
        .and_then(handlers::create_users);

    let get_one = warp::path!("get_users" / i32)
        .and(warp::get())
        .and(with_db(pool.clone()))
        .and_then(handlers::get_user_by_ids);

    let update = warp::path!("update_users" / i32)
        .and(warp::put())
        .and(warp::body::json())
        .and(with_db(pool.clone()))
        .and_then(handlers::update_users);

    let delete = warp::path!("delete_users" / i32)
        .and(warp::delete())
        .and(with_db(pool.clone()))
        .and_then(handlers::delete_users);

    let health_check = warp::path!("health")
        .and(warp::get())
        .and_then(handlers::health_check);

    let routes = get_all.or(create).or(get_one).or(update).or(delete).or(health_check);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_db(pool: sqlx::mysql::MySqlPool) -> impl warp::Filter<Extract = (sqlx::mysql::MySqlPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

