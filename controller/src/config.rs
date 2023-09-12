use std::{
  collections::HashMap,
  error::Error
};
use serde::{Serialize, Deserialize};
use sqlx::{
  postgres::PgConnection,
  Row
};

use crate::db::connect_to_db;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  name: String,
  author: String,
  #[serde(default)]
  description: String,
  stages: HashMap<String, String>,
  #[serde(skip)]
  db_connection: Option<PgConnection>
}

impl Config {
  async fn init(&mut self) -> Result<(), Box<dyn Error>> {
    if let Some(db) = connect_to_db().await {
      self.db_connection = Some(db.conn);
    } else {
      self.db_connection = None;
    }
    Ok(())
  }

  pub async fn push_tasks(&mut self) -> Result<(), Box<dyn Error>> {
    self.init().await?;
    if self.db_connection.is_none() {
      Err("Postgres database: no connection")?;
    }
    let q = sqlx::query(
        "
SELECT (job_count) FROM jobs WHERE name = $1 ORDER BY job_count DESC LIMIT 1;
        "
      )
      .bind(&self.name)
      .fetch_all(self.db_connection.as_mut().unwrap())
      .await?;
    
    let mut job_count = 1;
    if let Some(query_response) = q.first() {
      job_count = query_response.get::<i32, &str>("job_count") + 1;
    }
    let _ = sqlx::query(
        "
INSERT INTO jobs (name, job_count, author, description)
VALUES ($1, $2, $3, $4);
        "
      )
      .bind(&self.name)
      .bind(job_count)
      .bind(&self.author)
      .bind(&self.description)
      .execute(self.db_connection.as_mut().unwrap())
      .await?;

    for (key, stage_def) in self.stages.iter() {
      println!("{}: {}", key, stage_def);
      let stage_num = key.split('_').last().unwrap_or("");
      if !stage_num.is_empty() {
        sqlx::query(
            "
  INSERT INTO tasks (job_name, stage_number, definition)
  VALUES ($1, $2, $3);
            "
          )
          .bind(format!("{}_{}", self.name, job_count))
          .bind(stage_num)
          .bind(stage_def)
          .execute(self.db_connection.as_mut().unwrap())
          .await?;
      }
    };
    Ok(())
  }
}