#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

use rocket::http::Status;
use rocket::response::{content, Stream};
use rocket::tokio::io::AsyncRead;

#[get("/")]
async fn root() -> Result<content::Plain<Stream<impl AsyncRead>>, Status> {
    use rocket::futures::TryStreamExt as _;

    let result = reqwest::get("https://google.com")
        .await
        .map_err(|_| Status::InternalServerError)?;

    let reader = tokio_util::io::StreamReader::new(
        result
            .bytes_stream()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)),
    );

    Ok(content::Plain(Stream::from(reader)))
}

#[get("/async-stream")]
async fn get_async_stream() -> Result<content::Plain<Stream<impl AsyncRead>>, Status> {
    use tokio_stream::StreamExt as _;

    let result = reqwest::get("https://google.com")
        .await
        .map_err(|_| Status::InternalServerError)?;

    let mut bytes = result.bytes_stream();

    let reader = tokio_util::io::StreamReader::new(async_stream::stream! {
        while let Some(chunk) = bytes.next().await {
            yield chunk.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e));
        }
    });

    Ok(content::Plain(Stream::from(reader)))
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![root, get_async_stream])
}
