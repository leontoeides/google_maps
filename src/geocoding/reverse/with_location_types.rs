use crate::geocoding::reverse::ReverseRequest;
use crate::types::location_type::LocationType;

// -----------------------------------------------------------------------------

impl<'a> ReverseRequest<'a> {
    /// Restricts the results from the geocoder to the specified location
    /// type(s).
    ///
    /// # Arguments:
    ///
    /// * `location_type` - A single location-type filter.
    ///
    /// # Description:
    ///
    /// A filter of one or more location types. If the parameter contains
    /// multiple location types, the API returns all addresses that match any of
    /// the types. A note about processing: The `location_type` parameter does
    /// not _restrict_ the search to the specified location type(s). Rather, the
    /// `location_type` acts as a post-search filter: the API fetches all
    /// results for the specified `latlng`, then discards those results that do
    /// not match the specified location type(s).
    ///
    /// * `LocationType::RoofTop` returns only the addresses for which Google
    ///   has location information accurate down to street address precision.
    ///
    /// * `LocationType::RangeInterpolated` returns only the addresses that
    ///   reflect an approximation (usually on a road) interpolated between two
    ///   precise points (such as intersections). An interpolated range
    ///   generally indicates that rooftop geocodes are unavailable for a street
    ///   address.
    ///
    /// * `LocationType::GeometricCenter` returns only geometric centers of a
    ///   location such as a polyline (for example, a street) or polygon
    ///   (region).
    ///
    /// * `LocationType::Approximate` returns only the addresses that are
    ///   characterized as approximate.
    ///
    /// If both `result_type` and `location_type` filters are present then the
    /// API returns only those results that match both the `result_type` and the
    /// `location_type values`. If none of the filter values are acceptable, the
    /// API returns `ZERO_RESULTS`.
    ///
    /// # Examples:
    ///
    /// * A single location-type filter. This example restricts results to roof-
    ///   top results:
    ///
    /// ```rust
    /// .with_location_type(LocationType::RoofTop)
    /// ```
    ///
    /// * Multiple location type filters may be stacked together. This example
    ///   restricts results to roof-top and range-interpolated:
    ///
    /// ```rust
    /// .with_location_type(LocationType::RoofTop)
    /// .with_location_type(LocationType::RangeInterpolated)
    /// ```

    pub fn with_location_type(
        &'a mut self,
        location_type: impl Into<LocationType>
    ) -> &'a mut Self {
        self.location_types = vec![location_type.into()];
        // Return modified ReverseRequest struct to caller.
        self
    } // fn

    /// Restricts the results from the geocoder to the specified location
    /// type(s).
    ///
    /// # Description
    ///
    /// A filter of one or more location types. If the parameter contains
    /// multiple location types, the API returns all addresses that match any of
    /// the types.
    ///
    /// If both `result_type` and `location_type` filters are present then the
    /// API returns only those results that match both the `result_type` and the
    /// `location_type values`. If none of the filter values are acceptable, the
    /// API returns `ZERO_RESULTS`.
    ///
    /// # Example:
    ///
    /// * Alternatively, multiple location type filters may be passed in a
    ///   single method call by passing a slice. This example restricts results
    ///   to roof-top and range-interpolated:
    ///
    /// ```rust
    /// .with_location_types(&[
    ///     LocationType::RoofTop,
    ///     LocationType::RangeInterpolated,
    /// ])
    /// ```
    ///
    /// # Generics
    ///
    /// This method uses generics to improve ergonomics. The `C` generic is
    /// intended to represent any collection that can be iterated over, and the
    /// `L` generic is for any type that can be converted to the `LocationType`
    /// type.

    pub fn with_location_types<C, L>(
        &'a mut self,
        location_types: C
    ) -> &'a mut Self
    where
        C: IntoIterator<Item = L>,
        L: Into<LocationType> {
        // Add location types to ReverseRequest struct.
        self.location_types = location_types.into_iter().map(Into::into).collect();
        // Return modified ReverseRequest struct to caller.
        self
    } // fn
} // impl
