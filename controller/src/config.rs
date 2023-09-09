use std::{
  env,
  collections::HashMap,
  error::Error
};
use serde::{Serialize, Deserialize};
use sqlx::{
  postgres::PgConnection,
  Connection
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

impl Config {
  async fn init(&mut self) -> Result<(), Box<dyn Error>> {
      self.db_connection = Some(PgConnection::connect(
        &format!("postgresql://{}:{}@{}/{}", 
          env::var("DB_USER").unwrap(),
          env::var("DB_PASSWORD").unwrap(), 
          env::var("DB_HOST").unwrap(), 
          env::var("DB_NAME").unwrap()
        )[..]
      ).await?);
      sqlx::query(
          "
CREATE TABLE IF NOT EXISTS jobs (
  id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  name VARCHAR(255) NOT NULL,
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
      sqlx::query(
          "
INSERT INTO jobs (name, author, description)
VALUES ($1, $2, $3);
          "
      )
      .bind(&self.name)
      .bind(&self.author)
      .bind(&self.description)
      .execute(self.db_connection.as_mut().unwrap())
      .await?;

      for (key, stage_def) in self.stages.iter() {
          println!("{}: {}", key, stage_def);
          let stage_num = key.split("_").last().unwrap_or("");
          if stage_num != "" {
            sqlx::query(
                "
  INSERT INTO tasks (job_name, stage_number, definition)
  VALUES ($1, $2, $3);
                "
            )
            .bind(&self.name)
            .bind(stage_num)
            .bind(stage_def)
            .execute(self.db_connection.as_mut().unwrap())
            .await?;
          }
      };
      Ok(())
  }
}