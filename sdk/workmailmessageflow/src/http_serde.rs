// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn deser_payload_get_raw_message_content_get_raw_message_content_output_message_content(
    body: &mut smithy_http::body::SdkBody,
) -> std::result::Result<
    smithy_http::byte_stream::ByteStream,
    crate::error::GetRawMessageContentError,
> {
    // replace the body with an empty body
    let body = std::mem::replace(body, smithy_http::body::SdkBody::taken());
    Ok(smithy_http::byte_stream::ByteStream::new(body))
}