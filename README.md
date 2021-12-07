# rust-play
🎉 Hello Rust


## Configuring root Cargo.toml

```bash
vi Cargo.toml
```


```toml
[workspace]
members = [
  "packages/*",
]
```


## Creating the Second Package in the Workspace

```bash
cargo new packages/model --lib
cargo new packages/view --lib
cargo new packages/controller
```

## Run

```bash
cargo run
```
