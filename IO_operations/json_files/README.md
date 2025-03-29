# Working with JSON files in Rust

## Serde package
[Serde](https://serde.rs/) stands for Serialization-Deserialization and it's the
preferable way to work with serialization in Rust.

### Serde JSON
Serde can work with a myriad standard formats like YAML, TOML, JSON, BSON, CSV,
S-expression, and even Pickle among others.

Particularly, working with JSON will require to add `serde` and `serd_json` to
the depedency list.

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

We use the `serde` crate to allow `#[derive(Serialize, Deserialize)]` macro to
allow new structs to read from and write to the chosen format.

### Reading JSON with `serde_json`
There are three possible ways to read json objects:
1. Directly from a string with `serde_json::from_str`.
2. From a `&[u8]` slice with `serde_json::from_slice`.
3. From a file or any other `io::Read` streamers like TCP with
   `serde_json::from_reader`.

### Writing JSON with `serde_json`
Similarly to the reading case, there are three possible ways to write json
objects:
1. Directly to a string with `serde_json::to_string`.
2. To a vector `Vec<u8>` with `serde_json::to_vec`.
3. To a file or any other `io::Write` streamers like TCP with
   `serde_json::to_writer`.