//! Directions API error types and error messages.

// -----------------------------------------------------------------------------

use crate::directions::response::status::Status;
use miette::Diagnostic;
use thiserror::Error;

// -----------------------------------------------------------------------------
//
/// Errors that may be produced by the Google Maps Directions API client.

#[derive(Debug, Diagnostic, Error)]
#[diagnostic(code(google_maps::directions), url(docsrs))]
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
    /// Google Maps Directions API service generated an error. See the `Status`
    /// enum for more information.
    GoogleMapsService(Status, Option<String>),
    /// The HTTP request was unsuccessful.
    HttpUnsuccessful(String),
    /// API client library attempted to parse a string that contained an invalid
    /// avoid/restrictions code. See
    /// `google_maps\src\directions\request\avoid.rs` for more information.
    InvalidAvoidCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// currency code. See
    /// `google_maps\src\directions\response\currency.rs` for more information.
    InvalidCurrencyCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// geocoder status code. See
    /// `google_maps\src\directions\response\geocoder_status.rs` for more
    /// information.
    InvalidGeocoderStatusCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// maneuver type code. See
    /// `google_maps\src\directions\response\maneuver_type.rs` for more
    /// information.
    InvalidDrivingManeuverCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// status code. See `google_maps\src\directions\response\status.rs` for
    /// more information.
    InvalidStatusCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// time zone name. See
    /// `google_maps\src\directions\response\time_zone.rs` for more information.
    InvalidTimeZoneName(String),
    /// API client library attempted to parse a string that contained an invalid
    /// traffic model code. See
    /// `google_maps\src\directions\request\traffic_model.rs` for more
    /// information.
    InvalidTrafficModelCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// transit mode code. See
    /// `google_maps\src\directions\request\transit_mode.rs` for more
    /// information.
    InvalidTransitModeCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// transit routing preference code. See
    /// `google_maps\src\directions\request\transit_route_preference.rs` for
    /// more information.
    InvalidTransitRoutePreferenceCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// travel mode code. See `google_maps\src\directions\travel_mode.rs` for
    /// more information.
    InvalidTravelModeCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// unit system code. See
    /// `google_maps\src\directions\request\unit_system.rs` for more
    /// information.
    InvalidUnitSystemCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// vehicle type code. See `google_maps\src\directions\vehicle_type.rs` for
    /// more information.
    InvalidVehicleTypeCode(String),
    /// API client library attempted to parse a string that contained an invalid
    /// departure time. See
    /// `google_maps\src\directions\request\departure_time.rs` for more
    /// information.
    InvalidDepartureTime(String),
    /// The query string must be built before the request may be sent to the
    /// Google Maps Directions API service.
    QueryNotBuilt,
    /// The request must be validated before a query string may be built.
    RequestNotValidated,
    /// The dependency library Reqwest generated an error.
    #[cfg(feature = "enable-reqwest")]
    Reqwest(reqwest::Error),
    /// The dependency library Reqwest generated an error. The error could
    /// not be passed normally so a `String` representation is passed instead.
    #[cfg(feature = "enable-reqwest")]
    ReqwestMessage(String),
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
                "Google Maps Directions API client: \
                The with_arrival_time() method may only be used when with_travel_mode() is set to `TravelMode::Transit`. \
                The travel mode is set to `{travel_mode}` and the arrival time is set to `{arrival_time}`. \
                Try again either with a travel mode of `TravelMode::Transit` or no arrival time."),
            Error::EitherAlternativesOrWaypoints(waypoint_count) => write!(f,
                "Google Maps Directions API client: \
                The with_alternatives() method cannot be set to `true` if with_waypoints() has been set. \
                {waypoint_count} waypoint(s) are set. \
                Try again either with no waypoints or no alternatives."),
            Error::EitherDepartureTimeOrArrivalTime(arrival_time, departure_time) => write!(f,
                "Google Maps Directions API client: \
                The with_departure_time() method cannot be used when with_arrival_time() has been set. \
                The arrival time is set to `{arrival_time}` and the departure time is set to `{departure_time}`. \
                Try again either with no arrival time or no departure time."),
            Error::EitherRestrictionsOrWaypoints(waypoint_count, restrictions) => write!(f,
                "Google Maps Directions API client: \
                The with_restrictions() method cannot be used when with_waypoints() has been set. \
                {waypoint_count} waypoint(s) are set and the restriction(s) are set to `{restrictions}`. \
                Try again either with no waypoints or no restrictions."),
            Error::EitherWaypointsOrTransitMode(waypoint_count) => write!(f,
                "Google Maps Directions API client: \
                The with_waypoints() method cannot be used when with_travel_mode() is set to `TravelMode::Transit`. \
                {waypoint_count} waypoint(s) are set. \
                Try again either with a different travel mode or no waypoints."),
            Error::GoogleMapsService(status, error_message) => match error_message {
                // If the Google Maps Directions API service generated an error
                // message, return that:
                Some(error_message) => write!(f, "Google Maps Directions API service: {error_message}"),
                // If the Google Maps Directions API service did not generate an
                // error message, return a generic message derived from the
                // response status:
                None => match status {
                    Status::InvalidRequest => write!(f, "Google Maps Directions API service: \
                        Invalid request. \
                        This may indicate that the query (address, components, or latlng) is missing, an invalid result type, or an invalid location type."),
                    Status::MaxRouteLengthExceeded => write!(f, "Google Maps Directions API service: \
                        Maximum route length exceeded. \
                        Try reducing the number of waypoints, turns, or instructions."),
                    Status::MaxWaypointsExceeded => write!(f, "Google Maps Directions API service: \
                        Maximum waypoints exceeded. \
                        The maximum allowed number of waypoints is 25, plus the origin and destination."),
                    Status::NotFound => write!(f, "Google Maps Directions API service: \
                        Not found. \
                        An origin, destination, or waypoint could not be geocoded."),
                    Status::Ok => write!(f, "Google Maps Directions server: \
                        Ok. \
                        The request was successful."),
                    Status::OverDailyLimit => write!(f, "Google Maps Directions API service: \
                        Over daily limit. \
                        Usage cap has been exceeded, API key is invalid, billing has not been enabled, or method of payment is no longer valid."),
                    Status::OverQueryLimit => write!(f, "Google Maps Directions API service: \
                        Over query limit. \
                        Requestor has exceeded quota."),
                    Status::RequestDenied => write!(f, "Google Maps Directions API service: \
                        Request denied. \
                        Service did not complete the request."),
                    Status::UnknownError => write!(f, "Google Maps Directions API service: \
                        Unknown error."),
                    Status::ZeroResults => write!(f, "Google Maps Directions API service: \
                        Zero results. \
                        This may occur if the geocoder was passed a non-existent address."),
                } // match
            }, // match
            Error::HttpUnsuccessful(status) => write!(f,
                "Google Maps Directions API client: \
                Could not successfully query the Google Cloud Platform service. \
                The service last responded with a `{status}` status."),
            Error::InvalidAvoidCode(avoid_code) => write!(f,
                "Google Maps Directions API client: \
                `{avoid_code}` is not a valid restrictions code. \
                Valid codes are `ferries`, `highways`, `indoor`, and `tolls`."),
             Error::InvalidCurrencyCode(currency_code) => write!(f,
                "Google Maps Directions API client: \
                `{currency_code}` is not a recognized currency code. \
                For a list of supported currencies see \
                https://en.wikipedia.org/wiki/ISO_4217"),
            Error::InvalidGeocoderStatusCode(geocoder_status_code) => write!(f,
                "Google Maps Directions API client: \
                `{geocoder_status_code}` is not a valid geocoder status code. \
                Valid codes are `OK`, and `ZERO_RESULTS`."),
            Error::InvalidDrivingManeuverCode(maneuver_type_code) => write!(f,
                "Google Maps Directions API client: \
                `{maneuver_type_code}` is not a valid maneuver type code. \
                Valid codes are `ferry`, `ferry-train`, `fork-left`, \
                `fork-right`, `keep-left`, `keep-right`, `merge`, `ramp-left`, \
                `ramp-right`, `roundabout-left`, `roundabout-right`, \
                `straight`, `turn-left`, `turn-right`, `turn-sharp-left`, \
                `turn-sharp-right`, `turn-slight-left`, `turn-slight-right`, \
                `uturn-left`, and `uturn-right`."),
            Error::InvalidStatusCode(status_code) => write!(f,
                "Google Maps Directions API client: \
                `{status_code}` is not a valid status code. \
                Valid codes are `INVALID_REQUEST`, `MAX_ROUTE_LENGTH_EXCEEDED` \
                `MAX_WAYPOINTS_EXCEEDED`, `NOT_FOUND`, `OK`, \
                `OVER_DAILY_LIMIT`, `OVER_QUERY_LIMIT`, `REQUEST_DENIED`, \
                `UNKNOWN_ERROR`, and `ZERO_RESULTS`."),
            Error::InvalidTimeZoneName(time_zone_name) => write!(f,
                "Google Maps Directions API client: \
                `{time_zone_name}` is not a recognized time zone name. \
                For a list of supported time zones see \
                https://www.iana.org/time-zones"),
            Error::InvalidTrafficModelCode(traffic_model_code) => write!(f,
                "Google Maps Directions API client: \
                `{traffic_model_code}` is not a valid traffic model code. \
                Valid codes are `best_guess`, `optimistic`, and `pessimistic`."),
            Error::InvalidTransitModeCode(transit_mode_code) => write!(f,
                "Google Maps Directions API client: \
                `{transit_mode_code}` is not a valid transit mode code. Valid codes are `bus`,
                `rail`, `subway`, `train`, and `tram`."),
            Error::InvalidTransitRoutePreferenceCode(transit_route_preference_code) =>
                write!(f, "Google Maps Directions API client: \
                `{transit_route_preference_code}` is not a valid transit route preference code. \
                Valid codes are `fewer_transfers` and `less_walking`."),
            Error::InvalidTravelModeCode(travel_mode_code) => write!(f,
                "Google Maps Directions API client: \
                `{travel_mode_code}` is not a valid travel mode code. \
                Valid codes are `bicycling`, `driving`, `transit`, and \
                `walking`."),
            Error::InvalidUnitSystemCode(unit_system_code) => write!(f,
                "Google Maps Directions API client: \
                `{unit_system_code}` is not a valid unit system code. \
                Valid codes are `imperial`, and `metric`."),
            Error::InvalidVehicleTypeCode(vehicle_type_code) => write!(f,
                "Google Maps Directions API client: \
                `{vehicle_type_code}` is not a valid vehicle type code. \
                Valid codes are `BUS`, `CABLE_CAR`, `COMMUTER_TRAIN`,  \
                `FERRY`, `FUNICULAR`, `GONDOLA_LIFT`, `HEAVY_RAIL`, \
                `HIGH_SPEED_TRAIN`, `INTERCITY_BUS`, `LONG_DISTANCE_TRAIN`, \
                `METRO_RAIL`, `MONORAIL`, `OTHER`, `RAIL`, `SHARE_TAXI`, \
                `SUBWAY`, `TRAM`, and `TROLLEYBUS`."),
            Error::QueryNotBuilt => write!(f,
                "Google Maps Directions API client: \
                The query string must be built before the request may be sent to the Google Cloud Maps Platform. \
                Ensure the build() method is called before run()."),
            Error::RequestNotValidated => write!(f,
                "Google Maps Directions API client: \
                The request must be validated before a query string may be built. \
                Ensure the validate() method is called before build()."),
            #[cfg(feature = "enable-reqwest")]
            Error::Reqwest(error) => write!(f, "Google Maps Directions API client in the Reqwest library: {error}"),
            #[cfg(feature = "enable-reqwest")]
            Error::ReqwestMessage(error) => write!(f, "Google Maps Geocoding API client in the Reqwest library: {error}"),
            Error::SerdeJson(error) => write!(f, "Google Maps Directions API client in the Serde JSON library: {error}"),
            Error::TooManyWaypoints(waypoint_count) => write!(f,
                "Google Maps Directions API client: \
                The maximum allowed number of waypoints is 25 plus the origin and destination. \
                {} waypoints are set. \
                Try again with {} fewer waypoint(s).",
                waypoint_count,
                waypoint_count - 25),
            Error::TransitModeIsForTransitOnly(travel_mode, transit_modes) => write!(f,
                "Google Maps Directions API client: \
                The with_transit_modes() method may only be used when with_travel_mode() is set to `TravelMode::Transit`. \
                The travel mode is set to `{travel_mode}` and the transit mode(s) are set to `{transit_modes}`. \
                Try again either with a travel mode of `TravelMode::Transit` or no transit modes."),
            Error::TransitRoutePreferenceIsForTransitOnly(travel_mode, transit_route_preference) => write!(f,
                "Google Maps Directions API client: \
                The with_transit_route_preference() method may only be used when with_travel_mode() is set to `TravelMode::Transit`. \
                The travel mode is set to `{travel_mode}` and the transit route preference is set to `{transit_route_preference}`. \
                Try again either with a travel mode of `TravelMode::Transit` or no transit route preference."),
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

#[cfg(feature = "enable-reqwest")]
impl From<reqwest::Error> for Error {
    /// This trait converts from an Reqwest error type (`reqwest::Error`) into a
    /// Google Maps Directions API error type
    /// (`google_maps::directions::error::Error`) by wrapping it inside. This
    /// function is required to use the `?` operator.
    fn from(error: reqwest::Error) -> Error {
        Error::Reqwest(error)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl From<serde_json::error::Error> for Error {
    /// This trait converts from an Serde JSON (`serde_json::error::Error`)
    /// error type into a Google Maps Directions API error type
    /// (`google_maps::directions::error::Error`) by wrapping it inside. This
    /// function is required to use the `?` operator.
    fn from(error: serde_json::error::Error) -> Error {
        Error::SerdeJson(error)
    } // fn
} // impl