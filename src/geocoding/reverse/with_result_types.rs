use crate::{geocoding::reverse::ReverseRequest, place_type::PlaceType};

impl<'a> ReverseRequest<'a> {

    /// Restricts the results from the geocoder to the specified result type(s).
    ///
    /// # Arguments:
    ///
    /// * `result_type` - A single result type filter.
    ///
    /// # Description:
    ///
    /// A filter of one or more result types. If the parameter contains multiple
    /// result types, the API returns all addresses that match any of the types.
    /// A note about processing: The `result_type` parameter does not _restrict_
    /// the search to the specified location type(s). Rather, the `result_type`
    /// acts as a post-search filter: the API fetches all results for the
    /// specified `latlng`, then discards those results that do not match the
    /// specified result type(s). The following values are supported:
    ///
    /// * `PlaceType::StreetAddress` indicates a precise street address.
    ///
    /// * `PlaceType::Route` indicates a named route (such as "US 101").
    ///
    /// * `PlaceType::Intersection` indicates a major intersection, usually of
    /// two major roads.
    ///
    /// * `PlaceType::Political` indicates a political entity. Usually, this
    /// type indicates a polygon of some civil administration.
    ///
    /// * `PlaceType::Country` indicates the national political entity, and is
    /// typically the highest order type returned by the Geocoder.
    ///
    /// * `PlaceType::AdministrativeAreaLevel1` indicates a first-order civil
    /// entity below the country level. Within the United States, these
    /// administrative levels are states. Not all nations exhibit these
    /// administrative levels. In most cases,
    /// `PlaceType::AdministrativeAreaLevel1` short names will closely match ISO
    /// 3166-2 subdivisions and other widely circulated lists; however this is
    /// not guaranteed as our geocoding results are based on a variety of
    /// signals and location data.
    ///
    /// * `PlaceType::AdministrativeAreaLevel2` indicates a second-order civil
    /// entity below the country level. Within the United States, these
    /// administrative levels are counties. Not all nations exhibit these
    /// administrative levels.
    ///
    /// * `PlaceType::AdministrativeAreaLevel3` indicates a third-order civil
    /// entity below the country level. This type indicates a minor civil
    /// division. Not all nations exhibit these administrative levels.
    ///
    /// * `PlaceType::AdministrativeAreaLevel4` indicates a fourth-order civil
    /// entity below the country level. This type indicates a minor civil
    /// division. Not all nations exhibit these administrative levels.
    ///
    /// * `PlaceType::AdministrativeAreaLevel5` indicates a fifth-order civil
    /// entity below the country level. This type indicates a minor civil
    /// division. Not all nations exhibit these administrative levels.
    ///
    /// * `PlaceType::ColloquialArea` indicates a commonly-used alternative name
    /// for the entity.
    ///
    /// * `PlaceType::Locality` indicates an incorporated city or town political
    /// entity.
    ///
    /// * `PlaceType::Sublocality` indicates a first-order civil entity below a
    /// locality. For some locations may receive one of the additional types:
    /// `PlaceType::SublocalityLevel1` to `PlaceType::SublocalityLevel5`. Each
    /// sublocality level is a civil entity. Larger numbers indicate a smaller
    /// geographic area.
    ///
    /// * `PlaceType::Neighborhood` indicates a named neighborhood.
    ///
    /// * `PlaceType::Premise` indicates a named location, usually a building or
    /// collection of buildings with a common name.
    ///
    /// * `PlaceType::Subpremise` indicates a first-order entity below a named
    /// location, usually a singular building within a collection of buildings
    /// with a common name.
    ///
    /// * `PlaceType::PostalCode` indicates a postal code as used to address
    /// postal mail within the country.
    ///
    /// * `PlaceType::NaturalFeature` indicates a prominent natural feature.
    ///
    /// * `PlaceType::Airport` indicates an airport.
    ///
    /// * `PlaceType::Park` indicates a named park.
    ///
    /// * `PlaceType::PointOfInterest` indicates a named point of interest.
    /// Typically, these "POI"s are prominent local entities that don't easily
    /// fit in another category, such as "Empire State Building" or "Eiffel
    /// Tower".
    ///
    /// If both `result_type` and `location_type` filters are present then the
    /// API returns only those results that match both the `result_type` and the
    /// `location_type values`. If none of the filter values are acceptable, the
    /// API returns `ZERO_RESULTS`.
    ///
    /// # Examples:
    ///
    /// * A single result type filter. This example restricts results to the
    /// neighbourhood:
    ///
    /// ```rust
    /// .with_result_type(PlaceType::Neighborhood)
    /// ```
    ///
    /// * Multiple component filters may be stacked together. This example
    /// restricts results to a neighborhood and a locality:
    ///
    /// ```rust
    /// .with_result_type(PlaceType::Neighborhood)
    /// .with_result_type(PlaceType::Locality)
    /// ```

    pub fn with_result_type(
        &'a mut self,
        result_type_element: PlaceType,
    ) -> &'a mut ReverseRequest {
        // Add result type to ReverseRequest struct.
        match &mut self.result_types {
            // If there are no filters in the request struct, initialize field:
            None => self.result_types = Some(vec![result_type_element]),
            // If there are already filters, append to them:
            Some(reverse_request_struct) => reverse_request_struct.push(result_type_element),
        } // match
        // Return modified ReverseRequest struct to caller.
        self
    } // fn

    /// Restricts the results from the geocoder to the specified result type(s).
    ///
    /// # Description
    ///
    /// A filter of one or more result types. If the parameter contains
    /// multiple results types, the API returns all addresses that match any of
    /// the types.
    ///
    /// If both `result_type` and `location_type` filters are present then the
    /// API returns only those results that match both the `result_type` and the
    /// `location_type values`. If none of the filter values are acceptable, the
    /// API returns `ZERO_RESULTS`.
    ///
    /// # Example:
    ///
    /// * Alternatively, multiple result type filters may be passed in a single
    /// method call by passing a Vec. This example restricts results a
    /// neighborhood and a locality:
    ///
    /// ```rust
    /// .with_components(&vec![
    ///     PlaceType::Neighborhood,
    ///     PlaceType::Locality,
    /// ])
    /// ```

    pub fn with_result_types(
        &'a mut self,
        result_type_slice: &[PlaceType],
    ) -> &'a mut ReverseRequest {
        // Add location types to ReverseRequest struct.
        match &mut self.result_types {
            // If there are no filters in the request struct, initialize field:
            None => self.result_types = Some(result_type_slice.to_vec()),
            // If there are already filters, append to them:
            Some(reverse_request_struct) => result_type_slice
                .iter()
                .for_each(|result_type| reverse_request_struct.push(result_type.to_owned())),
        } // match
        // Return modified ReverseRequest struct to caller.
        self
    } // fn

} // impl