use async_compression::tokio::bufread::GzipDecoder;
use futures::stream::TryStreamExt;
use tokio::io::AsyncBufReadExt;
use tokio_util::compat::FuturesAsyncReadCompatExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = "https://f001.backblazeb2.com/file/korteur/hello-world.txt.gz";
    let response = reqwest::get(url).await?;
    let stream = response
        .bytes_stream()
        .map_err(|e| futures::io::Error::new(futures::io::ErrorKind::Other, e))
        .into_async_read()
        .compat();
    let gzip_decoder = GzipDecoder::new(stream);

    // Print decompressed txt content
    let buf_reader = tokio::io::BufReader::new(gzip_decoder);
    let mut lines = buf_reader.lines();
    while let Some(line) = lines.next_line().await? {
        println!("{line}");
    }

    Ok(())
}
