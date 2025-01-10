//! **Look in this module for documentation on building your _Address Validation
//! API_ query**. In particular, look at the _Request_ struct for examples of
//! the builder pattern. This module contains the tools (enums, structs,
//! methods) for building your Google Maps Platform request.

pub(crate) mod postal_address;
pub(super) mod language_options;
pub(super) mod request_query;

mod end_point;
mod query_string;
mod request_body;
mod validatable;

#[cfg(feature = "reqwest")]
mod execute;

pub use crate::address_validation::validate_address::request::{
    language_options::LanguageOptions,
    postal_address::PostalAddress,
    request_builder::SetClient,
    request_query::RequestQuery,
    RequestBuilder as RequestBuilderExport,
};

// -----------------------------------------------------------------------------

use bon::Builder;
use getset::{CopyGetters, Getters, MutGetters, Setters};

// -----------------------------------------------------------------------------
//
/// **Look at this `Request` struct for documentation on how to build your
/// _Address Validation_ _Validate Address_ query**. The methods implemented for
/// this struct are what's used to build your request.
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

    /// Required. The address being validated. Unformatted addresses should be
    /// submitted via
    /// [addressLines](https://developers.google.com/maps/documentation/address-validation/reference/rest/v1/TopLevel/validateAddress#PostalAddress.FIELDS.address_lines).
    ///
    /// The total length of the fields in this input must not exceed 280
    /// characters.
    ///
    /// Supported regions can be found
    /// [here](https://developers.google.com/maps/documentation/address-validation/coverage).
    ///
    /// The `languageCode` value in the input address is reserved for future
    /// uses and is ignored today. The validated address result will be
    /// populated based on the preferred language for the given address, as
    /// identified by the system.
    ///
    /// The Address Validation API ignores the values in
    /// [recipients](https://developers.google.com/maps/documentation/address-validation/reference/rest/v1/TopLevel/validateAddress#PostalAddress.FIELDS.recipients) and [organization](https://developers.google.com/maps/documentation/address-validation/reference/rest/v1/TopLevel/validateAddress#PostalAddress.FIELDS.organization).
    /// Any values in those fields will be discarded and not returned. Please do
    /// not set them.
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub address: PostalAddress,

    /// This field must be empty for the first address validation request. If
    /// more requests are necessary to fully validate a single address (for
    /// example if the changes the user makes after the initial validation
    /// need to be re-validated), then each followup request must populate this
    /// field with the responseId from the very first response in the validation
    /// sequence.
    #[builder(into)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub previous_response_id: Option<String>,

    /// Enables USPS CASS compatible mode.
    ///
    /// The United States Postal Service® (USPS®) maintains the [Coding Accuracy
    /// Support System (CASS™)](https://postalpro.usps.com/certifications/cass)
    /// to support and certify address validation providers.
    ///
    /// * **Note**: The Address Validation API is confirmed as a CASS-Certified™
    ///   service. This means the service is confirmed for its ability to fill
    ///   in missing address information, standardize it, and update it to the
    ///   most current and accurate address.
    ///
    /// * **Note**:  If you don't require CASS processing, or if you are
    ///   validating addresses outside of the US and PR regions, Google does not
    ///   recommend enabling this feature.
    ///
    /// This affects only the
    /// [google.maps.addressvalidation.v1.ValidationResult.usps_data](https://developers.google.com/maps/documentation/address-validation/reference/rest/v1/TopLevel/validateAddress#ValidationResult.FIELDS.usps_data)
    /// field of
    /// [google.maps.addressvalidation.v1.ValidationResult](https://developers.google.com/maps/documentation/address-validation/reference/rest/v1/TopLevel/validateAddress#ValidationResult).
    /// Note: for USPS CASS™ enabled requests for addresses in Puerto Rico, a
    /// [google.type.PostalAddress.region_code](https://developers.google.com/maps/documentation/address-validation/reference/rest/v1/TopLevel/validateAddress#PostalAddress.FIELDS.region_code)
    /// of the **address** must be provided as PR", or an
    /// [google.type.PostalAddress.administrative_area](https://developers.google.com/maps/documentation/address-validation/reference/rest/v1/TopLevel/validateAddress#PostalAddress.FIELDS.administrative_area)
    /// of the **address** must be provided as "Puerto Rico" (case-insensitive)
    /// or "PR".
    #[builder(into)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub enable_usps_cass: Option<bool>,

    /// Optional. Preview: This feature is in Preview (pre-GA). Pre-GA products
    /// and features might have limited support, and changes to pre-GA products
    /// and features might not be compatible with other pre-GA versions. Pre-GA
    /// Offerings are covered by the [Google Maps Platform Service Specific
    /// Terms](https://cloud.google.com/maps-platform/terms/maps-service-terms).
    /// For more information, see the
    /// [launch stage descriptions](https://developers.google.com/maps/launch-stages).
    ///
    /// Enables the Address Validation API to include additional information in the response.
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub language_options: Option<LanguageOptions>,

    /// Optional. A string which identifies an Autocomplete session for billing
    /// purposes. Must be a URL and filename safe base64 string with at most 36
    /// ASCII characters in length. Otherwise an `INVALID_ARGUMENT` error is
    /// returned.
    ///
    /// The session begins when the user makes an Autocomplete query, and
    /// concludes when they select a place and a call to Place Details or
    /// Address Validation is made. Each session can have multiple Autocomplete
    /// queries, followed by one Place Details or Address Validation request.
    /// The credentials used for each request within a session must belong to
    /// the same Google Cloud Console project. Once a session has concluded, the
    /// token is no longer valid; your app must generate a fresh token for each
    /// session. If the `sessionToken` parameter is omitted, or if you reuse a
    /// session token, the session is charged as if no session token was
    /// provided (each request is billed separately).
    ///
    /// Note: Address Validation can only be used in sessions with the
    /// Autocomplete (New) API, not the Autocomplete API. See
    /// <https://developers.google.com/maps/documentation/places/web-service/session-pricing>
    /// for more details.
    #[builder(into)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub session_token: Option<String>,
} // struct Request