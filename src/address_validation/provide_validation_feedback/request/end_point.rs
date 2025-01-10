#[cfg(feature = "reqwest")]
use crate::request_rate::api::Api;

// -----------------------------------------------------------------------------
//
/// Defines a Google Maps API HTTP end-point for a request.
///
/// This trait returns information needed to make connect HTTP `GET` requests to
/// their API end-point. It also includes some extra information for debugging
/// and rating-limiting.
impl crate::traits::EndPoint for crate::address_validation::provide_validation_feedback::Request<'_> {
    fn service_url() -> &'static str { "https://addressvalidation.googleapis.com/v1:provideValidationFeedback" }
    fn output_format() -> std::option::Option<&'static str> { None }
    #[cfg(feature = "reqwest")]
    fn title() -> &'static str { "Address Validation API Provide Validation Feedback" }
    #[cfg(feature = "reqwest")]
    fn apis() -> &'static [Api] { &[Api::All, Api::AddressValidation] }
} // impl