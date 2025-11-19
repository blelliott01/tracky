# Changelog

All notable changes to **Tracky** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

------------------------------------------------------------------------

## \[0.1.0\] - 2025-02-XX

### Added

-   Initial multi-crate Rust workspace.
-   `tracky-core` library crate with scanning engine.
-   `tracky-cli` binary crate with JSON output.
-   VS Code project configuration.
-   Basic model types (`Track`, `ScanResult`).
-   Initial file scanning logic using WalkDir.

### Planned

-   Tag parsing (m4a/mp3/flac).
-   SQLite database schema.
-   Artwork extraction.
-   Sync action log system.
-   Folder hashing + change detection.
