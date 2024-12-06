# Object Store Gzip Bug Repro

This repo contains a minimal repro for a bug in the `object_store` crate where it fails on a missing `Content-Length` header when the `gzip` feature for `reqwest` is enabled.

```bash
cargo test -- --nocapture
...
running 1 test
thread 'tests::repro' panicked at src/lib.rs:15:14:
called `Result::unwrap()` on an `Err` value: Generic { store: "HTTP", source: Header { source: MissingContentLength } }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test tests::repro ... FAILED
```

Removing the `gzip` feature from `reqwest` in `Cargo.toml` restores the correct behavior.
