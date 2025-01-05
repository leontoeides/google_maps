//! Once you have a `place_id` from a Place Search, you can request more details
//! about a particular establishment or point of interest by initiating a Place
//! Details request. A Place Details request returns more comprehensive
//! information about the indicated place such as its complete address, phone
//! number, user rating and reviews.

pub mod field;
pub mod request;
pub mod response;
pub mod sort_order;

// -----------------------------------------------------------------------------

pub use crate::places::{
    error::Error,
    status::Status
};

pub use crate::places::place_details::{
    field::Field,
    request::Request,
    response::Response,
    sort_order::SortOrder,
};