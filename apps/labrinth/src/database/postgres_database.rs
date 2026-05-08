use crate::util::date::APP_TZ_NAME;
use log::info;
use sqlx::Executor;
use sqlx::migrate::MigrateDatabase;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::{Connection, PgConnection, Postgres};
use std::time::Duration;

pub async fn connect() -> Result<PgPool, sqlx::Error> {
    info!("Initializing database connection");
    let database_url =
        dotenvy::var("DATABASE_URL").expect("`DATABASE_URL` not in .env");
    let pool = PgPoolOptions::new()
        .min_connections(
            dotenvy::var("DATABASE_MIN_CONNECTIONS")
                .ok()
                .and_then(|x| x.parse().ok())
                .unwrap_or(0),
        )
        .max_connections(
            dotenvy::var("DATABASE_MAX_CONNECTIONS")
                .ok()
                .and_then(|x| x.parse().ok())
                .unwrap_or(16),
        )
        .max_lifetime(Some(Duration::from_secs(60 * 60)))
        // 让所有连接默认使用 APP_TZ_NAME（北京时间），从而 CURRENT_DATE / DATE_TRUNC /
        // DATE() 等针对 timestamptz 的隐式时区转换自动按北京日聚合。
        // timestamptz 字段的存储/读取仍然是 UTC，由 sqlx + chrono::DateTime<Utc> 强类型保证。
        .after_connect(|conn, _meta| {
            Box::pin(async move {
                let stmt = format!("SET TIME ZONE '{APP_TZ_NAME}'");
                conn.execute(stmt.as_str()).await?;
                Ok(())
            })
        })
        .connect(&database_url)
        .await?;

    Ok(pool)
}
pub async fn check_for_migrations() -> Result<(), sqlx::Error> {
    let uri =
        dotenvy::var("DATABASE_URL").expect("`DATABASE_URL` 未在 .env 中设置");
    let uri = uri.as_str();
    if !Postgres::database_exists(uri).await? {
        info!("正在创建数据库...");
        Postgres::create_database(uri).await?;
    }

    info!("正在检查数据结构版本更新...");

    let mut conn: PgConnection = PgConnection::connect(uri).await?;
    sqlx::migrate!()
        .run(&mut conn)
        .await
        .expect("运行数据库迁移时出错！");

    Ok(())
}
