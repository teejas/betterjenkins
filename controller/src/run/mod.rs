use actix_web::{App, HttpServer};
use std::error::Error;
use tokio::{self, signal, task};
use tokio_util::sync::CancellationToken;

use crate::workspace::WorkspaceManager;
use crate::db::{connect_to_db, DBConn};

mod server;
mod executors;
use executors::run_executors;

pub struct Run {
  token: CancellationToken,
  db: DBConn
}

impl Run {
  pub async fn new() -> Result<Run, Box<dyn Error + Send + Sync>>{
    println!("
  Starting betterjenkins controller...
  Press Ctrl+C to exit gracefully
    ");

    // try to connect to the db and minio server, if either fails exit immediately
    if let Some(try_db) = connect_to_db().await {
      let token = CancellationToken::new();

      Ok(Run {
        token,
        db: try_db
      })
    } else {
      Err("Error connecting to database.")?
    }
  }

  pub async fn run(&mut self) {
    // create thread which runs the workspace manager
    let ws_token = self.token.clone();
    let run_token = self.token.clone();
    let mut workspace_t = tokio::spawn(async move {
      let mut wm_opt: Option<WorkspaceManager> = None;
      loop {
        tokio::select! {
          _ = ws_token.cancelled() => {
            break
          }
          _ = tokio::time::sleep(std::time::Duration::from_secs(5)) => {
            wm_opt = match wm_opt {
              Some(mut wm) => {
                let _ = wm.create_workspace_dirs().await;
                let _ = wm.cleanup_workspace_dirs().await;
                Some(wm)
              },
              None => {
                if let Ok(mut new_wm) = WorkspaceManager::new().await {
                  Some(new_wm)
                } else {
                  eprintln!("Error setting up workspace manager.");
                  None
                }
              }
            }
          }
        };
      }
    });

    // Clone the token for use in another task
    // Wait for token cancellation or a long time
    let mut run_executors_t = tokio::spawn(async move {
      let mut tasks_count: i64 = 0;
      loop {
        tokio::select! {
          _ = run_token.cancelled() => {
            break
          }
          ret_count = run_executors(tasks_count) => {
            tasks_count = ret_count
          }
        }
      }
    });

    let serve = HttpServer::new(|| {
      App::new()
        .service(server::index)
        .service(server::process_upload)
    })
      .bind(("0.0.0.0", 8080))
      .unwrap()
      .run();
    let mut server_t = tokio::spawn(async move { serve.await.unwrap() });
    
    match signal::ctrl_c().await {
      Ok(()) => {
        println!("Exiting the betterjenkins controller...");
        self.shutdown(vec![workspace_t, run_executors_t, server_t]);
      },
      Err(err) => {
        eprintln!("Unable to listen for shutdown signal: {}", err);
        self.shutdown(vec![workspace_t, run_executors_t, server_t]);
      },
    }
  }

  async fn shutdown<T>(&mut self, handles: Vec<task::JoinHandle<T>>) {
    // Cancel the original token after a small delay
    let token: CancellationToken = self.token.clone();
    tokio::spawn(async move {
      tokio::time::sleep(std::time::Duration::from_millis(10)).await;
      token.cancel();
    });

    // Wait for tasks to complete
    for handle in handles {
      handle.await.unwrap();
    }
    // (&mut self.run_executors_t).await.unwrap();
    // (&mut self.workspace_t).await.unwrap();
    // (&mut self.server_t).await.unwrap();
  }
}