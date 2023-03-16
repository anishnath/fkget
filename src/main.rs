use std::env;
use fkget::fk_get;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = match env::args().nth(1) {
        Some(url_str) => url_str,
        None => "https://speed.hetzner.de/100MB.bin".to_string(),
    };
    fk_get::download_file(&url).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use super::*;
    #[tokio::test]
    async fn test_download_file() {
        let url = "https://speed.hetzner.de/100MB.bin".to_string();
        fk_get::download_file(&url).await.unwrap();
        let file = File::open("100MB.bin").unwrap();
        let file_size = file.metadata().unwrap().len();
        assert_eq!(file_size, 104_857_600);
        std::fs::remove_file("100MB.bin").unwrap();
    }
}
