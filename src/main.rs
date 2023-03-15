use reqwest::Url;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{stdout, Write};
use std::time::Instant;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn download_file(url: &Url) -> Result<(), Box<dyn std::error::Error>> {
    let mut response = reqwest::get(url.as_ref()).await?;
    let start_time = Instant::now();
    let file_name = url
        .path_segments()
        .and_then(|segments| segments.last())
        .unwrap_or("file.bin");
    let mut file = tokio::fs::File::create(file_name).await?;
    let mut downloaded_size = 0;
    while let Some(chunk) = response.chunk().await? {
        let len = chunk.len();
        downloaded_size += len;
        file.write_all(&chunk).await?;
        let elapsed_time = start_time.elapsed();
        let elapsed_seconds = elapsed_time.as_secs_f64();
        let download_speed = downloaded_size as f64 / elapsed_seconds / 1_000_000.0;
        print!(
            "\rDownloaded {:.2} MB at {:.2} Mbps",
            downloaded_size as f64 / 1_000_000.0,
            download_speed
        );
        stdout().flush()?;
    }
    let elapsed_time = start_time.elapsed();
    let elapsed_seconds = elapsed_time.as_secs_f64();
    let file_size = downloaded_size as f64;
    let download_speed = file_size / elapsed_seconds / 1_000_000.0;
    println!(
        "\nDownloaded {:.2} MB in {:.2} seconds ({:.2} Mbps)",
        file_size / 1_000_000.0,
        elapsed_seconds,
        download_speed
    );
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = match env::args().nth(1) {
        Some(url_str) => Url::parse(&url_str)?,
        None => Url::parse("https://speed.hetzner.de/100MB.bin")?,
    };
    download_file(&url).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_download_file() {
        let url = Url::parse("https://speed.hetzner.de/100MB.bin").unwrap();
        download_file(&url).await.unwrap();
        let file = File::open("100MB.bin").unwrap();
        let file_size = file.metadata().unwrap().len();
        assert_eq!(file_size, 104_857_600);
        std::fs::remove_file("100MB.bin").unwrap();
    }
}
