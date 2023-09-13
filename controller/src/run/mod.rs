


use actix_web::{App, HttpServer};
use tokio::{self, signal, task};
use tokio_util::sync::CancellationToken;


use crate::workspace::WorkspaceManager;

mod server;
mod executors;
use executors::run_executors;

async fn shutdown<T>(token: CancellationToken, handles: Vec<&mut task::JoinHandle<T>>) {
    // Cancel the original token after a small delay
    tokio::spawn(async move {
      tokio::time::sleep(std::time::Duration::from_millis(10)).await;
      token.cancel();
    });

  // Wait for tasks to complete
  for handle in handles {
    handle.await.unwrap();
  }
}

pub async fn start_threads() {
  println!("
Starting betterjenkins controller...
Press Ctrl+C to exit gracefully
  ");
  let token = CancellationToken::new();

  // create thread which runs the workspace manager
  let ws_token = token.clone();
  let mut workspace = tokio::spawn(async move {
    if let Ok(mut wm) = WorkspaceManager::new().await {
      loop {
        tokio::select! {
          _ = ws_token.cancelled() => {
            break
          }
          _ = wm.create_workspace_dirs() => {
            let _ = wm.cleanup_workspace_dirs().await;
          }
        };
      }
    } else {
      eprintln!("Error setting up workspace manager.");
    }
  });

  // Clone the token for use in another task
  let run_token = token.clone();
  // Wait for token cancellation or a long time
  let mut run_thread = tokio::spawn(async move {
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
  let mut serve_handle = tokio::spawn(async move { serve.await.unwrap() });

  match signal::ctrl_c().await {
    Ok(()) => {
      println!("Exiting the betterjenkins controller...");
      shutdown(token, vec![&mut run_thread, &mut serve_handle, &mut workspace]).await;
    },
    Err(err) => {
      eprintln!("Unable to listen for shutdown signal: {}", err);
      shutdown(token, vec![&mut run_thread, &mut serve_handle, &mut workspace]).await;
    },
  }
}
