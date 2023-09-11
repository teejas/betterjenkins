use std::{
  env,
  collections::HashMap,
  error::Error
};
use serde::{Serialize, Deserialize};
use sqlx::{
  postgres::PgConnection,
  Connection,
  Row
};

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

pub async fn connect_to_db() -> Result<PgConnection, Box<dyn Error>> {
  Ok(PgConnection::connect(
    &format!("postgresql://{}:{}@{}/{}", 
      env::var("DB_USER").unwrap(),
      env::var("DB_PASSWORD").unwrap(), 
      env::var("DB_HOST").unwrap(), 
      env::var("DB_NAME").unwrap()
    )[..]
  ).await?)
}

impl Config {
  async fn init(&mut self) -> Result<(), Box<dyn Error>> {
    self.db_connection = Some(connect_to_db().await?);
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
      .execute(self.db_connection.as_mut().unwrap())
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
      .execute(self.db_connection.as_mut().unwrap())
      .await?;

    Ok(())
  }

  pub async fn push_tasks(&mut self) -> Result<(), Box<dyn Error>> {
    if self.db_connection.is_none() {
      self.init().await?;
    }
    let query_response = sqlx::query(
        "
SELECT (job_count) FROM jobs WHERE name = $1 ORDER BY job_count DESC LIMIT 1;
        "
      )
      .bind(&self.name)
      .fetch_one(self.db_connection.as_mut().unwrap())
      .await?;
    
    let job_count = query_response.get::<i32, &str>("job_count") + 1;
    println!("{}", job_count.to_string());
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