## Resync Cargo

If `target/` or `Cargo.lock` is missing after downloading or copying the project, run these from the folder containing `Cargo.toml`:

```bash
cargo build     # re-downloads dependencies and recreates target/
cargo run       # build (if needed) and run the program
```

If you get strange build errors or corrupted files:

```bash
cargo clean     # delete the target/ folder
cargo build     # rebuild from scratch
```
[Go to src folder](src/)

By click you will open the code file
Only `Cargo.toml`, `Cargo.lock`, and `src/` are required. If `Cargo.toml` or `src/` is missing, the download is incomplete and must be copied again.