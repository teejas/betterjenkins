use actix_web::{App, HttpServer};
use tokio::{self, signal, task};
use tokio_util::sync::CancellationToken;

mod run;
use run::run_executors;
mod server;

async fn shutdown<T>(
  token: CancellationToken, 
  run_handle: &mut task::JoinHandle<T>, 
  serve_handle: &mut task::JoinHandle<T>) {
    // Cancel the original token after a small delay
    tokio::spawn(async move {
      tokio::time::sleep(std::time::Duration::from_millis(10)).await;
      token.cancel();
    });

  // Wait for tasks to complete
  run_handle.await.unwrap();
  serve_handle.await.unwrap();
}

pub async fn start_threads() {
  println!("
Starting betterjenkins controller...
Press Ctrl+C to exit gracefully
  ");
  let token = CancellationToken::new();

  // Clone the token for use in another task
  let run_token = token.clone();

  // Task 1 - Wait for token cancellation or a long time
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
      shutdown(token, &mut run_thread, &mut serve_handle).await;
    },
    Err(err) => {
      eprintln!("Unable to listen for shutdown signal: {}", err);
      shutdown(token, &mut run_thread, &mut serve_handle).await;
    },
  }
}