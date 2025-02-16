# Object Store Compression Bug Repro

This repo contains a minimal repro for a bug in the `object_store` crate where it fails on a missing `Content-Length` header when any of the compression features for `reqwest` are enabled (except for `gzip`).

Based on the prior [`object_store_gzip_repro`](https://github.com/phillipleblanc/object_store_gzip_repro) example used for reproducing https://github.com/apache/arrow-rs/issues/6842.

```bash
cargo test -- --nocapture
...
running 1 test
thread 'tests::repro' panicked at src/lib.rs:32:14:
called `Result::unwrap()` on an `Err` value: Generic { store: "HTTP", source: Header { source: MissingContentLength } }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test tests::repro ... FAILED
```

Removing the `zstd` feature from `reqwest` in `Cargo.toml` restores the correct behavior.
