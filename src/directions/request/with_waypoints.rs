use crate::directions::request::{waypoint::Waypoint, Request};

impl<'a> Request<'a> {
    /// Specify pass throughs or stopovers at intermediate locations.
    ///
    /// ## Arguments
    ///
    /// * `waypoints` ‧ Specifies intermediate locations to visit before
    ///   arriving at the final destination.
    ///
    /// ## Description
    ///
    /// Specifies an array of intermediate locations to include along the route
    /// between the origin and destination points as pass through or stopover
    /// locations. Waypoints alter a route by directing it through the specified
    /// location(s). The API supports waypoints for these travel modes: driving,
    /// walking and bicycling; not transit. You can specify waypoints using the
    /// following values:
    ///
    /// * `Waypoint::Address`: An explicit value pair.
    ///
    /// * `Waypoint::PlaceId`: The unique value specific to a location. This
    ///   value is only available only if the request includes an API key or
    ///   Google Maps Platform Premium Plan client ID
    ///   (`ChIJGwVKWe5w44kRcr4b9E25`).
    ///
    /// * `Waypoint::Address`: Address string (`Charlestown, Boston,MA`)
    ///
    /// * `Waypoint::Polyline`: Encoded polyline that can be specified by a set
    ///   of any of the above. (`lexeF{~wsZejrPjtye@:`)
    ///
    /// [Waypoints](https://developers.google.com/maps/documentation/directions/intro#Waypoints)
    /// ----------------------------------------------------------------------------------------
    ///
    /// *Caution*: Requests using 10 or more waypoints, or waypoint
    /// optimization, are billed at a higher rate.
    /// [Learn more about billing for Google Maps Platform products](https://developers.google.com/maps/billing/understanding-cost-of-use).
    ///
    /// When calculating routes using the Directions API, you may specify
    /// waypoints to return a route that includes pass throughs or stopovers at
    /// intermediate locations. You can add waypoints to driving, walking or
    /// bicycling directions but not transit directions.
    ///
    /// [Specify locations in the waypoints parameter.](https://developers.google.com/maps/documentation/directions/intro#specify-locations-in-the-waypoints-parameter.)
    ///
    /// You can supply one or more locations in the form of a place ID, an
    /// address, or latitude/longitude coordinates. By default, the Directions
    /// service calculates a route using the waypoints in the order they are
    /// given. The precedence for parsing the value of the waypoint is place ID,
    /// latitude/longitude coordinates, then address.
    ///
    /// * `Waypoint::PlaceId` If you pass a place ID, you must provide an API
    ///   key or a Google Maps Platform Premium Plan client ID. You can retrieve
    ///   place IDs from the Geocoding API and the Places API (including Place
    ///   Autocomplete). For an example using place IDs from Place Autocomplete,
    ///   see
    ///   [Place Autocomplete and Directions](https://developers.google.com/maps/documentation/javascript/examples/places-autocomplete-directions).
    ///   For more about place IDs, see the
    ///   [Place ID overview](https://developers.google.com/places/place-id).
    ///
    /// * For efficiency and accuracy, use place ID's when possible. These ID's
    ///   are uniquely explicit like a lat/lng value pair and provide geocoding
    ///   benefits for routing such as access points and traffic variables.
    ///   Unlike an address, ID's do not require the service to perform a search
    ///   or an intermediate request for place details; therefore, performance
    ///   is better.
    ///
    /// * `Waypoint::LatLng` If you pass latitude/longitude coordinates, the
    ///   values go directly to the front-end server to calculate directions
    ///   without geocoding. The points are snapped to roads and might not
    ///   provide the accuracy your app needs. Use coordinates when you are
    ///   confident the values truly specify the points your app needs for
    ///   routing without regard to possible access points or additional
    ///   geocoding details.
    ///
    /// * `Waypoint::Address` If you pass an address, the Directions service
    ///   will geocode the string and convert it into latitude/longitude
    ///   coordinates to calculate directions. If the address value is ambiguous,
    ///   the value might evoke a search to disambiguate from similar addresses.
    ///   For example, "1st Street" could be a complete value or a partial value
    ///   for "1st street NE" or "1st St SE". This result may be different from
    ///   that returned by the Geocoding API. You can avoid possible
    ///   misinterpretations using place IDs. See
    ///   [troubleshooting the results of my route request](https://developers.google.com/maps/documentation/directions/intro).
    ///
    /// ## Example
    ///
    /// * After departing from the `origin` location, stop for groceries at
    ///   Sobeys before finally going to the `destination` location:
    ///
    /// ```rust
    /// // Orléans Sobeys, 2276 Tenth Line Rd, Orléans, ON K4A 0X4
    /// .with_waypoint(Waypoint::PlaceId(String::from("ChIJi5fWgmcSzkwRePJ_I9-xCRg")))
    /// ```

    pub fn with_waypoint(
        &'a mut self,
        waypoint: impl Into<Waypoint>
    ) -> &'a mut Self {
        let waypoint: Waypoint = waypoint.into();
        // Add waypoint to Request struct.
        self.waypoints = vec![waypoint];
        // Return modified Request struct to caller.
        self
    } // fn

    /// ## Example
    ///
    /// * After departing from the `origin` location; visit the Canadian Museum
    ///   of Nature, Rideau Canal National Historic Site, intersection of Bank
    ///   St & Queen Elizabeth Driveway, and then Patterson's Creek Park before
    ///   finally going to the `destination` location:
    ///
    /// ```rust
    /// .with_waypoints(&[
    ///     // Canadian Museum of Nature
    ///     Waypoint::Address(String::from("240 McLeod St, Ottawa, ON K2P 2R1")),
    ///     // Rideau Canal National Historic Site
    ///     Waypoint::LatLng(LatLng::try_from_dec(dec!(45.40453), dec!(-75.6821073))?),
    ///     // Polyline to Bank St & Queen Elizabeth Driveway
    ///     Waypoint::Polyline(String::from("}`ctGdm|lMfBdEfRsLdSbHfExT")),
    ///     // Patterson's Creek Park
    ///     Waypoint::PlaceId(String::from("ChIJyeH59bkFzkwRnPg4zYevwQk")),
    /// ])
    /// ```
    ///
    /// # Generics
    ///
    /// This method uses generics to improve ergonomics. The `C` generic is
    /// intended to represent any collection that can be iterated over, and the
    /// `W` generic is for any type that can be converted to a `Waypoint` type.

    pub fn with_waypoints<C, W>(
        &'a mut self,
        waypoints: C
    ) -> &'a mut Self
    where
        C: IntoIterator<Item = W>,
        W: Into<Waypoint> {
        // Add waypoints to Request struct.
        self.waypoints = waypoints.into_iter().map(Into::into).collect();
        // Return modified Request struct to caller.
        self
    } // fn
} // impl
