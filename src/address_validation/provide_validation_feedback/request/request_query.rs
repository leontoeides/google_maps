use bon::Builder;
use crate::address_validation::ValidationConclusion;
use getset::{CopyGetters, Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// This `struct` is submitted to the Google Maps server, in the HTTP `POST`
/// request's body, in JSON format.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize, Builder, CopyGetters, Getters, MutGetters, Setters)]
#[serde(rename_all = "camelCase")]
pub struct RequestQuery {
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
} // struct RequestQuery

// -----------------------------------------------------------------------------

use crate::address_validation::provide_validation_feedback::Request;

impl std::convert::From<&Request<'_>> for RequestQuery {
    fn from(request: &Request) -> Self {
        Self {
            conclusion: request.conclusion,
            response_id: request.response_id.clone(),
        } // Self
    } // fn
} // impl