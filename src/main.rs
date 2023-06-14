use lambda_http::{run, IntoResponse, Request, RequestExt};
use lambda_runtime::{service_fn, Error};

#[tracing::instrument()]
async fn say_hello(request: Request) -> Result<impl IntoResponse, Error> {
    tracing::info!(request = ?request);

    Ok(format!(
        "hello {}",
        request
            .query_string_parameters_ref()
            .and_then(|params| params.first("name"))
            .unwrap_or("stranger")
    ))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    run(service_fn(say_hello)).await
}
