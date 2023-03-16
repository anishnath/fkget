//! A simple library for downloading files using reqwest.

pub mod fk_get {

    use reqwest::Url;
    use rusoto_core::Region;
    use rusoto_s3::{GetObjectRequest, S3Client, S3};
    use std::io::{stdout, Write};
    use std::time::{Duration, Instant};
    use tokio::io::AsyncWriteExt;
    use tokio::{fs::File, io};
    use colored::*;

    pub async fn download_file(url: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("{} {} {}", "fkget".bright_green(), "downloading".bright_yellow(), format!("file {}.", url).bright_cyan());
        println!("{} If you encounter any issues, please contact {} for support.", "Note:".bold(), "https://8gwifi.org".underline());

        let url = Url::parse(url)?;
        let mut file_name = String::from("file.bin");
        let mut content_length: u64 = 0;

        if url.scheme() == "s3" {
            let key = url.path().trim_start_matches('/');
            let bucket = url.host().unwrap().to_string();
            println!("S3_Key {}, S3_Bucket  {}", format!(" {}.", key).green(), format!(" {}.", bucket).green());
            let s3_client = S3Client::new(Region::default());
            let mut object = s3_client
                .get_object(GetObjectRequest {
                    key: key.into(),
                    bucket: bucket.into(),
                    ..Default::default()
                })
                .await?;

            let body = object.body.take().expect("The object has no body");

            let mut body = body.into_async_read();

            file_name = url
                .path_segments()
                .and_then(|segments| segments.last())
                .unwrap_or("file.bin")
                .parse()
                .unwrap();

            let x = &file_name.to_string();

            let mut file = File::create(file_name).await?;
            io::copy(&mut body, &mut file).await?;

            println!("Downloaded file {}.", format!("file {}.", x).bright_cyan());

        } else {
            let mut response = reqwest::get(url.as_ref()).await?;
            file_name = url
                .path_segments()
                .and_then(|segments| segments.last())
                .unwrap_or("file.bin")
                .parse()
                .unwrap();
            content_length = response.content_length().unwrap_or(0);

            let start_time = Instant::now();
            let mut file = tokio::fs::File::create(&file_name).await?;
            let mut downloaded_size = 0;
            while let Some(chunk) = response.chunk().await? {
                let len = chunk.len();
                downloaded_size += len;
                file.write_all(&chunk).await?;
                let elapsed_time = start_time.elapsed();
                let elapsed_seconds = elapsed_time.as_secs_f64();
                let download_speed = downloaded_size as f64 / elapsed_seconds / 1_000_000.0;
                let remaining_size = content_length as f64 - downloaded_size as f64;
                //let remaining_time = remaining_size as f64 / download_speed;

                let remaining_time = if download_speed > 0.0 {
                    remaining_size as f64 / download_speed
                } else {
                    f64::INFINITY
                };

                print!(
                    "\rDownloaded {:.2} MB of {:.2} MB ({:.2}%) at {:.2} Mbps, {} remaining",
                    downloaded_size as f64 / 1_000_000.0,
                    content_length as f64 / 1_000_000.0,
                    downloaded_size as f64 / content_length as f64 * 100.0,
                    download_speed,
                    format_duration(Duration::from_secs(remaining_time as u64)),
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
        }

        Ok(())
    }

    fn format_duration(d: Duration) -> String {
        let secs = d.as_secs();
        let mins = secs / 60;
        let secs = secs % 60;
        let _hours = mins / 60;
        let mins = mins % 60;
        // if hours > 0 {
        //     format!("{}h{}m{}s", hours, mins, secs)
        // } else
        if mins > 0 {
            format!("{}m{}s", mins, secs)
        } else {
            format!("{}s", secs)
        }
    }
}
