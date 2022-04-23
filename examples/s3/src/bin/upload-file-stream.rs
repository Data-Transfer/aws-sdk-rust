 
// Upload file to s3 bucket

// Initialisation:
//
//  #1
//  use aws_config::meta::region::RegionProviderChain;
//  let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
//  let config = aws_config::from_env().region(region_provider).load().await;
//  let client = Client::new(&config);
//  #2
//  let conf = aws_config::from_env()
//             .credentials_provider(
//                 aws_config::profile::ProfileFileCredentialsProvider::builder()
//                 .profile_name("acacia")
//                  .build())
//             .load()
//             .await;
//  #3
//  let conf = aws_config::from_env()
//    .credentials_provider(aws_sdk_s3::Credentials::new(
//           aws_access_key_id,
//           aws_secret_access_key,
//           None,
//           None,
//          "custom profile",
//       )).load().await;
//
use aws_sdk_s3::types::ByteStream;
use aws_sdk_s3::{Client, Endpoint, Error};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), aws_sdk_s3::Error> {
    let args = std::env::args().collect::<Vec<_>>();
    let usage = format!("{} <profile> <url> <bucket> <key> <input file> <start offset> <chunk size, 0 for whole file>", args[0]);
    let profile = args.get(1).expect(&usage);
    let url = args.get(2).expect(&usage);
    let bucket = args.get(3).expect(&usage);
    let key = args.get(4).expect(&usage);
    let file_name = args.get(5).expect(&usage);
    let start_offset = args.get(6).expect(&usage).parse::<u64>().expect("Error parsing offset");
    let chunk_size = args.get(7).expect(&usage).parse::<u64>().expect("Error parsing chunk size");
    let md = std::fs::metadata(file_name).map_err(|err| Error::Unhandled(Box::new(err)))?;
    let chunk_size = if chunk_size == 0 { md.len()} else {chunk_size};

    // credentials are read from .aws/credentials file
    let conf = aws_config::from_env()
        .region("us-east-1")
        .credentials_provider(
            aws_config::profile::ProfileFileCredentialsProvider::builder()
                .profile_name(profile)
                .build(),
        )
        .load()
        .await;
    let uri = url.parse::<http::uri::Uri>().expect("Invalid URL");
    let ep = Endpoint::immutable(uri);
    let s3_conf = aws_sdk_s3::config::Builder::from(&conf)
        .endpoint_resolver(ep)
        .build();
    let client = Client::from_conf(s3_conf);
    upload_chunk(&client, &bucket, &file_name, &key, start_offset, chunk_size).await?;
    Ok(())
}

use futures::stream::Stream;
use std::task::Poll;
struct FileStream {
    buffer: Vec<u8>,
    counter: usize
}
impl FileStream {
    fn new() -> Self {
        FileStream{ buffer: vec![0; 128], counter: 0 }
    }
}
impl Stream for FileStream {
    type Item = Result<Vec::<u8>, String>;
    fn poll_next(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        if self.counter == 0 { 
            self.counter += 1; 
            return Poll::Pending;
        }
        if self.counter == 64 { return Poll::Ready(None); }
        return Poll::Ready(Some(Ok(self.buffer[..self.counter].to_vec())));
    }
}

// upload file chunk to bucket/key
use std::time::Instant;
use hyper::body::Body;
pub async fn upload_chunk(
    client: &Client,
    bucket: &str,
    file_name: &str,
    key: &str,
    start_offset: u64,
    chunk_size: u64
) -> Result<(), Error> {
    // only works with patched aws_sdk (https://github.com/Data-Transfer/aws-sdk-rust)
    let mut file = tokio::fs::File::open(Path::new(file_name)).await.map_err(|err| Error::Unhandled(Box::new(err)))?;
    use tokio::io::AsyncSeekExt;
    file.seek(std::io::SeekFrom::Start(start_offset)).await.map_err(|err| Error::Unhandled(Box::new(err)))?;
    let mut buf = vec![0_u8; chunk_size as usize];
    use tokio::io::AsyncReadExt;
    file.read(&mut buf).await.map_err(|err| Error::Unhandled(Box::new(err)))?;
    let s = FileStream::new();
    let b = hyper::body::Body::wrap_stream(s);
    let body = ByteStream::from(b);
    let start = Instant::now();
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(body)
        .send()
        .await?;
    let elapsed = start.elapsed();
    println!(
        "Uploaded chunk of size {} from file {} in {:.2} s",
        chunk_size,
        file_name,
        elapsed.as_secs_f32()
    );
    Ok(())
}

// upload file
pub async fn upload(
    client: &Client,
    bucket: &str,
    file_name: &str,
    key: &str) -> Result<(), Error> {
    let body = ByteStream::from_path(Path::new(file_name))
        .await
        .expect(&format!("Cannot read from {}", file_name));
    let start = Instant::now();
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(body)
        .send()
        .await?;
    let elapsed = start.elapsed();
    println!(
        "Uploaded file {} in {:.2} s",
        file_name,
        elapsed.as_secs_f32()
    );
    Ok(())
}

