use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{json, Value};

#[tracing::instrument()]
async fn say_hello(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, context) = event.into_parts();
    tracing::info!(event = ?event, context = ?context);

    let name = &event["name"].as_str().unwrap_or("stranger");

    Ok(json!({
        "message": format!("Hello, {name}!"),
    }))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    env_logger::init();

    lambda_runtime::run(service_fn(say_hello)).await
}
