#[cfg(test)]
mod tests {
    use object_store::{http::HttpBuilder, path::Path, ObjectStore};

    #[tokio::test]
    async fn repro() {
        let http_store = HttpBuilder::new()
            .with_url("https://raw.githubusercontent.com/apache/arrow-rs/refs/heads/main")
            .build()
            .unwrap();

        let object = http_store
            .get(&Path::parse("LICENSE.txt").unwrap())
            .await
            .unwrap();
        println!("{:?}", object);
    }
}
