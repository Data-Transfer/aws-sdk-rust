// SHOWS HOW TO:
// 1) read a chunk of data from a file using:
//    - tokio::fs::File::seek
//    - tokio::io::AsyncReadExt::take (added to File, limits the number of bytes read)
// 2) minimise memory usage through tokio_util::FramedRead/BytesCodec 
// 
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


// upload file chunk to bucket/key uses framed read to minimise copies
use std::time::Instant;
pub async fn upload_chunk(
    client: &Client,
    bucket: &str,
    file_name: &str,
    key: &str,
    start_offset: u64,
    chunk_size: u64
) -> Result<(), Error> {
    // minimise memory copies https://github.com/hyperium/hyper/issues/2166#issuecomment-612363623
    let mut file = tokio::fs::File::open(Path::new(file_name)).await.map_err(|err| Error::Unhandled(Box::new(err)))?;
    use tokio::io::{AsyncSeekExt, AsyncReadExt};
    file.seek(std::io::SeekFrom::Start(start_offset)).await.map_err(|err| Error::Unhandled(Box::new(err)))?; 
    let file = file.take(chunk_size);    
    use tokio_util::codec::{FramedRead, BytesCodec};
    let stream = FramedRead::with_capacity(file, BytesCodec::new(), chunk_size as usize);
    let b = hyper::Body::wrap_stream(stream);
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
