use std::{
  env,
  error::Error
};
use sqlx::{
  postgres::PgConnection,
  Connection
};

pub struct DBConn {
  pub conn: PgConnection
}

impl DBConn {
  pub async fn new() -> Result<DBConn, Box<dyn Error>> {
    let mut db = PgConnection::connect(
      &format!("postgresql://{}:{}@{}/{}", 
        env::var("DB_USER").unwrap(),
        env::var("DB_PASSWORD").unwrap(), 
        env::var("DB_HOST").unwrap(), 
        env::var("DB_NAME").unwrap()
      )[..]
    ).await?;
    sqlx::query(
      "
CREATE TABLE IF NOT EXISTS jobs (
id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
name VARCHAR(255) NOT NULL,
job_count INTEGER NOT NULL,
author VARCHAR(255) NOT NULL,
description VARCHAR(255)
);
      "
    )
    .execute(&mut db)
    .await?;

    sqlx::query(
        "
  CREATE TABLE IF NOT EXISTS tasks (
  id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  job_name VARCHAR(255) NOT NULL,
  stage_number VARCHAR(255) NOT NULL,
  definition VARCHAR(255) NOT NULL
  );
        "
      )
      .execute(&mut db)
      .await?;

    Ok(DBConn { 
      conn: db
    })
  }
}