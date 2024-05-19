use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use rust_to_dtr;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let body = req.body().await?;
    let body = String::from_utf8(body.to_vec())?;
    println!("Request body: {}", body);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "message": "你好，世界",
              "body": body
            //   "rust_to_dtr_version": rust_to_dtr::version(),
            })
            .to_string()
            .into(),
        )?)
}