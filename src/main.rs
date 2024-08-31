use std::{fs::File, io::Write};

use actix_multipart::Multipart;
use futures_util::{StreamExt, TryStreamExt};
use actix_web::{http::header::ContentType, web, App, HttpResponse, HttpServer, Result};
use maud::{html, Markup};
use serde::Serialize;

#[derive(Serialize)]
pub struct JsonResponse {
    is_error: bool,
    message: String
}

async fn index() -> Result<Markup> {
    Ok(
        html!(
            html {

                head {
                    meta charset="utf-8" {}

                    title { "Uploader" }

                }

                body {

                    form class="challenge-upload-form" method="post" enctype="multipart/form-data" {
                        input type="file" name="challenge-file" id="fileToUpload" accept=".tar.gz" {}
                        button id="upload-challenge" { "upload" }
                    }

                }

                script src="/static/js/upload.js" {}

            }
        )
    )
}

async fn upload_handle(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
    while let Some(mut field) = payload.try_next().await? {
        let data_part = field.content_disposition().unwrap();
        if let Some(_) = data_part.get_filename() {

            let filepath = format!("x.tar.gz");

            let mut f = File::create(&filepath).unwrap();
            while let Some(chunk) = field.next().await {
                let data = chunk.unwrap();
                f.write_all(&data).unwrap();
            }
            let json_resp = JsonResponse {is_error: false, message: format!("{} ok", filepath)};
            let res = serde_json::to_string(&json_resp).unwrap();
            return Ok(HttpResponse::Ok().content_type(ContentType::json()).body(res))
        }
    }
    let json_resp = JsonResponse {is_error: true, message: "no file uploaded".to_string()};
    let res = serde_json::to_string(&json_resp).unwrap();
    Ok(HttpResponse::BadRequest().content_type(ContentType::json()).body(res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
        .service(actix_files::Files::new("/static", "./src/static"))
        .route("/", web::get().to(index))
        .route("/api/upload", web::post().to(upload_handle))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
