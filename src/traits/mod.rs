//! Contains `trait` declarations that are used internally in the `google_maps`
//! crate, particularly for validating and converting request structs into
//! HTTP queries.

pub mod classifiable_error;
pub mod end_point;
pub mod query_string;
pub mod query_url;
pub mod validatable;

// -----------------------------------------------------------------------------

pub use crate::traits::classifiable_error::ClassifiableError;
pub use crate::traits::end_point::EndPoint;
pub use crate::traits::query_string::QueryString;
pub use crate::traits::query_url::QueryUrl;
pub use crate::traits::validatable::Validatable;
