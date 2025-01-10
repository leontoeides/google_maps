//! **Look in this module for documentation on building your Address Validation
//! API _Provide Validation Feedback_ query**. In particular, look at the
//! _Request_ struct for examples of the builder pattern. This module contains
//! the tools (enums, structs, methods) for building your Google Maps Platform
//! request.

pub(super) mod validation_conclusion;

mod end_point;
mod query_string;
mod request_body;
mod request_query;
mod validatable;

#[cfg(feature = "reqwest")]
mod execute;

pub use crate::address_validation::provide_validation_feedback::request::{
    request_builder::SetClient,
    request_query::RequestQuery,
    RequestBuilder as RequestBuilderExport,
    validation_conclusion::ValidationConclusion,
};

// -----------------------------------------------------------------------------

use bon::Builder;
use getset::{CopyGetters, Getters, MutGetters, Setters};

// -----------------------------------------------------------------------------
//
/// **Look at this `Request` struct for documentation on how to build your
/// _Address Validation_ _Provide Validation Feedback_ query**. The methods
/// implemented for this struct are what's used to build your request.
#[derive(Debug, Builder, CopyGetters, Getters, MutGetters, Setters)]
pub struct Request<'r> {
    /// This structure contains the application's API key and other
    /// user-definable settings such as "maximum retries," and most importantly,
    /// the [reqwest](https://crates.io/crates/reqwest) client.
    pub client: &'r crate::Client,

    // -------------------------------------------------------------------------
    // The below fields will be converted into a `RequestQuery` struct, then to
    // JSON, and then submitted in to Google Maps in the HTTP POST request body:
    // -------------------------------------------------------------------------

    /// Required. The outcome of the sequence of validation attempts.
    ///
    /// If this field is set to `VALIDATION_CONCLUSION_UNSPECIFIED`, an
    /// `INVALID_ARGUMENT` error will be returned.
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub conclusion: ValidationConclusion,

    /// Required. The ID of the response that this feedback is for. This should
    /// be the
    /// [responseId][google.maps.addressvalidation.v1.ValidateAddressRequest.response_id]
    /// from the first response in a series of address validation attempts.
    #[builder(into)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub response_id: String,
} // struct Request