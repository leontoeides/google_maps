use crate::directions::{error::Error, travel_mode::TravelMode};

// -----------------------------------------------------------------------------

impl crate::traits::Validatable for crate::directions::Request<'_> {
    /// Ensures that the request is valid.
    ///
    /// This function checks the combination of parameters to ensure that they
    /// make sense together and that Google Maps API will accept them.
    ///
    /// For example, Google Maps will not allow both a Positional Request and a
    /// Sampled Path Request in the same query.
    ///
    /// This function does not check parameter values for validity - i.e. it
    /// will not ensure Polylines or Latitudes/Longitudes are valid and
    /// well-formed.
    ///
    /// ## Arguments
    ///
    /// This method accepts no arguments.
    ///
    /// # Errors
    ///
    /// * This will fail if the request `struct` fails validation. For example,
    ///   parameters in the request conflict with one another, or the request
    ///   parameters are set in a way that's incompatible.
    ///
    ///   For example, Google Maps Directions API cannot calculate alternative
    ///   routes if waypoints have been set. This will cause a validation
    ///   failure.
    fn validate(&self) -> Result<(), crate::Error> {
        if let Some(travel_mode) = &self.travel_mode {
            // If the travel mode has been set to TravelMode::Transit...
            if *travel_mode == TravelMode::Transit {
                // ...waypoints cannot be set:
                if !self.waypoints.is_empty() {
                    return Err(Error::EitherWaypointsOrTransitMode(self.waypoints.len()))?;
                } // if

            // If the transit mode is not set to TravelMode::Transit...
            } else {
                // ...an arrival time cannot be set:
                if let Some(arrival_time) = &self.arrival_time {
                    return Err(Error::ArrivalTimeIsForTransitOnly(
                        travel_mode.to_string(),
                        arrival_time.format("%F %r").to_string(),
                    ))?; // Err
                } // if

                // ...a transit mode cannot be set:
                if !self.transit_modes.is_empty() {
                    return Err(Error::TransitModeIsForTransitOnly(
                        travel_mode.to_string(),
                        self.transit_modes
                            .iter()
                            .map(std::string::ToString::to_string)
                            .collect::<Vec<String>>()
                            .join("|"),
                    ))?; // Err
                } // if

                // ...a transit route preference cannot be set:
                if let Some(transit_route_preference) = &self.transit_route_preference {
                    return Err(Error::TransitRoutePreferenceIsForTransitOnly(
                        travel_mode.to_string(),
                        transit_route_preference.to_string(),
                    ))?; // Err
                } // if
            } // if
        } // if

        // If waypoints have been set...
        if !self.waypoints.is_empty() {
            // ...alternatives cannot be set to true:
            if let Some(alternatives) = &self.alternatives {
                if !alternatives {
                    return Err(Error::EitherAlternativesOrWaypoints(self.waypoints.len()))?;
                    // Err
                } // if
            } // if

            // ...restrictions cannot be set:
            if !self.restrictions.is_empty() {
                return Err(Error::EitherRestrictionsOrWaypoints(
                    self.waypoints.len(),
                    self.restrictions
                        .iter()
                        .map(std::string::ToString::to_string)
                        .collect::<Vec<String>>()
                        .join("|"),
                ))?; // Err
            } // if

            // ...ensure that the number of waypoints is equal to or less than 25:
            if self.waypoints.len() > 25 {
                return Err(Error::TooManyWaypoints(self.waypoints.len()))?;
            } // if
        } // if

        // If an arrival time has been set...
        if let Some(arrival_time) = &self.arrival_time {
            // ...a departure time cannot be set:
            if let Some(departure_time) = &self.departure_time {
                return Err(Error::EitherDepartureTimeOrArrivalTime(
                    arrival_time.format("%F %r").to_string(),
                    departure_time.to_string(),
                ))?; // Err
            } // if
        } // if

        // If we made it to the bottom, all tests have passed. Return `Ok` to
        // caller:
        Ok(())
    } // fn
} // impl