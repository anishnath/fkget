use std::env;
use url::Url;
use fkget::fk_get;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = match env::args().nth(1) {
        Some(url_str) => Url::parse(&url_str)?,
        None => Url::parse("https://speed.hetzner.de/100MB.bin")?,
    };
    fk_get::download_file(&url).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_download_file() {
        let url = Url::parse("https://speed.hetzner.de/100MB.bin").unwrap();
        fk_get::download_file(&url).await.unwrap();
        let file = File::open("100MB.bin").unwrap();
        let file_size = file.metadata().unwrap().len();
        assert_eq!(file_size, 104_857_600);
        std::fs::remove_file("100MB.bin").unwrap();
    }
}
