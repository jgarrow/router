#[cfg(test)]
use http::Request;
#[cfg(test)]
use tower::BoxError;
#[cfg(test)]
use tracing::{info, Level};

#[cfg(test)]
use crate::graphql;
use crate::Plugin;

#[derive(Default)]
struct MyPlugin;
impl Plugin for MyPlugin {}

#[tokio::test]
async fn demo() -> Result<(), BoxError> {
    let _ = tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .try_init();

    let router = crate::builder().with_plugin(MyPlugin::default()).build();

    let response = router
        .call(
            Request::builder()
                .header("A", "HEADER_A")
                .body(graphql::Request {
                    body: "Hello1".to_string(),
                })
                .unwrap(),
        )
        .await?;
    info!("{:?}", response);

    Ok(())
}