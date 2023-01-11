use once_cell::sync::Lazy;
use sqlx;
use sqlx::mysql::MySqlPoolOptions;
use std::env;

struct MysqlConfig {
    host: String,
    port: String,
    user: String,
    password: String,
    database: String,
}

impl MysqlConfig {
    pub fn database_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.database
        )
    }
}

static CONFIG: Lazy<MysqlConfig> = Lazy::new(|| MysqlConfig {
    host: std::env::var("MYSQL_HOST").unwrap(),
    port: std::env::var("MYSQL_PORT").unwrap(),
    user: std::env::var("MYSQL_USER").unwrap(),
    password: std::env::var("MYSQL_PASSWORD").unwrap(),
    database: std::env::var("MYSQL_DB").unwrap(),
});

#[derive(Debug, sqlx::FromRow)]
pub struct Country {
    pub id: u64,
    pub code: String,
    pub value: String,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
    pub created_user_code: String,
    pub updated_at: sqlx::types::chrono::NaiveDateTime,
    pub updated_user_code: String,
}

pub async fn query() -> anyhow::Result<()> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&CONFIG.database_url())
        .await?;

    let row: (i64, i64) =
        sqlx::query_as("SELECT count(1) as count, count(1) as count1 FROM m_country")
            .bind(150_i64)
            .fetch_one(&pool)
            .await?;
    println!("{:?}", row);

    let rows = sqlx::query_as::<_, Country>("SELECT * FROM m_country")
        // .bind(150_i64)
        .fetch_all(&pool)
        .await
        .unwrap();

    println!("{:?}", rows.len());
    println!("{:?}", rows);

    Ok(())
}
