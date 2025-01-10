use crate::elevation::Locations;

// -----------------------------------------------------------------------------

impl crate::elevation::Request<'_> {
    /// Adds the _sampled path request_ parameters to the Elevation API query.
    ///
    /// ## Arguments
    ///
    /// * `path` ‧ Defines a path on the earth for which to return elevation
    ///   data. This parameter defines a set of two or more ordered
    ///   latitude/longitude pairs defining a path along the surface of the
    ///   earth. For more information, see [Specifying
    ///   Paths](https://developers.google.com/maps/documentation/elevation/intro#Paths).
    ///
    /// * `samples` ‧ Specifies the number of sample points along a path for
    ///   which to return elevation data. The samples parameter divides the
    ///   given path into an ordered set of equidistant points along the path.
    ///
    /// ## Examples:
    ///
    /// * 2 elevation samples between two points:
    /// ```rust
    /// .for_sampled_path_request(
    ///     Locations::LatLngs(vec![
    ///         // Denver, Colorado
    ///         LatLng::try_from_dec(dec!(40.714728), dec!(-73.998672))?,
    ///         // Death Valley, California
    ///         LatLng::try_from_dec(dec!(-34.397), dec!(-116.866667))?,
    ///     ]),
    ///     // Number of samples
    ///     2
    /// )
    /// ```
    ///
    /// * 4 elevation samples along a polyline:
    /// ```rust
    /// .for_sampled_path_request(
    ///     Locations::Polyline(String::from("gfo}EtohhUxD@bAxJmGF")),
    ///     // Number of samples
    ///     4
    /// )
    /// ```
    #[must_use] pub fn for_sampled_path_request(
        mut self,
        path: impl Into<Locations>,
        samples: impl Into<u8>
    ) -> Self {
        let path: Locations = path.into();
        let samples: u8 = samples.into();
        // Set the path in Request struct.
        self.path = Some(path);
        // Set the sample number in Request struct.
        self.samples = Some(samples);
        // Return modified Request struct to caller.
        self
    } // fn

    /// Adds the _positional request_ parameter to the Elevation API query.
    ///
    /// This function is the same as `for_sampled_path_request` but it supports
    /// the [geo](https://crates.io/crates/geo) crate's
    /// [LineString](https://docs.rs/geo/latest/geo/geometry/struct.LineString.html)
    /// type.
    ///
    /// ## Arguments
    ///
    /// * `line_string` ‧ Specifies the sample points along a path for which to
    ///   return elevation data. The samples parameter divides the given path
    ///   into an ordered set of equidistant points along the path.
    #[cfg(feature = "geo")]
    #[deprecated(since = "3.5.1", note =
        "you may now use geo types directly with the google_maps crate. \
        the geo-specific methods are no longer necessary. \
        it's suggested to use the `for_sampled_path_request` method instead"
    )]
    #[must_use] pub fn for_line_string_request(
        mut self,
        line_string: geo_types::LineString
    ) -> Result<Self, crate::error::Error> {
        // Set the path in Request struct.
        self.locations = Some(Locations::LineString(line_string));
        // Return modified Request struct to caller.
        Ok(self)
    } // fn
} // impl