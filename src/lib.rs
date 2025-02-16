#[cfg(test)]
mod tests {
    use std::future::IntoFuture as _;

    use object_store::{http::HttpBuilder, path::Path, ObjectStore};

    #[tokio::test]
    async fn repro() {
        let app = axum::Router::new()
            .route(
                "/example.txt",
                axum::routing::get(|| async { "Hello world!" }),
            )
            .layer(
                tower_http::compression::CompressionLayer::new()
                    .compress_when(tower_http::compression::predicate::SizeAbove::new(0)),
            );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();

        let server_address = listener.local_addr().unwrap();
        tokio::spawn(axum::serve(listener, app).into_future());

        let http_store = HttpBuilder::new()
            .with_url(format!("http://{server_address}"))
            .with_client_options(object_store::ClientOptions::new().with_allow_http(true))
            .build()
            .unwrap();

        let object = http_store
            .get(&Path::parse("example.txt").unwrap())
            .await
            .unwrap();
        println!("{:?}", object);
    }
}
