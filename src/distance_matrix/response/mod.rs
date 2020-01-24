//! Resources (enums, structs) for the client process the Distance-Matrix API
//! response from the Google Cloud server.

mod element;
mod element_status;
mod row;
pub mod status;

use crate::distance_matrix::response::row::Row;
use crate::distance_matrix::response::status::Status;
use serde::{Serialize, Deserialize};

/// Distance Matrix responses contain the following root elements.

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Response {

    /// Contains an array of addresses as returned by the API from your original
    /// request. As with `origin_addresses`, these are localized if appropriate.
    pub destination_addresses: Vec<String>,

    /// Contains an array of addresses as returned by the API from your original
    /// request. These are formatted by the
    /// [geocoder](https://developers.google.com/maps/documentation/geocoding/)
    /// and localized according to the `language` parameter passed with the
    /// request.
    pub origin_addresses: Vec<String>,

    /// Contains an array of elements, which in turn each contain a `status`,
    /// `duration`, and `distance` element.
    pub rows: Vec<Row>,

    /// Contains metadata on the request.
    pub status: Status,

} // struct