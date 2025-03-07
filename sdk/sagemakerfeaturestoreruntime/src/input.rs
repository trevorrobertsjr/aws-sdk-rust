// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;
/// See [`BatchGetRecordInput`](crate::input::BatchGetRecordInput)
pub mod batch_get_record_input {
    /// A builder for [`BatchGetRecordInput`](crate::input::BatchGetRecordInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) identifiers:
            std::option::Option<std::vec::Vec<crate::model::BatchGetRecordIdentifier>>,
    }
    impl Builder {
        pub fn identifiers(
            mut self,
            input: impl Into<crate::model::BatchGetRecordIdentifier>,
        ) -> Self {
            let mut v = self.identifiers.unwrap_or_default();
            v.push(input.into());
            self.identifiers = Some(v);
            self
        }
        pub fn set_identifiers(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::BatchGetRecordIdentifier>>,
        ) -> Self {
            self.identifiers = input;
            self
        }
        /// Consumes the builder and constructs a [`BatchGetRecordInput`](crate::input::BatchGetRecordInput)
        pub fn build(
            self,
        ) -> std::result::Result<
            crate::input::BatchGetRecordInput,
            smithy_http::operation::BuildError,
        > {
            Ok(crate::input::BatchGetRecordInput {
                identifiers: self.identifiers,
            })
        }
    }
}
#[doc(hidden)]
pub type BatchGetRecordInputOperationOutputAlias = crate::operation::BatchGetRecord;
#[doc(hidden)]
pub type BatchGetRecordInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl BatchGetRecordInput {
    /// Consumes the builder and constructs an Operation<[`BatchGetRecord`](crate::operation::BatchGetRecord)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::BatchGetRecord,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let request = self.request_builder_base()?;
            let body = crate::operation_ser::serialize_operation_batch_get_record(&self).map_err(
                |err| smithy_http::operation::BuildError::SerializationError(err.into()),
            )?;
            let request = Self::assemble(request, body);
            #[allow(unused_mut)]
            let mut request =
                smithy_http::operation::Request::new(request.map(smithy_http::body::SdkBody::from));
            request
                .config_mut()
                .insert(aws_http::user_agent::AwsUserAgent::new_from_environment(
                    crate::API_METADATA.clone(),
                ));
            #[allow(unused_mut)]
            let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
            request.config_mut().insert(signing_config);
            request
                .config_mut()
                .insert(aws_types::SigningService::from_static(
                    _config.signing_service(),
                ));
            aws_endpoint::set_endpoint_resolver(
                &mut request.config_mut(),
                _config.endpoint_resolver.clone(),
            );
            if let Some(region) = &_config.region {
                request.config_mut().insert(region.clone());
            }
            aws_auth::provider::set_provider(
                &mut request.config_mut(),
                _config.credentials_provider.clone(),
            );
            let op = smithy_http::operation::Operation::new(
                request,
                crate::operation::BatchGetRecord::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "BatchGetRecord",
                "sagemakerfeaturestoreruntime",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        write!(output, "/BatchGetRecord").expect("formatting should succeed");
        Ok(())
    }
    #[allow(clippy::unnecessary_wraps)]
    fn update_http_builder(
        &self,
        builder: http::request::Builder,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut uri = String::new();
        self.uri_base(&mut uri)?;
        Ok(builder.method("POST").uri(uri))
    }
    #[allow(clippy::unnecessary_wraps)]
    fn request_builder_base(
        &self,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut builder = self.update_http_builder(http::request::Builder::new())?;
        if !builder
            .headers_ref()
            .map(|h| h.contains_key("content-type"))
            .unwrap_or(false)
        {
            builder = builder.header("content-type", "application/json");
        }
        Ok(builder)
    }
    fn assemble(
        mut builder: http::request::Builder,
        body: smithy_http::body::SdkBody,
    ) -> http::request::Request<smithy_http::body::SdkBody> {
        if let Some(content_length) = body.content_length() {
            builder = builder.header(http::header::CONTENT_LENGTH, content_length)
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`BatchGetRecordInput`](crate::input::BatchGetRecordInput)
    pub fn builder() -> crate::input::batch_get_record_input::Builder {
        crate::input::batch_get_record_input::Builder::default()
    }
}

/// See [`DeleteRecordInput`](crate::input::DeleteRecordInput)
pub mod delete_record_input {
    /// A builder for [`DeleteRecordInput`](crate::input::DeleteRecordInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) feature_group_name: std::option::Option<std::string::String>,
        pub(crate) record_identifier_value_as_string: std::option::Option<std::string::String>,
        pub(crate) event_time: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The name of the feature group to delete the record from. </p>
        pub fn feature_group_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.feature_group_name = Some(input.into());
            self
        }
        pub fn set_feature_group_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.feature_group_name = input;
            self
        }
        /// <p>The value for the <code>RecordIdentifier</code> that uniquely identifies the record, in
        /// string format. </p>
        pub fn record_identifier_value_as_string(
            mut self,
            input: impl Into<std::string::String>,
        ) -> Self {
            self.record_identifier_value_as_string = Some(input.into());
            self
        }
        pub fn set_record_identifier_value_as_string(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.record_identifier_value_as_string = input;
            self
        }
        /// <p>Timestamp indicating when the deletion event occurred. <code>EventTime</code> can be
        /// used to query data at a certain point in time.</p>
        pub fn event_time(mut self, input: impl Into<std::string::String>) -> Self {
            self.event_time = Some(input.into());
            self
        }
        pub fn set_event_time(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.event_time = input;
            self
        }
        /// Consumes the builder and constructs a [`DeleteRecordInput`](crate::input::DeleteRecordInput)
        pub fn build(
            self,
        ) -> std::result::Result<crate::input::DeleteRecordInput, smithy_http::operation::BuildError>
        {
            Ok(crate::input::DeleteRecordInput {
                feature_group_name: self.feature_group_name,
                record_identifier_value_as_string: self.record_identifier_value_as_string,
                event_time: self.event_time,
            })
        }
    }
}
#[doc(hidden)]
pub type DeleteRecordInputOperationOutputAlias = crate::operation::DeleteRecord;
#[doc(hidden)]
pub type DeleteRecordInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl DeleteRecordInput {
    /// Consumes the builder and constructs an Operation<[`DeleteRecord`](crate::operation::DeleteRecord)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::DeleteRecord,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let request = self.request_builder_base()?;
            let body = smithy_http::body::SdkBody::from("");
            let request = Self::assemble(request, body);
            #[allow(unused_mut)]
            let mut request =
                smithy_http::operation::Request::new(request.map(smithy_http::body::SdkBody::from));
            request
                .config_mut()
                .insert(aws_http::user_agent::AwsUserAgent::new_from_environment(
                    crate::API_METADATA.clone(),
                ));
            #[allow(unused_mut)]
            let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
            request.config_mut().insert(signing_config);
            request
                .config_mut()
                .insert(aws_types::SigningService::from_static(
                    _config.signing_service(),
                ));
            aws_endpoint::set_endpoint_resolver(
                &mut request.config_mut(),
                _config.endpoint_resolver.clone(),
            );
            if let Some(region) = &_config.region {
                request.config_mut().insert(region.clone());
            }
            aws_auth::provider::set_provider(
                &mut request.config_mut(),
                _config.credentials_provider.clone(),
            );
            let op = smithy_http::operation::Operation::new(
                request,
                crate::operation::DeleteRecord::new(),
            )
            .with_metadata(smithy_http::operation::Metadata::new(
                "DeleteRecord",
                "sagemakerfeaturestoreruntime",
            ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        let feature_group_name = {
            let input = &self.feature_group_name;
            let input = input
                .as_ref()
                .ok_or(smithy_http::operation::BuildError::MissingField {
                    field: "feature_group_name",
                    details: "cannot be empty or unset",
                })?;
            let formatted = smithy_http::label::fmt_string(input, false);
            if formatted.is_empty() {
                return Err(smithy_http::operation::BuildError::MissingField {
                    field: "feature_group_name",
                    details: "cannot be empty or unset",
                });
            }
            formatted
        };
        write!(
            output,
            "/FeatureGroup/{FeatureGroupName}",
            FeatureGroupName = feature_group_name
        )
        .expect("formatting should succeed");
        Ok(())
    }
    fn uri_query(&self, mut output: &mut String) {
        let mut query = smithy_http::query::Writer::new(&mut output);
        if let Some(inner_1) = &self.record_identifier_value_as_string {
            query.push_kv(
                "RecordIdentifierValueAsString",
                &smithy_http::query::fmt_string(&inner_1),
            );
        }
        if let Some(inner_2) = &self.event_time {
            query.push_kv("EventTime", &smithy_http::query::fmt_string(&inner_2));
        }
    }
    #[allow(clippy::unnecessary_wraps)]
    fn update_http_builder(
        &self,
        builder: http::request::Builder,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut uri = String::new();
        self.uri_base(&mut uri)?;
        self.uri_query(&mut uri);
        Ok(builder.method("DELETE").uri(uri))
    }
    #[allow(clippy::unnecessary_wraps)]
    fn request_builder_base(
        &self,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut builder = self.update_http_builder(http::request::Builder::new())?;
        if !builder
            .headers_ref()
            .map(|h| h.contains_key("content-type"))
            .unwrap_or(false)
        {
            builder = builder.header("content-type", "application/json");
        }
        Ok(builder)
    }
    fn assemble(
        mut builder: http::request::Builder,
        body: smithy_http::body::SdkBody,
    ) -> http::request::Request<smithy_http::body::SdkBody> {
        if let Some(content_length) = body.content_length() {
            builder = builder.header(http::header::CONTENT_LENGTH, content_length)
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`DeleteRecordInput`](crate::input::DeleteRecordInput)
    pub fn builder() -> crate::input::delete_record_input::Builder {
        crate::input::delete_record_input::Builder::default()
    }
}

/// See [`GetRecordInput`](crate::input::GetRecordInput)
pub mod get_record_input {
    /// A builder for [`GetRecordInput`](crate::input::GetRecordInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) feature_group_name: std::option::Option<std::string::String>,
        pub(crate) record_identifier_value_as_string: std::option::Option<std::string::String>,
        pub(crate) feature_names: std::option::Option<std::vec::Vec<std::string::String>>,
    }
    impl Builder {
        /// <p>The name of the feature group in which you want to put the records.</p>
        pub fn feature_group_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.feature_group_name = Some(input.into());
            self
        }
        pub fn set_feature_group_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.feature_group_name = input;
            self
        }
        /// <p>The value that corresponds to <code>RecordIdentifier</code> type and uniquely identifies
        /// the record in the <code>FeatureGroup</code>. </p>
        pub fn record_identifier_value_as_string(
            mut self,
            input: impl Into<std::string::String>,
        ) -> Self {
            self.record_identifier_value_as_string = Some(input.into());
            self
        }
        pub fn set_record_identifier_value_as_string(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.record_identifier_value_as_string = input;
            self
        }
        pub fn feature_names(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.feature_names.unwrap_or_default();
            v.push(input.into());
            self.feature_names = Some(v);
            self
        }
        pub fn set_feature_names(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.feature_names = input;
            self
        }
        /// Consumes the builder and constructs a [`GetRecordInput`](crate::input::GetRecordInput)
        pub fn build(
            self,
        ) -> std::result::Result<crate::input::GetRecordInput, smithy_http::operation::BuildError>
        {
            Ok(crate::input::GetRecordInput {
                feature_group_name: self.feature_group_name,
                record_identifier_value_as_string: self.record_identifier_value_as_string,
                feature_names: self.feature_names,
            })
        }
    }
}
#[doc(hidden)]
pub type GetRecordInputOperationOutputAlias = crate::operation::GetRecord;
#[doc(hidden)]
pub type GetRecordInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl GetRecordInput {
    /// Consumes the builder and constructs an Operation<[`GetRecord`](crate::operation::GetRecord)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::GetRecord,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let request = self.request_builder_base()?;
            let body = smithy_http::body::SdkBody::from("");
            let request = Self::assemble(request, body);
            #[allow(unused_mut)]
            let mut request =
                smithy_http::operation::Request::new(request.map(smithy_http::body::SdkBody::from));
            request
                .config_mut()
                .insert(aws_http::user_agent::AwsUserAgent::new_from_environment(
                    crate::API_METADATA.clone(),
                ));
            #[allow(unused_mut)]
            let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
            request.config_mut().insert(signing_config);
            request
                .config_mut()
                .insert(aws_types::SigningService::from_static(
                    _config.signing_service(),
                ));
            aws_endpoint::set_endpoint_resolver(
                &mut request.config_mut(),
                _config.endpoint_resolver.clone(),
            );
            if let Some(region) = &_config.region {
                request.config_mut().insert(region.clone());
            }
            aws_auth::provider::set_provider(
                &mut request.config_mut(),
                _config.credentials_provider.clone(),
            );
            let op =
                smithy_http::operation::Operation::new(request, crate::operation::GetRecord::new())
                    .with_metadata(smithy_http::operation::Metadata::new(
                        "GetRecord",
                        "sagemakerfeaturestoreruntime",
                    ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        let feature_group_name = {
            let input = &self.feature_group_name;
            let input = input
                .as_ref()
                .ok_or(smithy_http::operation::BuildError::MissingField {
                    field: "feature_group_name",
                    details: "cannot be empty or unset",
                })?;
            let formatted = smithy_http::label::fmt_string(input, false);
            if formatted.is_empty() {
                return Err(smithy_http::operation::BuildError::MissingField {
                    field: "feature_group_name",
                    details: "cannot be empty or unset",
                });
            }
            formatted
        };
        write!(
            output,
            "/FeatureGroup/{FeatureGroupName}",
            FeatureGroupName = feature_group_name
        )
        .expect("formatting should succeed");
        Ok(())
    }
    fn uri_query(&self, mut output: &mut String) {
        let mut query = smithy_http::query::Writer::new(&mut output);
        if let Some(inner_3) = &self.record_identifier_value_as_string {
            query.push_kv(
                "RecordIdentifierValueAsString",
                &smithy_http::query::fmt_string(&inner_3),
            );
        }
        if let Some(inner_4) = &self.feature_names {
            for inner_5 in inner_4 {
                query.push_kv("FeatureName", &smithy_http::query::fmt_string(&inner_5));
            }
        }
    }
    #[allow(clippy::unnecessary_wraps)]
    fn update_http_builder(
        &self,
        builder: http::request::Builder,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut uri = String::new();
        self.uri_base(&mut uri)?;
        self.uri_query(&mut uri);
        Ok(builder.method("GET").uri(uri))
    }
    #[allow(clippy::unnecessary_wraps)]
    fn request_builder_base(
        &self,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut builder = self.update_http_builder(http::request::Builder::new())?;
        if !builder
            .headers_ref()
            .map(|h| h.contains_key("content-type"))
            .unwrap_or(false)
        {
            builder = builder.header("content-type", "application/json");
        }
        Ok(builder)
    }
    fn assemble(
        mut builder: http::request::Builder,
        body: smithy_http::body::SdkBody,
    ) -> http::request::Request<smithy_http::body::SdkBody> {
        if let Some(content_length) = body.content_length() {
            builder = builder.header(http::header::CONTENT_LENGTH, content_length)
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`GetRecordInput`](crate::input::GetRecordInput)
    pub fn builder() -> crate::input::get_record_input::Builder {
        crate::input::get_record_input::Builder::default()
    }
}

/// See [`PutRecordInput`](crate::input::PutRecordInput)
pub mod put_record_input {
    /// A builder for [`PutRecordInput`](crate::input::PutRecordInput)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) feature_group_name: std::option::Option<std::string::String>,
        pub(crate) record: std::option::Option<std::vec::Vec<crate::model::FeatureValue>>,
    }
    impl Builder {
        /// <p>The name of the feature group that you want to insert the record into.</p>
        pub fn feature_group_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.feature_group_name = Some(input.into());
            self
        }
        pub fn set_feature_group_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.feature_group_name = input;
            self
        }
        pub fn record(mut self, input: impl Into<crate::model::FeatureValue>) -> Self {
            let mut v = self.record.unwrap_or_default();
            v.push(input.into());
            self.record = Some(v);
            self
        }
        pub fn set_record(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::FeatureValue>>,
        ) -> Self {
            self.record = input;
            self
        }
        /// Consumes the builder and constructs a [`PutRecordInput`](crate::input::PutRecordInput)
        pub fn build(
            self,
        ) -> std::result::Result<crate::input::PutRecordInput, smithy_http::operation::BuildError>
        {
            Ok(crate::input::PutRecordInput {
                feature_group_name: self.feature_group_name,
                record: self.record,
            })
        }
    }
}
#[doc(hidden)]
pub type PutRecordInputOperationOutputAlias = crate::operation::PutRecord;
#[doc(hidden)]
pub type PutRecordInputOperationRetryAlias = aws_http::AwsErrorRetryPolicy;
impl PutRecordInput {
    /// Consumes the builder and constructs an Operation<[`PutRecord`](crate::operation::PutRecord)>
    #[allow(clippy::let_and_return)]
    pub fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        smithy_http::operation::Operation<
            crate::operation::PutRecord,
            aws_http::AwsErrorRetryPolicy,
        >,
        smithy_http::operation::BuildError,
    > {
        Ok({
            let request = self.request_builder_base()?;
            let body =
                crate::operation_ser::serialize_operation_put_record(&self).map_err(|err| {
                    smithy_http::operation::BuildError::SerializationError(err.into())
                })?;
            let request = Self::assemble(request, body);
            #[allow(unused_mut)]
            let mut request =
                smithy_http::operation::Request::new(request.map(smithy_http::body::SdkBody::from));
            request
                .config_mut()
                .insert(aws_http::user_agent::AwsUserAgent::new_from_environment(
                    crate::API_METADATA.clone(),
                ));
            #[allow(unused_mut)]
            let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
            request.config_mut().insert(signing_config);
            request
                .config_mut()
                .insert(aws_types::SigningService::from_static(
                    _config.signing_service(),
                ));
            aws_endpoint::set_endpoint_resolver(
                &mut request.config_mut(),
                _config.endpoint_resolver.clone(),
            );
            if let Some(region) = &_config.region {
                request.config_mut().insert(region.clone());
            }
            aws_auth::provider::set_provider(
                &mut request.config_mut(),
                _config.credentials_provider.clone(),
            );
            let op =
                smithy_http::operation::Operation::new(request, crate::operation::PutRecord::new())
                    .with_metadata(smithy_http::operation::Metadata::new(
                        "PutRecord",
                        "sagemakerfeaturestoreruntime",
                    ));
            let op = op.with_retry_policy(aws_http::AwsErrorRetryPolicy::new());
            op
        })
    }
    fn uri_base(&self, output: &mut String) -> Result<(), smithy_http::operation::BuildError> {
        let feature_group_name = {
            let input = &self.feature_group_name;
            let input = input
                .as_ref()
                .ok_or(smithy_http::operation::BuildError::MissingField {
                    field: "feature_group_name",
                    details: "cannot be empty or unset",
                })?;
            let formatted = smithy_http::label::fmt_string(input, false);
            if formatted.is_empty() {
                return Err(smithy_http::operation::BuildError::MissingField {
                    field: "feature_group_name",
                    details: "cannot be empty or unset",
                });
            }
            formatted
        };
        write!(
            output,
            "/FeatureGroup/{FeatureGroupName}",
            FeatureGroupName = feature_group_name
        )
        .expect("formatting should succeed");
        Ok(())
    }
    #[allow(clippy::unnecessary_wraps)]
    fn update_http_builder(
        &self,
        builder: http::request::Builder,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut uri = String::new();
        self.uri_base(&mut uri)?;
        Ok(builder.method("PUT").uri(uri))
    }
    #[allow(clippy::unnecessary_wraps)]
    fn request_builder_base(
        &self,
    ) -> std::result::Result<http::request::Builder, smithy_http::operation::BuildError> {
        let mut builder = self.update_http_builder(http::request::Builder::new())?;
        if !builder
            .headers_ref()
            .map(|h| h.contains_key("content-type"))
            .unwrap_or(false)
        {
            builder = builder.header("content-type", "application/json");
        }
        Ok(builder)
    }
    fn assemble(
        mut builder: http::request::Builder,
        body: smithy_http::body::SdkBody,
    ) -> http::request::Request<smithy_http::body::SdkBody> {
        if let Some(content_length) = body.content_length() {
            builder = builder.header(http::header::CONTENT_LENGTH, content_length)
        }
        builder.body(body).expect("should be valid request")
    }
    /// Creates a new builder-style object to manufacture [`PutRecordInput`](crate::input::PutRecordInput)
    pub fn builder() -> crate::input::put_record_input::Builder {
        crate::input::put_record_input::Builder::default()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct PutRecordInput {
    /// <p>The name of the feature group that you want to insert the record into.</p>
    pub feature_group_name: std::option::Option<std::string::String>,
    /// <p>List of FeatureValues to be inserted. This will be a full over-write. If you only want
    /// to update few of the feature values, do the following:</p>
    /// <ul>
    /// <li>
    /// <p>Use <code>GetRecord</code> to retrieve the latest record.</p>
    /// </li>
    /// <li>
    /// <p>Update the record returned from <code>GetRecord</code>. </p>
    /// </li>
    /// <li>
    /// <p>Use <code>PutRecord</code> to update feature values.</p>
    /// </li>
    /// </ul>
    pub record: std::option::Option<std::vec::Vec<crate::model::FeatureValue>>,
}
impl std::fmt::Debug for PutRecordInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("PutRecordInput");
        formatter.field("feature_group_name", &self.feature_group_name);
        formatter.field("record", &self.record);
        formatter.finish()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetRecordInput {
    /// <p>The name of the feature group in which you want to put the records.</p>
    pub feature_group_name: std::option::Option<std::string::String>,
    /// <p>The value that corresponds to <code>RecordIdentifier</code> type and uniquely identifies
    /// the record in the <code>FeatureGroup</code>. </p>
    pub record_identifier_value_as_string: std::option::Option<std::string::String>,
    /// <p>List of names of Features to be retrieved. If not specified, the latest value for all
    /// the Features are returned.</p>
    pub feature_names: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl std::fmt::Debug for GetRecordInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetRecordInput");
        formatter.field("feature_group_name", &self.feature_group_name);
        formatter.field(
            "record_identifier_value_as_string",
            &self.record_identifier_value_as_string,
        );
        formatter.field("feature_names", &self.feature_names);
        formatter.finish()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DeleteRecordInput {
    /// <p>The name of the feature group to delete the record from. </p>
    pub feature_group_name: std::option::Option<std::string::String>,
    /// <p>The value for the <code>RecordIdentifier</code> that uniquely identifies the record, in
    /// string format. </p>
    pub record_identifier_value_as_string: std::option::Option<std::string::String>,
    /// <p>Timestamp indicating when the deletion event occurred. <code>EventTime</code> can be
    /// used to query data at a certain point in time.</p>
    pub event_time: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for DeleteRecordInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DeleteRecordInput");
        formatter.field("feature_group_name", &self.feature_group_name);
        formatter.field(
            "record_identifier_value_as_string",
            &self.record_identifier_value_as_string,
        );
        formatter.field("event_time", &self.event_time);
        formatter.finish()
    }
}

#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct BatchGetRecordInput {
    /// <p>A list of <code>FeatureGroup</code> names, with their corresponding <code>RecordIdentifier</code> value, and Feature name
    /// that have been requested to be retrieved in batch.</p>
    pub identifiers: std::option::Option<std::vec::Vec<crate::model::BatchGetRecordIdentifier>>,
}
impl std::fmt::Debug for BatchGetRecordInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("BatchGetRecordInput");
        formatter.field("identifiers", &self.identifiers);
        formatter.finish()
    }
}
