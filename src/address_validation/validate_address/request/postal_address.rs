use bon::Builder;
use getset::{CopyGetters, Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// “The total length of the fields in this input must not exceed 280
/// characters.”
pub const MAX_CHARS: usize = 280;

// -----------------------------------------------------------------------------
//
/// Represents a postal address, e.g. for postal delivery or payments addresses.
/// Given a postal address, a postal service can deliver items to a premise,
/// P.O. Box or similar. It is not intended to model geographical locations
/// (roads, towns, mountains).
///
/// In typical usage an address would be created via user input or from
/// importing existing data, depending on the type of process.
///
/// Advice on address input / editing: - Use an internationalization-ready
/// address widget such as
/// [https://github.com/google/libaddressinput](https://github.com/google/libaddressinput)) -
/// Users should not be presented with UI elements for input or editing of
/// fields outside countries where that field is used.
///
/// For more guidance on how to use this schema, please see:
/// <https://support.google.com/business/answer/6397478>
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize, Builder, CopyGetters, Getters, MutGetters, Setters)]
#[serde(rename_all = "camelCase")]
pub struct PostalAddress {
    /// The schema revision of the `PostalAddress`. Any value other than 0 will
    /// cause the API to return an `INVALID_ARGUMENT` error.
    #[serde(default)]
    #[builder(into)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub revision: Option<usize>,

    /// Optional. CLDR region code of the country/region of the address. See
    /// <https://cldr.unicode.org/> and
    /// <https://www.unicode.org/cldr/charts/30/supplemental/territory_information.html>
    /// for details. Example: "CH" for Switzerland. If the region code is not
    /// provided, it will be inferred from the address. For best performance, it
    /// is recommended to include the region code if you know it. Having
    /// inconsistent or repeated regions can lead to poor performance, for
    /// example, if the `addressLines` already includes the region, do not
    /// provide the region code again in this field. Supported regions can be
    /// found in the [FAQ](https://developers.google.com/maps/documentation/address-validation/coverage).
    #[serde(default)]
    #[builder(into)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub region_code: Option<String>,

    /// The language code in the input address is reserved for future uses and
    /// is ignored today. The API returns the address in the appropriate
    /// language for where the address is located.
    #[serde(default)]
    #[builder(into)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub language_code: Option<String>,

    /// Optional. Postal code of the address. Not all countries use or require
    /// postal codes to be present, but where they are used, they may trigger
    /// additional validation with other parts of the address (e.g. state/zip
    /// validation in the U.S.A.).
    #[serde(default)]
    #[builder(into)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub postal_code: Option<String>,

    /// Optional. Additional, country-specific, sorting code. This is not used
    /// in most regions. Where it is used, the value is either a string like
    /// "CEDEX", optionally followed by a number (e.g. "CEDEX 7"), or just a
    /// number alone, representing the "sector code" (Jamaica), "delivery area
    /// indicator" (Malawi) or "post office indicator" (e.g. Côte d'Ivoire).
    #[serde(default)]
    #[builder(into)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub sorting_code: Option<String>,

    /// Optional. Highest administrative subdivision which is used for postal
    /// addresses of a country or region. For example, this can be a state, a
    /// province, an oblast, or a prefecture. Specifically, for Spain this is
    /// the province and not the autonomous community (e.g. "Barcelona" and not
    /// "Catalonia"). Many countries don't use an administrative area in postal
    /// addresses. E.g. in Switzerland this should be left unpopulated.
    #[serde(default)]
    #[builder(into)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub administrative_area: Option<String>,

    /// Optional. Generally refers to the city/town portion of the address.
    /// Examples: US city, IT comune, UK post town. In regions of the world
    /// where localities are not well defined or do not fit into this structure
    /// well, leave locality empty and use addressLines.
    #[serde(default)]
    #[builder(into)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub locality: Option<String>,

    /// Optional. Sublocality of the address. For example, this can be
    /// neighborhoods, boroughs, districts.
    #[serde(default)]
    #[builder(into)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub sublocality: Option<String>,

    /// Required. Unstructured address lines describing the lower levels of an
    /// address.
    ///
    /// Because values in addressLines do not have type information and may
    /// sometimes contain multiple values in a single field (e.g. "Austin, TX"),
    /// it is important that the line order is clear. The order of address lines
    /// should be "envelope order" for the country/region of the address.
    ///
    /// The minimum permitted structural representation of an address consists
    /// of all information placed in the `addressLines`. If a `regionCode` is
    /// not provided, the region is inferred from the address lines.
    ///
    /// Creating an address only containing `addressLines`, and then geocoding
    /// is the recommended way to handle completely unstructured addresses (as
    /// opposed to guessing which parts of the address should be localities or
    /// administrative areas).
    #[serde(default)]
    #[builder(default = Vec::new())]
    #[builder(with = |v: Vec<impl ToString>| v.into_iter().map(|s| s.to_string()).collect::<Vec<String>>())]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub address_lines: Vec<String>,

    /// Please avoid setting this field. The Address Validation API does not
    /// currently use it. Although at this time the API will not reject requests
    /// with this field set, the information will be discarded and will not be
    /// returned in the response.
    #[serde(default)]
    #[builder(default = Vec::new())]
    #[builder(with = |v: Vec<impl ToString>| v.into_iter().map(|s| s.to_string()).collect::<Vec<String>>())]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub recipients: Vec<String>,

    /// Please avoid setting this field. The Address Validation API does not
    /// currently use it. Although at this time the API will not reject requests
    /// with this field set, the information will be discarded and will not be
    /// returned in the response.
    #[serde(default)]
    #[builder(into)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub organization: Option<String>,
} // struct PostalAddress

// -----------------------------------------------------------------------------

impl PostalAddress {
    /// Calculates the total length of all the fields in characters.
    ///
    /// This is used to validate the request before sending it to the Google
    /// Maps server:
    ///
    ///     **“The total length of the fields in this input must not exceed 280
    ///     characters.”**
    #[must_use] pub fn len(&self) -> usize {
        let mut len: usize = 0;

        len += self.revision.as_ref().map_or(0, |int| int.to_string().len());
        len += self.region_code.as_ref().map_or(0, String::len);
        len += self.language_code.as_ref().map_or(0, String::len);
        len += self.postal_code.as_ref().map_or(0, String::len);
        len += self.sorting_code.as_ref().map_or(0, String::len);
        len += self.administrative_area.as_ref().map_or(0, String::len);
        len += self.locality.as_ref().map_or(0, String::len);
        len += self.sublocality.as_ref().map_or(0, String::len);
        len += self.address_lines.iter().map(String::len).sum::<usize>();
        len += self.recipients.iter().map(String::len).sum::<usize>();
        len += self.organization.as_ref().map_or(0, String::len);

        len
    } // fn

    /// Returns whether the `PostalAddress` is empty or not.
    #[must_use] pub fn is_empty(&self) -> bool { self.len() == 0 }
} // impl