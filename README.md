# Tracky

Tracky is a multi-crate Rust workspace consisting of:

- **tracky-core** — the core Rust library (scanning, metadata, future DB and sync logic)
- **tracky-cli** — a command-line interface built on top of tracky-core

## Build

```
cargo build
```

## Run

```
cargo run -p tracky-cli -- <path>
```

## Roadmap

- Add tag/metadata parsing  
- Add SQLite library database  
- Add sync action logs for iOS  
- Add artwork extraction  
- Add folder hashing and change detection  
