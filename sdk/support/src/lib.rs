#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
//! <fullname>AWS Support</fullname>
//! <p>The <i>AWS Support API Reference</i> is intended for programmers who need detailed
//! information about the AWS Support operations and data types. You can use the API to manage
//! your support cases programmatically. The AWS Support API uses HTTP methods that return
//! results in JSON format.</p>
//! <note>
//! <ul>
//! <li>
//! <p>You must have a Business or Enterprise Support plan to use the AWS Support
//! API. </p>
//! </li>
//! <li>
//! <p>If you call the AWS Support API from an account that does not have a
//! Business or Enterprise Support plan, the
//! <code>SubscriptionRequiredException</code> error message appears. For
//! information about changing your support plan, see <a href="http://aws.amazon.com/premiumsupport/">AWS Support</a>.</p>
//! </li>
//! </ul>
//! </note>
//! <p>The AWS Support service also exposes a set of <a href="http://aws.amazon.com/premiumsupport/trustedadvisor/">AWS Trusted Advisor</a> features. You can
//! retrieve a list of checks and their descriptions, get check results, specify checks to
//! refresh, and get the refresh status of checks.</p>
//! <p>The following list describes the AWS Support case management operations:</p>
//! <ul>
//! <li>
//! <p> Service names, issue categories, and available severity levels  - The
//! <a>DescribeServices</a> and <a>DescribeSeverityLevels</a> operations return AWS service names,
//! service codes, service categories, and problem severity levels. You use these
//! values when you call the <a>CreateCase</a> operation.</p>
//! </li>
//! <li>
//! <p> Case creation, case details, and case resolution - The <a>CreateCase</a>, <a>DescribeCases</a>, <a>DescribeAttachment</a>, and <a>ResolveCase</a> operations
//! create AWS Support cases, retrieve information about cases, and resolve cases.</p>
//! </li>
//! <li>
//! <p> Case communication - The <a>DescribeCommunications</a>,
//! <a>AddCommunicationToCase</a>, and <a>AddAttachmentsToSet</a> operations retrieve and add communications
//! and attachments to AWS Support cases.</p>
//! </li>
//! </ul>
//! <p>The following list describes the operations available from the AWS Support service for
//! Trusted Advisor:</p>
//! <ul>
//! <li>
//! <p>
//! <a>DescribeTrustedAdvisorChecks</a> returns the list of checks that
//! run against your AWS resources.</p>
//! </li>
//! <li>
//! <p>Using the <code>checkId</code> for a specific check returned by <a>DescribeTrustedAdvisorChecks</a>, you can call <a>DescribeTrustedAdvisorCheckResult</a> to obtain the results for the
//! check that you specified.</p>
//! </li>
//! <li>
//! <p>
//! <a>DescribeTrustedAdvisorCheckSummaries</a> returns summarized
//! results for one or more Trusted Advisor checks.</p>
//! </li>
//! <li>
//! <p>
//! <a>RefreshTrustedAdvisorCheck</a> requests that Trusted Advisor rerun a
//! specified check.</p>
//! </li>
//! <li>
//! <p>
//! <a>DescribeTrustedAdvisorCheckRefreshStatuses</a> reports the refresh
//! status of one or more checks.</p>
//! </li>
//! </ul>
//! <p>For authentication of requests, AWS Support uses <a href="https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html">Signature Version 4 Signing
//! Process</a>.</p>
//! <p>See <a href="https://docs.aws.amazon.com/awssupport/latest/user/Welcome.html">About the
//! AWS Support API</a> in the <i>AWS Support User Guide</i> for
//! information about how to use this service to create and manage your support cases, and
//! how to call Trusted Advisor for results of checks on your resources.</p>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_endpoint;
#[cfg(feature = "client")]
pub mod client;
pub mod config;
pub mod error;
mod error_meta;
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
pub mod model;
mod no_credentials;
pub mod operation;
mod operation_deser;
mod operation_ser;
pub mod output;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use smithy_http::byte_stream::ByteStream;
pub use smithy_http::result::SdkError;
pub use smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("support", PKG_VERSION);
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;