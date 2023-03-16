# fkget

fkget download files from the internet and S3

To be Added support for GCS, Azure Blob Storage, and FTP

## Usage

fkget is a simple library for downloading files using reqwest. It is designed to be used in a multithreaded environment.

### Example

```rust
use fkget::fk_get;
#[tokio::main]
async fn main() {
    let url = "https://speed.hetzner.de/100MB.bin".to_string();
    fk_get::download_file(&url).await.unwrap();
    
    //Download S3 file 
    url =  "s3://bucket/key".to_string();
    fk_get::download_file(&url).await.unwrap();
}


```

### Downloading a file

```bash
fkget https://speed.hetzner.de/100MB.bin
```

### Downloading a file from S3 Endopoint

```bash
fkget  s3://bucket/jeykey
```

## License

MIT
---