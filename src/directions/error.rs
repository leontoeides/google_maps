//! Resources (enums, traits) for the client to handle Directions API errors.

use crate::directions::response::status::Status;

/// Errors that may be produced by the Google Maps Directions API client.
#[derive(Debug)]
pub enum Error {
    /// An arrival time may only be specified in Transit travel mode.
    ArrivalTimeIsForTransitOnly(String, String),
    /// Alternatives may not be requested when waypoints are specified.
    EitherAlternativesOrWaypoints(usize),
    /// An departure time may not be specified when an arrival time is
    /// specified.
    EitherDepartureTimeOrArrivalTime(String, String),
    /// Restrictions may not be specified when waypoints are specified.
    EitherRestrictionsOrWaypoints(usize, String),
    /// Waypoints may not be specified in Transit travel mode.
    EitherWaypointsOrTransitMode(usize),
    /// Google Maps Directions API server generated an error. See the `Status`
    /// enum for more information.
    GoogleMapsDirectionsServer(Status, Option<String>),
    /// The dependency library Ihsahc generated an error.
    // Isahc(isahc::Error),
    /// The query string must be built before the request may be sent to the
    /// Google Maps Directions API server.
    QueryNotBuilt,
    /// The request must be validated before a query string may be built.
    RequestNotValidated,
    /// The dependency library Reqwest generated an error.
    Reqwest(reqwest::Error),
    /// The dependency library Serde JSON generated an error.
    SerdeJson(serde_json::error::Error),
    /// Too many waypoints specified.
    TooManyWaypoints(usize),
    /// Transit mode may only be specified in Transit travel mode.
    TransitModeIsForTransitOnly(String, String),
    /// Transit routing preference may only be specified in Transit travel mode.
    TransitRoutePreferenceIsForTransitOnly(String, String),
} // enum

impl std::fmt::Display for Error {
    /// This trait converts the error code into a format that may be presented
    /// to the user.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ArrivalTimeIsForTransitOnly(travel_mode, arrival_time) => write!(f,
                "Google Directions API client library: \
                The with_arrival_time() method may only be used when with_travel_mode() is set to `TravelMode::Transit`. \
                The travel mode is set to `{}` and the arrival time is set to `{}`. \
                Try again either with a travel mode of `TravelMode::Transit` or no arrival time.",
                travel_mode,
                arrival_time),
            Error::EitherAlternativesOrWaypoints(waypoint_count) => write!(f,
                "Google Directions API client library: \
                The with_alternatives() method cannot be set to `true` if with_waypoints() has been set. \
                {} waypoint(s) are set. \
                Try again either with no waypoints or no alternatives.",
                waypoint_count),
            Error::EitherDepartureTimeOrArrivalTime(arrival_time, departure_time) => write!(f,
                "Google Directions API client library: \
                The with_departure_time() method cannot be used when with_arrival_time() has been set. \
                The arrival time is set to `{}` and the departure time is set to `{}`. \
                Try again either with no arrival time or no departure time.",
                arrival_time,
                departure_time),
            Error::EitherRestrictionsOrWaypoints(waypoint_count, restrictions) => write!(f,
                "Google Directions API client library: \
                The with_restrictions() method cannot be used when with_waypoints() has been set. \
                {} waypoint(s) are set and the restrictions(s) are set to `{}`. \
                Try again either with no waypoints or no restrictions.",
                waypoint_count,
                restrictions),
            Error::EitherWaypointsOrTransitMode(waypoint_count) => write!(f,
                "Google Directions API client library: \
                The with_waypoints() method cannot be used when with_travel_mode() is set to `TravelMode::Transit`. \
                {} waypoint(s) are set. \
                Try again either with a different travel mode or no waypoints.",
                waypoint_count),
            Error::GoogleMapsDirectionsServer(status, error_message) => match error_message {
                // If the Google Maps Directions API server generated an error
                // message, return that:
                Some(error_message) => write!(f, "Google Maps Directions API server: {}", error_message),
                // If the Google Maps Directions API server did not generate an
                // error message, return a generic message derived from the
                // response status:
                None => match status {
                    Status::InvalidRequest => write!(f, "Google Maps Directions API server: \
                        Invalid request. \
                        This may indicate that the query (address, components, or latlng) is missing, an invalid result type, or an invalid location type."),
                    Status::MaxRouteLengthExceeded => write!(f, "Google Maps Directions API server:"),
                    Status::MaxWaypointsExceeded => write!(f, "Google Maps Directions API server:"),
                    Status::NotFound => write!(f, "Google Maps Directions API server:"),
                    Status::Ok => write!(f, "Google Maps Directions server: \
                        Ok. \
                        The request was successful."),
                    Status::OverDailyLimit => write!(f, "Google Maps Directions API server: \
                        Over daily limit. \
                        Usage cap has been exceeded, API key is invalid, billing has not been enabled, or method of payment is no longer valid."),
                    Status::OverQueryLimit => write!(f, "Google Maps Directions API server: \
                        Over query limit. \
                        Requestor has exceeded quota."),
                    Status::RequestDenied => write!(f, "Google Maps Directions API server: \
                        Request denied \
                        Service did not complete the request."),
                    Status::UnknownError => write!(f, "Google Maps Directions API server: \
                        Unknown error."),
                    Status::ZeroResults => write!(f, "Google Maps Directions API server: \
                        Zero results.
                        This may occur if the geocoder was passed a non-existent address."),
                } // match
            }, // match
            // Error::Isahc(error) => write!(f, "Google Maps Directions API client in the Isahc library: {}", error),
            Error::QueryNotBuilt => write!(f, "Google Maps Directions API client library: \
                The query string must be built before the request may be sent to the Google Cloud Maps Platform. \
                Ensure the build() method is called before run()."),
            Error::RequestNotValidated => write!(f, "Google Maps Directions API client library: \
                The request must be validated before a query string may be built. \
                Ensure the validate() method is called before build()."),
            Error::Reqwest(error) => write!(f, "Google Maps Directions API client in the Reqwest library: {}", error),
            Error::SerdeJson(error) => write!(f, "Google Maps Directions API client in the Serde JSON library: {}", error),
            Error::TooManyWaypoints(waypoint_count) => write!(f,
                "Google Directions API client library: \
                The maximum allowed number of waypoints is 25 plus the origin and destination. \
                {} waypoints are set. \
                Try again with {} fewer waypoint(s).",
                waypoint_count,
                waypoint_count - 25),
            Error::TransitModeIsForTransitOnly(travel_mode, transit_modes) => write!(f,
                "Google Directions API client library: \
                The with_transit_modes() method may only be used when with_travel_mode() is set to `TravelMode::Transit`. \
                The travel mode is set to `{}` and the transit mode(s) are set to `{}`. \
                Try again either with a travel mode of `TravelMode::Transit` or no transit modes.",
                travel_mode,
                transit_modes),
            Error::TransitRoutePreferenceIsForTransitOnly(travel_mode, transit_route_preference) => write!(f,
                "Google Directions API client library: \
                The with_transit_route_preference() method may only be used when with_travel_mode() is set to `TravelMode::Transit`. \
                The travel mode is set to `{}` and the transit route preference is set to `{}`. \
                Try again either with a travel mode of `TravelMode::Transit` or no transit route preference.",
                travel_mode,
                transit_route_preference),
        } // match
    } // fn
} // impl

impl std::error::Error for Error {
    /// If the cause for the error is in an underlying library (not this
    /// library but a library this one depends on), this trait unwraps the
    /// original source error. This trait converts a Google Maps Directions API
    /// error type into the native error type of the underlying library.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ArrivalTimeIsForTransitOnly(_travel_mode, _arrival_time) => None,
            Error::EitherAlternativesOrWaypoints(_waypoint_count) => None,
            Error::EitherDepartureTimeOrArrivalTime(_arrival_time, _departure_time) => None,
            Error::EitherRestrictionsOrWaypoints(_waypoint_count, _restrictions) => None,
            Error::EitherWaypointsOrTransitMode(_waypoint_count) => None,
            Error::GoogleMapsDirectionsServer(_error, _message) => None,
            // Error::Isahc(error) => Some(error),
            Error::QueryNotBuilt => None,
            Error::RequestNotValidated => None,
            Error::Reqwest(error) => Some(error),
            Error::SerdeJson(error) => Some(error),
            Error::TooManyWaypoints(_waypoint_count) => None,
            Error::TransitModeIsForTransitOnly(_travel_mode, _transit_modes) => None,
            Error::TransitRoutePreferenceIsForTransitOnly(_travel_mode, _transit_route_preference) => None,
        } // match
    } // fn
} // impl

/* impl From<isahc::Error> for Error {
    /// This trait converts from an Isahc error type (`isahc::Error`) into a
    /// Google Maps Directions API error type
    /// (`google_maps::time_zone::error::Error`) by wrapping it inside. This
    /// function is required to use the `?` operator.
    fn from(error: isahc::Error) -> Error {
        Error::Isahc(error)
    } // fn
} // impl */

impl From<reqwest::Error> for Error {
    /// This trait converts from an Isahc error type (`reqwest::Error`) into a
    /// Google Maps Directions API error type
    /// (`google_maps::time_zone::error::Error`) by wrapping it inside. This
    /// function is required to use the `?` operator.
    fn from(error: reqwest::Error) -> Error {
        Error::Reqwest(error)
    } // fn
} // impl

impl From<serde_json::error::Error> for Error {
    /// This trait converts from an Serde JSON (`serde_json::error::Error`)
    /// error type into a Google Maps Directions API error type
    /// (`google_maps::time_zone::error::Error`) by wrapping it inside. This
    /// function is required to use the `?` operator.
    fn from(error: serde_json::error::Error) -> Error {
        Error::SerdeJson(error)
    } // fn
} // impl