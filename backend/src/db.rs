use sqlx::{Pool, Postgres};
use std::env;
use dotenvy::dotenv;

pub async fn connect_db() -> Result<Pool<Postgres>, sqlx::Error> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL non trouvé dans .env");
    let pool = Pool::<Postgres>::connect(&db_url).await?;
    Ok(pool)
}