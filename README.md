# fkget

A simple library for downloading files using reqwest.

## Usage

fkget is a simple library for downloading files using reqwest. It is designed to be used in a multithreaded environment.

### Example

```rust
use fkget::fk_get;
#[tokio::main]
async fn main() {
    let url = "https://speed.hetzner.de/100MB.bin".to_string();
    fk_get::download_file(&url).await.unwrap();
}
```

### Downloading a file

```bash
fkget https://example.com/file.txt
```

## License

MIT
---