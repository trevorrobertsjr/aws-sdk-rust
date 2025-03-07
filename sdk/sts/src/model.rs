// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>AWS credentials for API authentication.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Credentials {
    /// <p>The access key ID that identifies the temporary security credentials.</p>
    pub access_key_id: std::option::Option<std::string::String>,
    /// <p>The secret access key that can be used to sign requests.</p>
    pub secret_access_key: std::option::Option<std::string::String>,
    /// <p>The token that users must pass to the service API to use the temporary
    /// credentials.</p>
    pub session_token: std::option::Option<std::string::String>,
    /// <p>The date on which the current credentials expire.</p>
    pub expiration: std::option::Option<smithy_types::Instant>,
}
impl std::fmt::Debug for Credentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Credentials");
        formatter.field("access_key_id", &self.access_key_id);
        formatter.field("secret_access_key", &self.secret_access_key);
        formatter.field("session_token", &self.session_token);
        formatter.field("expiration", &self.expiration);
        formatter.finish()
    }
}
/// See [`Credentials`](crate::model::Credentials)
pub mod credentials {
    /// A builder for [`Credentials`](crate::model::Credentials)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) access_key_id: std::option::Option<std::string::String>,
        pub(crate) secret_access_key: std::option::Option<std::string::String>,
        pub(crate) session_token: std::option::Option<std::string::String>,
        pub(crate) expiration: std::option::Option<smithy_types::Instant>,
    }
    impl Builder {
        /// <p>The access key ID that identifies the temporary security credentials.</p>
        pub fn access_key_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.access_key_id = Some(input.into());
            self
        }
        pub fn set_access_key_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.access_key_id = input;
            self
        }
        /// <p>The secret access key that can be used to sign requests.</p>
        pub fn secret_access_key(mut self, input: impl Into<std::string::String>) -> Self {
            self.secret_access_key = Some(input.into());
            self
        }
        pub fn set_secret_access_key(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.secret_access_key = input;
            self
        }
        /// <p>The token that users must pass to the service API to use the temporary
        /// credentials.</p>
        pub fn session_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.session_token = Some(input.into());
            self
        }
        pub fn set_session_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.session_token = input;
            self
        }
        /// <p>The date on which the current credentials expire.</p>
        pub fn expiration(mut self, input: smithy_types::Instant) -> Self {
            self.expiration = Some(input);
            self
        }
        pub fn set_expiration(mut self, input: std::option::Option<smithy_types::Instant>) -> Self {
            self.expiration = input;
            self
        }
        /// Consumes the builder and constructs a [`Credentials`](crate::model::Credentials)
        pub fn build(self) -> crate::model::Credentials {
            crate::model::Credentials {
                access_key_id: self.access_key_id,
                secret_access_key: self.secret_access_key,
                session_token: self.session_token,
                expiration: self.expiration,
            }
        }
    }
}
impl Credentials {
    /// Creates a new builder-style object to manufacture [`Credentials`](crate::model::Credentials)
    pub fn builder() -> crate::model::credentials::Builder {
        crate::model::credentials::Builder::default()
    }
}

/// <p>Identifiers for the federated user that is associated with the credentials.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct FederatedUser {
    /// <p>The string that identifies the federated user associated with the credentials, similar
    /// to the unique ID of an IAM user.</p>
    pub federated_user_id: std::option::Option<std::string::String>,
    /// <p>The ARN that specifies the federated user that is associated with the credentials. For
    /// more information about ARNs and how to use them in policies, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html">IAM
    /// Identifiers</a> in the <i>IAM User Guide</i>. </p>
    pub arn: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for FederatedUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("FederatedUser");
        formatter.field("federated_user_id", &self.federated_user_id);
        formatter.field("arn", &self.arn);
        formatter.finish()
    }
}
/// See [`FederatedUser`](crate::model::FederatedUser)
pub mod federated_user {
    /// A builder for [`FederatedUser`](crate::model::FederatedUser)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) federated_user_id: std::option::Option<std::string::String>,
        pub(crate) arn: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The string that identifies the federated user associated with the credentials, similar
        /// to the unique ID of an IAM user.</p>
        pub fn federated_user_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.federated_user_id = Some(input.into());
            self
        }
        pub fn set_federated_user_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.federated_user_id = input;
            self
        }
        /// <p>The ARN that specifies the federated user that is associated with the credentials. For
        /// more information about ARNs and how to use them in policies, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html">IAM
        /// Identifiers</a> in the <i>IAM User Guide</i>. </p>
        pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.arn = Some(input.into());
            self
        }
        pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.arn = input;
            self
        }
        /// Consumes the builder and constructs a [`FederatedUser`](crate::model::FederatedUser)
        pub fn build(self) -> crate::model::FederatedUser {
            crate::model::FederatedUser {
                federated_user_id: self.federated_user_id,
                arn: self.arn,
            }
        }
    }
}
impl FederatedUser {
    /// Creates a new builder-style object to manufacture [`FederatedUser`](crate::model::FederatedUser)
    pub fn builder() -> crate::model::federated_user::Builder {
        crate::model::federated_user::Builder::default()
    }
}

/// <p>You can pass custom key-value pair attributes when you assume a role or federate a user.
/// These are called session tags. You can then use the session tags to control access to
/// resources. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_session-tags.html">Tagging AWS STS Sessions</a> in the
/// <i>IAM User Guide</i>.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Tag {
    /// <p>The key for a session tag.</p>
    /// <p>You can pass up to 50 session tags. The plain text session tag keys can’t exceed 128
    /// characters. For these and additional limits, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-limits.html#reference_iam-limits-entity-length">IAM
    /// and STS Character Limits</a> in the <i>IAM User Guide</i>.</p>
    pub key: std::option::Option<std::string::String>,
    /// <p>The value for a session tag.</p>
    /// <p>You can pass up to 50 session tags. The plain text session tag values can’t exceed 256
    /// characters. For these and additional limits, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-limits.html#reference_iam-limits-entity-length">IAM
    /// and STS Character Limits</a> in the <i>IAM User Guide</i>.</p>
    pub value: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Tag");
        formatter.field("key", &self.key);
        formatter.field("value", &self.value);
        formatter.finish()
    }
}
/// See [`Tag`](crate::model::Tag)
pub mod tag {
    /// A builder for [`Tag`](crate::model::Tag)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) key: std::option::Option<std::string::String>,
        pub(crate) value: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The key for a session tag.</p>
        /// <p>You can pass up to 50 session tags. The plain text session tag keys can’t exceed 128
        /// characters. For these and additional limits, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-limits.html#reference_iam-limits-entity-length">IAM
        /// and STS Character Limits</a> in the <i>IAM User Guide</i>.</p>
        pub fn key(mut self, input: impl Into<std::string::String>) -> Self {
            self.key = Some(input.into());
            self
        }
        pub fn set_key(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.key = input;
            self
        }
        /// <p>The value for a session tag.</p>
        /// <p>You can pass up to 50 session tags. The plain text session tag values can’t exceed 256
        /// characters. For these and additional limits, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-limits.html#reference_iam-limits-entity-length">IAM
        /// and STS Character Limits</a> in the <i>IAM User Guide</i>.</p>
        pub fn value(mut self, input: impl Into<std::string::String>) -> Self {
            self.value = Some(input.into());
            self
        }
        pub fn set_value(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.value = input;
            self
        }
        /// Consumes the builder and constructs a [`Tag`](crate::model::Tag)
        pub fn build(self) -> crate::model::Tag {
            crate::model::Tag {
                key: self.key,
                value: self.value,
            }
        }
    }
}
impl Tag {
    /// Creates a new builder-style object to manufacture [`Tag`](crate::model::Tag)
    pub fn builder() -> crate::model::tag::Builder {
        crate::model::tag::Builder::default()
    }
}

/// <p>A reference to the IAM managed policy that is passed as a session policy for a role
/// session or a federated user session.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct PolicyDescriptorType {
    /// <p>The Amazon Resource Name (ARN) of the IAM managed policy to use as a session policy
    /// for the role. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS
    /// Service Namespaces</a> in the <i>AWS General Reference</i>.</p>
    pub arn: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for PolicyDescriptorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("PolicyDescriptorType");
        formatter.field("arn", &self.arn);
        formatter.finish()
    }
}
/// See [`PolicyDescriptorType`](crate::model::PolicyDescriptorType)
pub mod policy_descriptor_type {
    /// A builder for [`PolicyDescriptorType`](crate::model::PolicyDescriptorType)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) arn: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The Amazon Resource Name (ARN) of the IAM managed policy to use as a session policy
        /// for the role. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS
        /// Service Namespaces</a> in the <i>AWS General Reference</i>.</p>
        pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.arn = Some(input.into());
            self
        }
        pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.arn = input;
            self
        }
        /// Consumes the builder and constructs a [`PolicyDescriptorType`](crate::model::PolicyDescriptorType)
        pub fn build(self) -> crate::model::PolicyDescriptorType {
            crate::model::PolicyDescriptorType { arn: self.arn }
        }
    }
}
impl PolicyDescriptorType {
    /// Creates a new builder-style object to manufacture [`PolicyDescriptorType`](crate::model::PolicyDescriptorType)
    pub fn builder() -> crate::model::policy_descriptor_type::Builder {
        crate::model::policy_descriptor_type::Builder::default()
    }
}

/// <p>The identifiers for the temporary security credentials that the operation
/// returns.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct AssumedRoleUser {
    /// <p>A unique identifier that contains the role ID and the role session name of the role that
    /// is being assumed. The role ID is generated by AWS when the role is created.</p>
    pub assumed_role_id: std::option::Option<std::string::String>,
    /// <p>The ARN of the temporary security credentials that are returned from the <a>AssumeRole</a> action. For more information about ARNs and how to use them in
    /// policies, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html">IAM Identifiers</a> in the
    /// <i>IAM User Guide</i>.</p>
    pub arn: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for AssumedRoleUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("AssumedRoleUser");
        formatter.field("assumed_role_id", &self.assumed_role_id);
        formatter.field("arn", &self.arn);
        formatter.finish()
    }
}
/// See [`AssumedRoleUser`](crate::model::AssumedRoleUser)
pub mod assumed_role_user {
    /// A builder for [`AssumedRoleUser`](crate::model::AssumedRoleUser)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) assumed_role_id: std::option::Option<std::string::String>,
        pub(crate) arn: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>A unique identifier that contains the role ID and the role session name of the role that
        /// is being assumed. The role ID is generated by AWS when the role is created.</p>
        pub fn assumed_role_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.assumed_role_id = Some(input.into());
            self
        }
        pub fn set_assumed_role_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.assumed_role_id = input;
            self
        }
        /// <p>The ARN of the temporary security credentials that are returned from the <a>AssumeRole</a> action. For more information about ARNs and how to use them in
        /// policies, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html">IAM Identifiers</a> in the
        /// <i>IAM User Guide</i>.</p>
        pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.arn = Some(input.into());
            self
        }
        pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.arn = input;
            self
        }
        /// Consumes the builder and constructs a [`AssumedRoleUser`](crate::model::AssumedRoleUser)
        pub fn build(self) -> crate::model::AssumedRoleUser {
            crate::model::AssumedRoleUser {
                assumed_role_id: self.assumed_role_id,
                arn: self.arn,
            }
        }
    }
}
impl AssumedRoleUser {
    /// Creates a new builder-style object to manufacture [`AssumedRoleUser`](crate::model::AssumedRoleUser)
    pub fn builder() -> crate::model::assumed_role_user::Builder {
        crate::model::assumed_role_user::Builder::default()
    }
}
