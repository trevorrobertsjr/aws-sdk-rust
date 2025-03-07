#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
//! <p>With Application Auto Scaling, you can configure automatic scaling for the following
//! resources:</p>
//! <ul>
//! <li>
//! <p>Amazon ECS services</p>
//! </li>
//! <li>
//! <p>Amazon EC2 Spot Fleet requests</p>
//! </li>
//! <li>
//! <p>Amazon EMR clusters</p>
//! </li>
//! <li>
//! <p>Amazon AppStream 2.0 fleets</p>
//! </li>
//! <li>
//! <p>Amazon DynamoDB tables and global secondary indexes throughput capacity</p>
//! </li>
//! <li>
//! <p>Amazon Aurora Replicas</p>
//! </li>
//! <li>
//! <p>Amazon SageMaker endpoint variants</p>
//! </li>
//! <li>
//! <p>Custom resources provided by your own applications or services</p>
//! </li>
//! <li>
//! <p>Amazon Comprehend document classification and entity recognizer endpoints</p>
//! </li>
//! <li>
//! <p>AWS Lambda function provisioned concurrency</p>
//! </li>
//! <li>
//! <p>Amazon Keyspaces (for Apache Cassandra) tables</p>
//! </li>
//! <li>
//! <p>Amazon Managed Streaming for Apache Kafka broker storage</p>
//! </li>
//! </ul>
//! <p>
//! <b>API Summary</b>
//! </p>
//! <p>The Application Auto Scaling service API includes three key sets of actions: </p>
//! <ul>
//! <li>
//! <p>Register and manage scalable targets - Register AWS or custom resources as scalable
//! targets (a resource that Application Auto Scaling can scale), set minimum and maximum capacity limits, and
//! retrieve information on existing scalable targets.</p>
//! </li>
//! <li>
//! <p>Configure and manage automatic scaling - Define scaling policies to dynamically scale
//! your resources in response to CloudWatch alarms, schedule one-time or recurring scaling actions,
//! and retrieve your recent scaling activity history.</p>
//! </li>
//! <li>
//! <p>Suspend and resume scaling - Temporarily suspend and later resume automatic scaling by
//! calling the <a href="https://docs.aws.amazon.com/autoscaling/application/APIReference/API_RegisterScalableTarget.html">RegisterScalableTarget</a> API action for any Application Auto Scaling scalable target. You can
//! suspend and resume (individually or in combination) scale-out activities that are
//! triggered by a scaling policy, scale-in activities that are triggered by a scaling policy,
//! and scheduled scaling.</p>
//! </li>
//! </ul>
//! <p>To learn more about Application Auto Scaling, including information about granting IAM users required
//! permissions for Application Auto Scaling actions, see the <a href="https://docs.aws.amazon.com/autoscaling/application/userguide/what-is-application-auto-scaling.html">Application Auto Scaling User
//! Guide</a>.</p>

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
pub mod operation;
mod operation_deser;
mod operation_ser;
pub mod output;
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use smithy_http::byte_stream::ByteStream;
pub use smithy_http::result::SdkError;
pub use smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("applicationautoscaling", PKG_VERSION);
pub use aws_auth::Credentials;
pub use aws_types::region::Region;
#[cfg(feature = "client")]
pub use client::Client;
pub use smithy_http::endpoint::Endpoint;
