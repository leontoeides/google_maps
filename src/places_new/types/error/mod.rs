#![allow(clippy::ref_option, reason = "for the `getset` crate")]

use crate::ClassifiedError;
use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// -------------------------------------------------------------------------------------------------
//
/// Standard Google API error response wrapper.
///
/// Top-level structure returned by newer Google APIs (like Places API New) when an error occurs.
/// Contains a single `Detail` field that holds all error information including status code,
/// message, and optional detailed error information.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize, Getters, thiserror::Error, miette::Diagnostic)]
pub struct Error {
    /// The detailed error information.
    #[getset(get = "pub")]
    pub error: Detail,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error.message)
    }
}

impl crate::traits::ClassifiableError<'_, Self> for Error {
    /// Classifies the error response based on its detail's status code.
    ///
    /// Determines whether the error is transient (temporary condition that may resolve on retry) or
    /// permanent (requires changes before retrying) by examining the underlying error detail and
    /// status code.
    fn classify(&self) -> ClassifiedError<'_, Self> {
        if self.error.classify().is_transient() {
            ClassifiedError::Transient(self)
        } else {
            ClassifiedError::Permanent(self)
        }
    }
}

// -------------------------------------------------------------------------------------------------
//
/// Detailed error information following the Google API error model.
///
/// Contains the core error data including HTTP status code, developer-facing message, status
/// string, and optional structured details.
///
/// This follows the `Status` type from `google.rpc.Status proto`, which provides a logical error
/// model suitable for REST and RPC APIs. The details field can contain additional context like
/// localized messages, retry information, or links to documentation.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize, CopyGetters, Getters)]
pub struct Detail {
    /// The error code as a canonical gRPC status code.
    ///
    /// Indicates the type of error that occurred using standard gRPC error codes.
    #[serde(deserialize_with = "deserialize_code")]
    #[getset(get_copy = "pub")]
    pub code: Code,

    /// A developer-facing error message in English.
    ///
    /// User-facing error messages should be localized and included in the details field as
    /// `LocalizedMessage` entries, or localized by the client.
    #[getset(get = "pub")]
    pub message: String,

    /// The status string representation (e.g., `INVALID_ARGUMENT`, `NOT_FOUND`).
    ///
    /// Corresponds to the string name of the google.rpc.Code enum value.
    #[getset(get = "pub")]
    pub status: String,

    /// Additional structured details about the error.
    ///
    /// A list of messages providing extra context about the error. Common types include `ErrorInfo`
    /// (reason and domain), `LocalizedMessage` (translated user messages), and `Help` (links to
    /// documentation). This field is optional and defaults to an empty vector.
    #[serde(default)]
    #[getset(get = "pub")]
    pub details: Vec<ErrorDetail>,
}

impl std::fmt::Display for Detail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl crate::traits::ClassifiableError<'_, Self> for Detail {
    /// Classifies the error detail based on its status code.
    ///
    /// Delegates to the `Code`'s classification logic, determining whether the error represents a
    /// transient condition (may succeed on retry) or a permanent failure (requires request or
    /// system changes).
    fn classify(&self) -> ClassifiedError<'_, Self> {
        if self.code.classify().is_transient() {
            ClassifiedError::Transient(self)
        } else {
            ClassifiedError::Permanent(self)
        }
    }
}

// -------------------------------------------------------------------------------------------------
//
/// Structured error detail information.
///
/// Represents various types of additional error context that can be attached to an error response.
/// Each variant corresponds to a specific proto message type from `google.rpc.error_details`. The
/// type is determined by the "@type" field in the JSON, which contains the fully qualified proto
/// message name.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ErrorDetail {
    /// Describes the cause of the error with structured details.
    ///
    /// Provides the reason, domain, and metadata for understanding why an error occurred.
    ///
    /// For example, when an API is not enabled, this would contain reason `API_DISABLED` with the
    /// domain "googleapis.com" and metadata identifying the affected resource and service.
    #[serde(rename = "type.googleapis.com/google.rpc.ErrorInfo")]
    ErrorInfo {
        /// The reason of the error as an `UPPER_SNAKE_CASE` constant.
        ///
        /// A constant value identifying the proximate cause of the error. Must be at most 63
        /// characters and match the pattern `[A-Z][A-Z0-9_]+[A-Z0-9]`. Error reasons are unique
        /// within a particular domain.
        reason: String,

        /// The logical grouping to which the reason belongs.
        ///
        /// Typically the registered service name of the tool or product generating the error
        /// (e.g., "pubsub.googleapis.com"). For common infrastructure errors, must be a globally
        /// unique value like "googleapis.com".
        domain: String,

        /// Additional structured details about this error.
        ///
        /// Keys should be lowerCamelCase, match `[a-z][a-zA-Z0-9-_]+`, and be limited to 64
        /// characters. When identifying exceeded limits, units should be in the key rather than the
        /// value (e.g., "instanceLimitPerRequest": "100").
        metadata: HashMap<String, String>,
    },

    /// Provides a localized error message safe to return to users.
    ///
    /// Contains a translated error message in the specified locale that can be displayed directly
    /// to end users. This is typically included alongside the developer-facing English message in
    /// the main Detail struct.
    #[serde(rename = "type.googleapis.com/google.rpc.LocalizedMessage")]
    LocalizedMessage {
        /// The locale following BCP 47 specification.
        ///
        /// Examples include `en-US`, `fr-CH`, `es-MX`. Indicates which language the localized
        /// message is in.
        #[serde(
            serialize_with = "crate::places_new::serde::serialize_locale",
            deserialize_with = "crate::places_new::serde::deserialize_locale"
        )]
        locale: icu_locale::Locale,

        /// The localized error message in the specified locale.
        ///
        /// A user-friendly error message translated to the language specified by locale.
        message: String,
    },

    /// Provides links to documentation or for performing out-of-band actions.
    ///
    /// Contains URLs pointing to additional information or actions the user can take.
    ///
    /// For example, if a quota check fails because a service isn't enabled, this might include a
    /// link directly to the developer console page where the service can be enabled.
    #[serde(rename = "type.googleapis.com/google.rpc.Help")]
    Help {
        /// URLs pointing to additional information on handling the error.
        ///
        /// Each link includes a description and URL to help users resolve the error or understand
        /// it better.
        links: Vec<Link>,
    },

    /// Describes when clients can retry a failed request.
    ///
    /// Provides guidance on retry timing using exponential backoff. Clients should wait at least
    /// the specified delay before retrying, then use exponential backoff for subsequent retries
    /// until reaching maximum retry attempts or delay cap.
    #[serde(rename = "type.googleapis.com/google.rpc.RetryInfo")]
    RetryInfo {
        /// Clients should wait at least this long between retrying the same request.
        ///
        /// Duration in seconds (with optional fractional nanoseconds) indicating the minimum delay
        /// before retry. This forms the base for exponential backoff calculations in subsequent
        /// retries.
        #[serde(rename = "retryDelay")]
        retry_delay: String,
    },

    /// Additional debugging information provided by the server.
    ///
    /// Contains stack traces and other debugging details to help diagnose errors. Typically only
    /// included in development or when additional debugging context is needed.
    #[serde(rename = "type.googleapis.com/google.rpc.DebugInfo")]
    DebugInfo {
        /// Stack trace entries indicating where the error occurred.
        ///
        /// Ordered list of stack trace strings showing the call path that led to the error.
        #[serde(default, rename = "stackEntries")]
        stack_entries: Vec<String>,

        /// Additional debugging information provided by the server.
        ///
        /// Free-form text providing extra context for debugging purposes.
        #[serde(default)]
        detail: String,
    },

    /// Describes how a quota check failed.
    ///
    /// Indicates which quota was exceeded and provides details about the violation.
    ///
    /// For example, if a daily API call limit was exceeded, this would contain the project ID and
    /// description of the quota limit. May appear alongside `RetryInfo` and Help details for quota
    /// failures.
    #[serde(rename = "type.googleapis.com/google.rpc.QuotaFailure")]
    QuotaFailure {
        /// All quota violations that occurred.
        ///
        /// One or more violations describing which quotas were exceeded and why.
        violations: Vec<QuotaViolation>,
    },

    /// Describes what preconditions have failed.
    ///
    /// Used when an operation cannot proceed because required preconditions aren't met.
    ///
    /// For example, if an RPC requires Terms of Service acceptance, this would list the TOS
    /// violation.
    #[serde(rename = "type.googleapis.com/google.rpc.PreconditionFailure")]
    PreconditionFailure {
        /// All precondition violations.
        ///
        /// One or more violations describing which preconditions failed and why.
        violations: Vec<PreconditionViolation>,
    },

    /// Describes violations in a client request's syntactic aspects.
    ///
    /// Focuses on syntax and structure issues in the request body, such as invalid field values or
    /// malformed data. Each violation identifies the specific field and explains what's wrong with
    /// it.
    #[serde(rename = "type.googleapis.com/google.rpc.BadRequest")]
    BadRequest {
        /// All field violations in the request.
        ///
        /// One or more violations describing which request fields are invalid and why.
        #[serde(rename = "fieldViolations")]
        field_violations: Vec<FieldViolation>,
    },

    /// Contains metadata about the request for debugging and feedback.
    ///
    /// Provides an opaque request ID and serving data that clients can include when filing bugs or
    /// providing feedback. The request ID can be used to identify requests in service logs.
    #[serde(rename = "type.googleapis.com/google.rpc.RequestInfo")]
    RequestInfo {
        /// An opaque string for identifying requests in logs.
        ///
        /// Should only be interpreted by the service that generated it. Used to correlate client
        /// reports with server-side logs.
        #[serde(rename = "requestId")]
        request_id: String,

        /// Data used to serve this request.
        ///
        /// Opaque data that can be sent back to the service provider for debugging, such as
        /// encrypted stack traces or internal routing information.
        #[serde(default, rename = "servingData")]
        serving_data: String,
    },

    /// Describes the resource that is being accessed.
    ///
    /// Provides information about which resource caused the error, including its type, name, owner,
    /// and a description of the access issue. Useful for permission denied and not found errors.
    #[serde(rename = "type.googleapis.com/google.rpc.ResourceInfo")]
    ResourceInfo {
        /// The type of resource being accessed.
        ///
        /// Human-readable type like "sql table" or "cloud storage bucket", or a type URL like
        /// `type.googleapis.com/google.pubsub.v1.Topic`.
        #[serde(rename = "resourceType")]
        resource_type: String,

        /// The name of the resource being accessed.
        ///
        /// For example, a shared calendar name like
        /// `example.com_4fghdhgsrgh@group.calendar.google.com`.
        #[serde(rename = "resourceName")]
        resource_name: String,

        /// The owner of the resource.
        ///
        /// Optional field in the format "user:<owner email>" or "project:<project id>".
        #[serde(default)]
        owner: String,

        /// Describes the error encountered when accessing this resource.
        ///
        /// Explains what permissions or conditions are needed. For example, might indicate that
        /// updating a cloud project requires "writer" permission.
        #[serde(default)]
        description: String,
    },

    /// Unknown or unrecognized error detail type.
    ///
    /// Used when the "@type" field contains a value not explicitly handled by this enum.
    /// Allows forward compatibility with new error detail types added to the API.
    #[serde(other)]
    Unknown,
}

// -------------------------------------------------------------------------------------------------
//
/// A URL link with description.
///
/// Provides a clickable link to external resources like documentation or console pages.
///
/// Used within Help error details to guide users toward resolving errors.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize, Getters)]
pub struct Link {
    /// Describes what the link offers.
    ///
    /// Human-readable text explaining where the link goes and why it's relevant.
    #[getset(get = "pub")]
    pub description: String,

    /// The URL of the link.
    ///
    /// Full URL to the external resource, typically HTTPS.
    #[getset(get = "pub")]
    pub url: String,
}

// -------------------------------------------------------------------------------------------------
//
/// A single quota violation describing an exceeded limit.
///
/// Provides detailed information about which quota was exceeded, including the metric, limit ID,
/// current value, and dimensional breakdown.
///
/// Used within `QuotaFailure` to explain quota errors.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize, Getters)]
pub struct QuotaViolation {
    /// The subject on which the quota check failed.
    ///
    /// Identifies what exceeded the quota. Examples include `clientip:<ip address>` or
    /// `project:<Google developer project id>`.
    #[getset(get = "pub")]
    pub subject: String,

    /// A description of how the quota check failed.
    ///
    /// Human-readable explanation that can help clients find documentation about the quota or
    /// identify which limit to adjust.
    ///
    /// Examples: `Service disabled` or `Daily Limit for read operations exceeded`.
    #[getset(get = "pub")]
    pub description: String,

    /// The API Service from which the violation originates.
    ///
    /// In some cases, quota issues originate from a dependency rather than the called service.
    ///
    /// For example, if calling Kubernetes Engine API (container.googleapis.com) but the quota
    /// violation occurs in Compute Engine API, this field would be `compute.googleapis.com`.
    #[serde(default, rename = "apiService")]
    #[getset(get = "pub")]
    pub api_service: String,

    /// The metric of the violated quota.
    ///
    /// Named counter measuring usage, such as `compute.googleapis.com/cpus_per_vm_family` or
    /// `storage.googleapis.com/internet_egress_bandwidth`.
    #[serde(default, rename = "quotaMetric")]
    #[getset(get = "pub")]
    pub quota_metric: String,

    /// The unique identifier of the violated quota.
    ///
    /// Also known as "limit name", this uniquely identifies a quota within an API service.
    ///
    /// Example: `CPUS-PER-VM-FAMILY-per-project-region`.
    #[serde(default, rename = "quotaId")]
    #[getset(get = "pub")]
    pub quota_id: String,

    /// The dimensions of the violated quota.
    ///
    /// For non-global quotas, specifies which aspects the counter applies to.
    ///
    /// For example, "CPUs per region per VM family" might have dimensions `{"region":
    /// "us-central1", "vm_family": "n1"}`.
    ///
    /// Global quotas have empty dimensions.
    #[serde(default, rename = "quotaDimensions")]
    #[getset(get = "pub")]
    pub quota_dimensions: HashMap<String, String>,

    /// The enforced quota value at the time of the failure.
    ///
    /// The limit that was exceeded. For example, if the CPU quota is 10 and this violation
    /// occurred, this field would be 10.
    #[serde(default, rename = "quotaValue")]
    #[getset(get_copy = "pub")]
    pub quota_value: i64,

    /// The new quota value being rolled out.
    ///
    /// If a quota rollout is in progress, this contains the value that will be enforced once the
    /// rollout completes.
    ///
    /// For example, if rolling out a change from 10 to 20 CPUs, this would be 20. Not set if no
    /// rollout is in progress.
    #[serde(default, rename = "futureQuotaValue")]
    #[getset(get = "pub")]
    pub future_quota_value: Option<i64>,
}

// -------------------------------------------------------------------------------------------------
//
/// A single precondition failure.
///
/// Describes which precondition wasn't met and why the operation cannot proceed.
///
/// Used within `PreconditionFailure` to explain what needs to be fixed before retrying.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize, Getters)]
pub struct PreconditionViolation {
    /// The type of precondition failure.
    ///
    /// Service-specific enum type defining supported precondition violations. Example: `TOS` for
    /// Terms of Service violation.
    #[serde(rename = "type")]
    #[getset(get = "pub")]
    pub violation_type: String,

    /// The subject, relative to the type, that failed.
    ///
    /// Identifies which specific instance of the precondition failed. For example,
    /// `google.com/cloud` relative to type `TOS` indicates which terms of service.
    #[getset(get = "pub")]
    pub subject: String,

    /// A description of how the precondition failed.
    ///
    /// Human-readable explanation that developers can use to understand how to fix the failure.
    /// Example: `Terms of service not accepted`.
    #[getset(get = "pub")]
    pub description: String,
}

// -------------------------------------------------------------------------------------------------
//
/// A single bad request field violation.
///
/// Identifies a specific field in the request that has invalid syntax or value, along with an
/// explanation of what's wrong. Used within `BadRequest` to enumerate all validation errors.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize, Getters)]
pub struct FieldViolation {
    /// Path to the field in the request body.
    ///
    /// Dot-separated identifiers identifying a protocol buffer field.
    ///
    /// For example: `fullName` for a top-level field, `emailAddresses[1].email` for a nested field
    /// in the second array element, or `emailAddresses[3].type[2]` for further nesting.
    #[getset(get = "pub")]
    pub field: String,

    /// A description of why the request element is bad.
    ///
    /// Human-readable explanation of the validation error for this field.
    #[getset(get = "pub")]
    pub description: String,

    /// The reason for the field-level error as an `UPPER_SNAKE_CASE` constant.
    ///
    /// Uniquely identifies the type of field violation within the `ErrorInfo` domain. Must be at
    /// most 63 characters and match `[A-Z][A-Z0-9_]+[A-Z0-9]`.
    #[serde(default)]
    #[getset(get = "pub")]
    pub reason: String,

    /// Localized error message for this field error.
    ///
    /// Optional localized message that is safe to return to API consumers, providing a translated
    /// explanation of the field violation.
    #[serde(default, rename = "localizedMessage")]
    #[getset(get = "pub")]
    pub localized_message: Option<LocalizedMessageInfo>,
}

// -------------------------------------------------------------------------------------------------
//
/// Localized error message information.
///
/// Contains a locale identifier and translated message. Used within `FieldViolation` to provide
/// user-facing localized descriptions of validation errors.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize, Getters)]
pub struct LocalizedMessageInfo {
    /// The locale following BCP 47 specification.
    ///
    /// Examples: `en-US`, `fr-CH`, `es-MX`.
    #[serde(
        serialize_with = "crate::places_new::serde::serialize_locale",
        deserialize_with = "crate::places_new::serde::deserialize_locale"
    )]
    #[getset(get = "pub")]
    pub locale: icu_locale::Locale,

    /// The localized error message in the specified locale.
    ///
    /// User-friendly error message translated to the language indicated by locale.
    #[getset(get = "pub")]
    pub message: String,
}

// -------------------------------------------------------------------------------------------------
//
/// Canonical gRPC error codes.
///
/// Represents the standard error codes used across Google APIs.
///
/// When multiple codes could apply, services should return the most specific one (e.g., prefer
/// `OUT_OF_RANGE` over `FAILED_PRECONDITION`, or `NOT_FOUND` over `FAILED_PRECONDITION`).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[repr(u16)]
pub enum Code {
    /// Not an error; returned on success.
    ///
    /// Indicates the operation completed successfully without any issues.
    ///
    /// HTTP Mapping: 200 OK
    #[serde(rename = "0")]
    Ok = 0,

    /// The operation was cancelled, typically by the caller.
    ///
    /// Indicates the client explicitly cancelled the operation before completion.
    ///
    /// HTTP Mapping: 499 Client Closed Request
    #[serde(rename = "1")]
    Cancelled = 1,

    /// Unknown error.
    ///
    /// Used when a Status from another address space belongs to an unknown error space, or when
    /// APIs don't return enough error information to determine a specific code.
    ///
    /// HTTP Mapping: 500 Internal Server Error
    #[serde(rename = "2")]
    Unknown = 2,

    /// The client specified an invalid argument.
    ///
    /// Indicates arguments that are problematic regardless of system state, such as malformed file
    /// names. This differs from `FAILED_PRECONDITION` which indicates state-dependent issues.
    ///
    /// HTTP Mapping: 400 Bad Request
    #[serde(rename = "3")]
    InvalidArgument = 3,

    /// The deadline expired before the operation could complete.
    ///
    /// May be returned even if the operation completed successfully but the response was delayed
    /// long enough for the deadline to expire.
    ///
    /// HTTP Mapping: 504 Gateway Timeout
    #[serde(rename = "4")]
    DeadlineExceeded = 4,

    /// Some requested entity was not found.
    ///
    /// Used when a specific resource doesn't exist. Also used for entire classes of users in
    /// gradual rollouts or undocumented allowlists (not `PERMISSION_DENIED`).
    ///
    /// HTTP Mapping: 404 Not Found
    #[serde(rename = "5")]
    NotFound = 5,

    /// The entity that a client attempted to create already exists.
    ///
    /// Indicates a conflict where the resource being created is already present.
    ///
    /// HTTP Mapping: 409 Conflict
    #[serde(rename = "6")]
    AlreadyExists = 6,

    /// The caller does not have permission to execute the specified operation.
    ///
    /// Must not be used for resource exhaustion (use `RESOURCE_EXHAUSTED`) or unauthenticated
    /// requests (use `UNAUTHENTICATED`). Does not imply the request is valid or that the entity
    /// exists.
    ///
    /// HTTP Mapping: 403 Forbidden
    #[serde(rename = "7")]
    PermissionDenied = 7,

    /// Some resource has been exhausted.
    ///
    /// Examples include per-user quota exceeded or entire file system out of space.
    ///
    /// HTTP Mapping: 429 Too Many Requests
    #[serde(rename = "8")]
    ResourceExhausted = 8,

    /// Operation was rejected because the system is not in a required state.
    ///
    /// Used when the system state must be explicitly fixed before retrying. For example, attempting
    /// to delete a non-empty directory.
    ///
    /// Use `UNAVAILABLE` for transient failures where retry might succeed, and `ABORTED` for
    /// failures requiring higher-level retry.
    ///
    /// HTTP Mapping: 400 Bad Request
    #[serde(rename = "9")]
    FailedPrecondition = 9,

    /// The operation was aborted.
    ///
    /// Typically due to concurrency issues like sequencer check failures or transaction aborts.
    /// Clients should retry at a higher level (e.g., restart read-modify-write).
    ///
    /// HTTP Mapping: 409 Conflict
    #[serde(rename = "10")]
    Aborted = 10,

    /// Operation was attempted past the valid range.
    ///
    /// Unlike `INVALID_ARGUMENT`, this indicates a problem fixable by system state changes.
    ///
    /// For example, reading past current file size generates `OUT_OF_RANGE`, while reading at an
    /// offset outside the valid range generates `INVALID_ARGUMENT`.
    ///
    /// HTTP Mapping: 400 Bad Request
    #[serde(rename = "11")]
    OutOfRange = 11,

    /// The operation is not implemented or not supported/enabled.
    ///
    /// Indicates the service doesn't support this operation at all.
    ///
    /// HTTP Mapping: 501 Not Implemented
    #[serde(rename = "12")]
    Unimplemented = 12,

    /// Internal errors.
    ///
    /// Indicates some invariants expected by the underlying system have been broken. Reserved for
    /// serious errors.
    ///
    /// HTTP Mapping: 500 Internal Server Error
    #[serde(rename = "13")]
    Internal = 13,

    /// The service is currently unavailable.
    ///
    /// Most likely a transient condition that can be corrected by retrying with backoff. Note that
    /// retrying non-idempotent operations is not always safe.
    ///
    /// HTTP Mapping: 503 Service Unavailable
    #[serde(rename = "14")]
    Unavailable = 14,

    /// Unrecoverable data loss or corruption.
    ///
    /// Indicates permanent data integrity issues.
    ///
    /// HTTP Mapping: 500 Internal Server Error
    #[serde(rename = "15")]
    DataLoss = 15,

    /// The request does not have valid authentication credentials.
    ///
    /// Indicates missing or invalid authentication for the operation.
    ///
    /// HTTP Mapping: 401 Unauthorized
    #[serde(rename = "16")]
    Unauthenticated = 16,
}

fn deserialize_code<'de, D>(deserializer: D) -> Result<Code, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let code_num: u16 = Deserialize::deserialize(deserializer)?;
    match code_num {
        200 => Ok(Code::Ok),
        499 => Ok(Code::Cancelled),
        400 => Ok(Code::InvalidArgument),
        504 => Ok(Code::DeadlineExceeded),
        404 => Ok(Code::NotFound),
        403 => Ok(Code::PermissionDenied),
        429 => Ok(Code::ResourceExhausted),
        409 => Ok(Code::Aborted),
        501 => Ok(Code::Unimplemented),
        500 => Ok(Code::Internal),
        503 => Ok(Code::Unavailable),
        401 => Ok(Code::Unauthenticated),
        _ => Ok(Code::Unknown), // Fallback for unknown codes
    }
}

impl std::fmt::Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Ok => "OK",
            Self::Cancelled => "CANCELLED",
            Self::Unknown => "UNKNOWN",
            Self::InvalidArgument => "INVALID_ARGUMENT",
            Self::DeadlineExceeded => "DEADLINE_EXCEEDED",
            Self::NotFound => "NOT_FOUND",
            Self::AlreadyExists => "ALREADY_EXISTS",
            Self::PermissionDenied => "PERMISSION_DENIED",
            Self::ResourceExhausted => "RESOURCE_EXHAUSTED",
            Self::FailedPrecondition => "FAILED_PRECONDITION",
            Self::Aborted => "ABORTED",
            Self::OutOfRange => "OUT_OF_RANGE",
            Self::Unimplemented => "UNIMPLEMENTED",
            Self::Internal => "INTERNAL",
            Self::Unavailable => "UNAVAILABLE",
            Self::DataLoss => "DATA_LOSS",
            Self::Unauthenticated => "UNAUTHENTICATED",
        };
        write!(f, "{name}")
    }
}

impl crate::traits::ClassifiableError<'_, Self> for Code {
    /// Classifies the gRPC error code as transient or permanent.
    ///
    /// * Transient errors indicate temporary conditions that may resolve on retry, such as service
    ///   unavailability or resource exhaustion.
    ///
    /// * Permanent errors indicate issues that require changes to the request or system state
    ///   before retrying.
    fn classify(&self) -> ClassifiedError<'_, Self> {
        match self {
            // Transient errors. May succeed on retry.
            Self::Unavailable | Self::ResourceExhausted | Self::Aborted | Self::DeadlineExceeded => {
                ClassifiedError::Transient(self)
            }

            // Could be either, but treat as transient since client might retry.
            Self::Cancelled |
            Self::Unknown => ClassifiedError::Transient(self),

            // All other errors are permanent - require request or state changes.
            Self::Ok |
            Self::InvalidArgument |
            Self::NotFound |
            Self::AlreadyExists |
            Self::PermissionDenied |
            Self::FailedPrecondition |
            Self::OutOfRange |
            Self::Unimplemented |
            Self::Internal |
            Self::DataLoss |
            Self::Unauthenticated => ClassifiedError::Permanent(self),
        }
    }
}