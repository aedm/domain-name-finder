use async_compression::tokio::bufread::GzipDecoder;
use futures::stream::TryStreamExt;
use tokio::io::AsyncBufReadExt;
use tokio_util::compat::FuturesAsyncReadCompatExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "http://localhost:8000/com.txt.gz";
    let response = reqwest::get(url).await?;
    let stream = response
        .bytes_stream()
        .map_err(|e| futures::io::Error::new(futures::io::ErrorKind::Other, e))
        .into_async_read()
        .compat();
    let gzip_decoder = GzipDecoder::new(stream);

    let buf_reader = tokio::io::BufReader::new(gzip_decoder);
    let mut lines = buf_reader.lines();
    let mut count = 0;
    while let Some(_line) = lines.next_line().await? {
        count += 1;
        if count % 10_000_000 == 0 {
            println!("Processed: {count}");
        }
    }

    println!("Total number of lines: {count}");
    Ok(())
}
