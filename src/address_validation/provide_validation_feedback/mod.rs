//! Feedback about the outcome of the sequence of validation attempts.

pub mod request;
pub mod response;

// -----------------------------------------------------------------------------

pub use crate::address_validation::provide_validation_feedback::{
    request::{
        Request,
        RequestQuery,
        ValidationConclusion,
    },
    response::{
        Response,
    },
};