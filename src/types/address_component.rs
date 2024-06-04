//! Contains the separate components applicable to an address.

use crate::types::PlaceType;
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// Contains the separate components applicable to this address.
///
/// Note the following facts about the `address_components` array:
///
/// * The array of address components may contain more components than the
///   `formatted_address`.
///
/// * The array does not necessarily include all the political entities that
///   contain an address, apart from those included in the `formatted_address`.
///   To retrieve all the political entities that contain a specific address,
///   you should use reverse geocoding, passing the latitude/longitude of the
///   address as a parameter to the request.
///
/// * The format of the response is not guaranteed to remain the same between
///   requests. In particular, the number of `address_components` varies based
///   on the address requested and can change over time for the same address. A
///   component can change position in the array. The type of the component can
///   change. A particular component may be missing in a later response.
///
/// To handle the array of components, you should parse the response and select
/// appropriate values via expressions. See the guide to [parsing a
/// response](https://developers.google.com/maps/documentation/geocoding/web-service-best-practices#Parsing).

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AddressComponent {
    /// The full text description or name of the address component as returned
    /// by the Geocoder.
    pub long_name: String,

    /// An abbreviated textual name for the address component, if available. For
    /// example, an address component for the state of Alaska may have a
    /// `long_name` of "Alaska" and a `short_name` of "AK" using the 2-letter
    /// postal abbreviation.
    pub short_name: String,

    /// An array indicating the type of the address component. See the list of
    /// [supported
    /// types](https://developers.google.com/places/web-service/supported_types).
    #[serde(default)]
    pub types: Vec<PlaceType>,
} // struct
