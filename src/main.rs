use std::fs::File;
use std::io::{stdout, Write, Read};
use std::time::{Instant};
use reqwest::Url;
use std::error::Error;
use std::env;

pub fn download_file(url: &Url) -> Result<(), Box<dyn std::error::Error>> {
    let mut response = reqwest::blocking::get(url.as_ref())?;
    let start_time = Instant::now();
    let file_name = url.path_segments().and_then(|segments| segments.last()).unwrap_or("file.bin");
    let mut file = File::create(file_name)?;
    let mut downloaded_size = 0;
    let mut buf = [0; 8192];
    loop {
        let len = response.read(&mut buf)?;
        if len == 0 {
            break;
        }
        downloaded_size += len;
        file.write_all(&buf[..len])?;
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

fn main() -> Result<(), Box<dyn Error>> {
    let url = match env::args().nth(1) {
        Some(url_str) => Url::parse(&url_str)?,
        None => Url::parse("https://speed.hetzner.de/100MB.bin")?,
    };
    download_file(&url)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_file() {
        let url = Url::parse("https://speed.hetzner.de/100MB.bin").unwrap();
        download_file(&url).unwrap();
        let file = File::open("100MB.bin").unwrap();
        let file_size = file.metadata().unwrap().len();
        assert_eq!(file_size, 104_857_600);
        std::fs::remove_file("100MB.bin").unwrap();
    }
}
