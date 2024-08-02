use crate::models::User;
use sqlx::MySqlPool;

pub async fn get_all_users(pool: &MySqlPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT id, name, email FROM users")
        .fetch_all(pool)
        .await
}

pub async fn find_user_by_id(id: i32, pool: &MySqlPool) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as::<_, User>("SELECT id, name, email FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(user)
}

pub async fn create_user(user: &User, pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
        .bind(&user.name)
        .bind(&user.email)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_user_by_id(id: i32, user: &User, pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET name = ?, email = ? WHERE id = ?")
        .bind(&user.name)
        .bind(&user.email)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_user_by_id(id: i32, pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
