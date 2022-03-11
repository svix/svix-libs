// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use sea_orm::{DatabaseConnection, DbBackend, SqlxPostgresConnector};
use sqlx::postgres::PgPoolOptions;

use crate::cfg::Configuration;

pub mod models;

static MIGRATIONS: sqlx::migrate::Migrator = sqlx::migrate!();

async fn connect(cfg: &Configuration) -> sqlx::Pool<sqlx::Postgres> {
    tracing::debug!("DB: Initializing pool");
    if DbBackend::Postgres.is_prefix_of(&cfg.db_dsn) {
        PgPoolOptions::new().connect(&cfg.db_dsn).await.unwrap()
    } else {
        panic!("db_dsn format not recognized. {}", &cfg.db_dsn)
    }
}

#[cfg(not(debug_assertions))]
pub async fn init_db(cfg: &Configuration) -> DatabaseConnection {
    init_db_and_run_migrations(cfg).await
}

#[cfg(debug_assertions)]
pub async fn init_db(cfg: &Configuration) -> DatabaseConnection {
    SqlxPostgresConnector::from_sqlx_postgres_pool(connect(cfg).await)
}

pub async fn init_db_and_run_migrations(cfg: &Configuration) -> DatabaseConnection {
    let db = connect(cfg).await;
    MIGRATIONS.run(&db).await.unwrap();

    SqlxPostgresConnector::from_sqlx_postgres_pool(db)
}
