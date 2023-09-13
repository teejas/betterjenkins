use std::{
  error::Error,
  path::PathBuf
};
use actix_files as fs;
use actix_web::{
  get, 
  post, 
  http::header::{ContentDisposition, DispositionType},
  web::Payload, 
  HttpRequest
};

use crate::config::Config;

fn get_index_html() -> fs::NamedFile {
  let path: PathBuf = PathBuf::from("web/index.html");
  fs::NamedFile::open(path).unwrap()
}

#[get("/")]
pub async fn index(_req: HttpRequest) -> Result<fs::NamedFile, actix_web::Error> {
  let file = get_index_html();
  Ok(file
      .use_last_modified(true)
      .set_content_disposition(ContentDisposition {
          disposition: DispositionType::Inline,
          parameters: vec![],
      }))
}

#[post("/")]
pub async fn process_upload(body: Payload) -> Result<fs::NamedFile, Box<dyn Error>> {
  let req = body.to_bytes().await?;
  let mut unencoded: bool = false;
  let lines: Vec<&str> = req
      .split(|&b| b == b'\r')
      .map(|line| {
          if let Ok(line_str) = std::str::from_utf8(line.strip_prefix(b"\n").unwrap_or(line)) {
              line_str
          } else {
              unencoded = true;
              ""
          }
      }).collect();

  if unencoded {
      eprintln!("File is not a YAML file or was not UTF-8 encoded");
  } else {
      let mut lines_iter = lines.iter();
      while let Some(line) = lines_iter.next() {
          if line.contains("Content-Type") {
              if !line.contains("yaml") {
                  eprintln!("File is not a YAML file")
              } else {
                  let _ = lines_iter.next().unwrap_or(&""); // blank line ""
                  let uploaded_file = lines_iter.next().unwrap_or(&"");
                  if uploaded_file == &"" {
                      eprintln!("Error reading file or file was empty");
                  } else {
                      let mut c: Config = serde_yaml::from_str(uploaded_file)?;
                      c.push_tasks().await?;
                  }
              }
          }
      }
  }
  let file = get_index_html();
  Ok(file
      .use_last_modified(true)
      .set_content_disposition(ContentDisposition {
          disposition: DispositionType::Inline,
          parameters: vec![],
      }))
}