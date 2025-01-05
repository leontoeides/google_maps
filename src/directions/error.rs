//! Directions API error types and error messages.

// -----------------------------------------------------------------------------
//
/// An error produced by a Google Maps Directions API request.
#[derive(Clone, Debug, thiserror::Error, miette::Diagnostic)]
pub enum Error {
    // -------------------------------------------------------------------------
    // Client-side errors:
    // -------------------------------------------------------------------------

    // Validation errors:

    /// An arrival time may only be specified in Transit travel mode.
    #[error("`with_arrival_time` method may only be used when \
        `with_travel_mode` is set to `TravelMode::Transit`. The travel mode has
        been set to `{0}` and the arrival time has been set to `{1}`")]
    #[diagnostic(
        code(google_maps::directions::validate::arrival_time_is_for_transit_only),
        url(docsrs),
        help("try again either with a travel mode of `TravelMode::Transit` or \
            no arrival time")
    )]
    ArrivalTimeIsForTransitOnly(String, String),

    /// Alternatives may not be requested when waypoints are specified.
    #[error("`with_alternatives` method cannot be set to `true` if \
        `with_waypoints` has been set. {0} waypoints have been set")]
    #[diagnostic(
        code(google_maps::directions::validate::arrival_time_is_for_transit_only),
        url(docsrs),
        help("try again either with no waypoints or no alternatives")
    )]
    EitherAlternativesOrWaypoints(usize),

    /// Either departure time or arrival time may be specified, but not both.
    #[error("`with_departure_time` method cannot be used when \
        `with_arrival_time` has been set. The arrival time has been set to \
        `{0}` and the departure time has been set to `{1}`")]
    #[diagnostic(
        code(google_maps::directions::validate::either_departure_time_or_arrival_time),
        url(docsrs),
        help("try again either with no arrival time or no departure time")
    )]
    EitherDepartureTimeOrArrivalTime(String, String),

    /// Restrictions may not be specified when waypoints are specified.
    #[error("`with_restrictions` method cannot be used when `with_waypoints` \
        has been set. {0} waypoints have been set and the \
        restrictions are set to `{1}`")]
    #[diagnostic(
        code(google_maps::directions::validate::either_restrictions_or_waypoints),
        url(docsrs),
        help("try again either with no waypoints or no restrictions")
    )]
    EitherRestrictionsOrWaypoints(usize, String),

    /// Waypoints may not be specified in `Transit` travel mode.
    #[error("`with_waypoints` method cannot be used when `with_travel_mode` \
        is set to `TravelMode::Transit` but {0} waypoints have been set")]
    #[diagnostic(
        code(google_maps::directions::validate::either_waypoints_or_transit_mode),
        url(docsrs),
        help("try again either with a different travel mode or no waypoints")
    )]
    EitherWaypointsOrTransitMode(usize),

    /// Too many waypoints.
    ///
    /// Reduce the number of waypoints. The maximum allowed is 25 plus the
    /// origin and destination.
    #[error("too many waypoints: {0}")]
    #[diagnostic(
        code(google_maps::directions::validate::too_many_waypoints),
        url(docsrs),
        help("the maximum allowed is 25 plus the origin and destination")
    )]
    TooManyWaypoints(usize),

    /// Transit mode may only be specified in Transit travel mode.
    #[error("`with_transit_modes` method may only be used when \
        `with_travel_mode` is set to `TravelMode::Transit`. The travel mode \
        has been set to `{0}` and the transit modes have been set to `{1}`")]
    #[diagnostic(
        code(google_maps::directions::validate::transit_mode_is_for_transit_only),
        url(docsrs),
        help("try again either with a travel mode of `TravelMode::Transit` or no transit modes")
    )]
    TransitModeIsForTransitOnly(String, String),

    /// Transit routing preference may only be specified in Transit travel mode.
    #[error("`with_transit_route_preference` method may only be used when \
        `with_travel_mode` is set to `TravelMode::Transit`. The travel mode \
        has been set to `{0}` and the transit route preference has been set to \
        `{1}`")]
    #[diagnostic(
        code(google_maps::directions::validate::transit_mode_is_for_transit_only),
        url(docsrs),
        help("try again either with a travel mode of `TravelMode::Transit` or no transit route preference")
    )]
    TransitRoutePreferenceIsForTransitOnly(String, String),

    // Parse errors:

    /// Invalid avoid code.
    ///
    /// Valid codes are `ferries`, `highways`, `indoor`, and `tolls`.
    #[error("invalid avoid code: `{0}`")]
    #[diagnostic(
        code(google_maps::directions::parse::invalid_avoid_code),
        url("https://developers.google.com/maps/documentation/directions/get-directions#avoid"),
        help("valid codes are `ferries`, `highways`, `indoor`, and `tolls`")
    )]
    InvalidAvoidCode(String),

    /// Invalid currency.
    ///
    /// See ISO 4217 for supported currencies.
    #[error("invalid currency code: `{0}`")]
    #[diagnostic(
        code(google_maps::directions::parse::invalid_currency_code),
        url("https://en.wikipedia.org/wiki/ISO_4217"),
        help("see ISO 4217 for supported currencies")
    )]
    InvalidCurrencyCode(String),

    /// Invalid driving maneuver.
    ///
    /// Valid codes are `ferry`, `ferry-train`, `fork-left`, `fork-right`, \
    /// `keep-left`, `keep-right`, `merge`, `ramp-left`, `ramp-right`, \
    /// `roundabout-left`, `roundabout-right`, `straight`, `turn-left`, \
    /// `turn-right`, `turn-sharp-left`, `turn-sharp-right`, \
    /// `turn-slight-left`, `turn-slight-right`, `uturn-left`, and
    /// `uturn-right`.
    #[error("invalid driving maneuver: `{0}`")]
    #[diagnostic(
        code(google_maps::directions::parse::invalid_driving_maneuver_code),
        url("https://developers.google.com/maps/documentation/directions/get-directions#DirectionsStep-maneuver"),
        help("valid codes are `ferry`, `ferry-train`, `fork-left`, \
            `fork-right`, `keep-left`, `keep-right`, `merge`, `ramp-left`, \
            `ramp-right`, `roundabout-left`, `roundabout-right`, `straight`, \
            `turn-left`, `turn-right`, `turn-sharp-left`, `turn-sharp-right`, \
            `turn-slight-left`, `turn-slight-right`, `uturn-left`, and \
            `uturn-right`")
    )]
    InvalidDrivingManeuverCode(String),

    /// Invalid geocoder status.
    ///
    /// Valid codes are `OK` and `ZERO_RESULTS`.
    #[error("invalid geocoder status: `{0}`")]
    #[diagnostic(
        code(google_maps::directions::parse::invalid_geocoder_status_code),
        url("https://developers.google.com/maps/documentation/directions/get-directions#DirectionsGeocodedWaypoint-geocoder_status"),
        help("valid codes are `OK` and `ZERO_RESULTS`")
    )]
    InvalidGeocoderStatusCode(String),

    /// Invalid time zone.
    ///
    /// Ensure the provided name is valid according to the IANA Time Zone
    /// Database.
    #[error("invalid time zone: `{0}`")]
    #[diagnostic(
        code(google_maps::directions::parse::invalid_time_zone_name),
        url("https://www.iana.org/time-zones"),
        help("ensure the provided name is valid according to the IANA Time \
            Zone Database")
    )]
    InvalidTimeZoneName(String),

    /// Invalid traffic model.
    ///
    /// Valid codes are `best_guess`, `optimistic`, and `pessimistic`.
    #[error("invalid traffic model: `{0}`")]
    #[diagnostic(
        code(google_maps::directions::parse::invalid_traffic_model_code),
        url("https://developers.google.com/maps/documentation/directions/get-directions#traffic_model"),
        help("valid codes are `best_guess`, `optimistic`, and `pessimistic`")
    )]
    InvalidTrafficModelCode(String),

    /// Invalid transit mode.
    ///
    /// Valid codes are `bus`, `rail`, `subway`, `train`, and `tram`.
    #[error("invalid transit mode: `{0}`")]
    #[diagnostic(
        code(google_maps::directions::parse::invalid_traffic_model_code),
        url("https://developers.google.com/maps/documentation/directions/get-directions#transit_mode"),
        help("valid codes are `bus`, `rail`, `subway`, `train`, and `tram`")
    )]
    InvalidTransitModeCode(String),

    /// Invalid transit route preference.
    ///
    /// Valid codes are `fewer_transfers` and `less_walking`.
    #[error("invalid transit route preference: `{0}`")]
    #[diagnostic(
        code(google_maps::directions::parse::invalid_transit_route_preference_code),
        url("https://developers.google.com/maps/documentation/directions/get-directions#transit_routing_preference"),
        help("valid codes are `fewer_transfers` and `less_walking`")
    )]
    InvalidTransitRoutePreferenceCode(String),

    /// Invalid travel mode.
    ///
    /// Valid codes are `bicycling`, `driving`, `transit`, and `walking`.
    #[error("invalid travel mode: `{0}`")]
    #[diagnostic(
        code(google_maps::directions::parse::invalid_travel_mode_code),
        url("https://developers.google.com/maps/documentation/directions/get-directions#TravelModes"),
        help("valid codes are `bicycling`, `driving`, `transit`, and `walking`")
    )]
    InvalidTravelModeCode(String),

    /// Invalid unit system.
    ///
    /// Valid codes are `imperial`, and `metric`.
    #[error("invalid travel mode: `{0}`")]
    #[diagnostic(
        code(google_maps::directions::parse::invalid_unit_system_code),
        url("https://developers.google.com/maps/documentation/directions/get-directions#units"),
        help("valid codes are `imperial`, and `metric`")
    )]
    InvalidUnitSystemCode(String),

    /// Invalid vehicle type.
    ///
    /// Valid codes are `BUS`, `CABLE_CAR`, `COMMUTER_TRAIN`, `FERRY`,
    /// `FUNICULAR`, `GONDOLA_LIFT`, `HEAVY_RAIL`, `HIGH_SPEED_TRAIN`,
    /// `INTERCITY_BUS`, `LONG_DISTANCE_TRAIN`, `METRO_RAIL`, `MONORAIL`,
    /// `OTHER`, `RAIL`, `SHARE_TAXI`, `SUBWAY`, `TRAM`, and `TROLLEYBUS`.
    #[error("invalid vehicle type: `{0}`")]
    #[diagnostic(
        code(google_maps::directions::parse::invalid_vehicle_type_code),
        url("https://developers.google.com/maps/documentation/directions/get-directions#DirectionsTransitVehicle-type"),
        help("valid codes are `BUS`, `CABLE_CAR`, `COMMUTER_TRAIN`, `FERRY`, \
            `FUNICULAR`, `GONDOLA_LIFT`, `HEAVY_RAIL`, `HIGH_SPEED_TRAIN`, \
            `INTERCITY_BUS`, `LONG_DISTANCE_TRAIN`, `METRO_RAIL`, `MONORAIL`, \
            `OTHER`, `RAIL`, `SHARE_TAXI`, `SUBWAY`, `TRAM`, and `TROLLEYBUS`")
    )]
    InvalidVehicleTypeCode(String),

    /// Invalid departure time.
    ///
    /// Valid departure times are `now`, or a UNIX timestamp.
    #[error("invalid departure time: `{0}`")]
    #[diagnostic(
        code(google_maps::directions::parse::invalid_departure_time),
        url("https://developers.google.com/maps/documentation/directions/get-directions#departure_time"),
        help("valid departure times are `now`, or a UNIX timestamp")
    )]
    InvalidDepartureTime(String),

    /// Invalid status code.
    ///
    /// Valid codes are `INVALID_REQUEST`, `MAX_ROUTE_LENGTH_EXCEEDED`,
    /// `MAX_WAYPOINTS_EXCEEDED`, `NOT_FOUND`, `OK`, `OVER_DAILY_LIMIT`,
    /// `OVER_QUERY_LIMIT`, `REQUEST_DENIED`, `UNKNOWN_ERROR`, and
    /// `ZERO_RESULTS`.
    #[error("invalid status: `{0}`")]
    #[diagnostic(
        code(google_maps::directions::parse::invalid_status_code),
        url("https://developers.google.com/maps/documentation/directions/get-directions#DirectionsStatus"),
        help("valid codes are `INVALID_REQUEST`, `MAX_ROUTE_LENGTH_EXCEEDED` \
            `MAX_WAYPOINTS_EXCEEDED`, `NOT_FOUND`, `OK`, `OVER_DAILY_LIMIT`, \
            `OVER_QUERY_LIMIT`, `REQUEST_DENIED`, `UNKNOWN_ERROR`, and \
            `ZERO_RESULTS`")
    )]
    InvalidStatusCode(String),

    // -------------------------------------------------------------------------
    // Server-side errors (statuses):
    // -------------------------------------------------------------------------

    /// Not found.
    ///
    /// An origin, destination, or waypoint could not be geocoded.
    #[error("not found")]
    #[diagnostic(
        code(google_maps::directions::status::not_found),
        url("https://developers.google.com/maps/documentation/directions/get-directions#DirectionsStatus"),
        help("indicates at least one of the locations specified in the \
            request's origin, destination, or waypoints could not be geocoded")
    )]
    NotFound,

    /// Zero results.
    ///
    /// No route could be found with the given parameters.
    #[error("zero results")]
    #[diagnostic(
        code(google_maps::directions::status::zero_results),
        url("https://developers.google.com/maps/documentation/directions/get-directions#DirectionsStatus"),
        help("indicates no route could be found between the origin and destination")
    )]
    ZeroResults,

    /// Maximum waypoints exceeded.
    ///
    /// The maximum allowed number of waypoints is 25, plus the origin and
    /// destination.
    #[error("maximum waypoints exceeded")]
    #[diagnostic(
        code(google_maps::directions::status::max_waypoints_exceeded),
        url("https://developers.google.com/maps/documentation/directions/get-directions#DirectionsStatus"),
        help("indicates that too many waypoints were provided in the request. \
            For applications using the Directions API as a web service, or the \
            directions service in the Maps JavaScript API, the maximum allowed \
            number of waypoints is 25, plus the origin and destination")
    )]
    MaxWaypointsExceeded,

    /// Maximum route length exceeded.
    ///
    /// Try reducing the number of waypoints, turns, or instructions.
    #[error("maximum route length exceeded")]
    #[diagnostic(
        code(google_maps::directions::status::max_route_length_exceeded),
        url("https://developers.google.com/maps/documentation/directions/get-directions#DirectionsStatus"),
        help("indicates the requested route is too long and cannot be \
            processed. This error occurs when more complex directions are \
            returned. Try reducing the number of waypoints, turns, or \
            instructions")
    )]
    MaxRouteLengthExceeded,

    /// Invalid request.
    ///
    /// This may indicate that the query (`address`, `components`, or `latlng`)
    /// is missing, an invalid result type, or an invalid location type.
    #[error("invalid request")]
    #[diagnostic(
        code(google_maps::directions::status::invalid_request),
        url("https://developers.google.com/maps/documentation/directions/get-directions#DirectionsStatus"),
        help("indicates that the provided request was invalid. Common causes \
            of this status include an invalid parameter or parameter value")
    )]
    InvalidRequest,

    /// Over daily limit indicates that the request was denied for one or more
    /// of the following reasons:
    ///
    /// * The API key is missing or invalid.
    ///
    /// * Billing has not been enabled on your account.
    ///
    /// * A self-imposed usage cap has been exceeded.
    ///
    /// * The provided method of payment is no longer valid (for example, a
    ///   credit card has expired).
    ///
    /// In order to use Google Maps Platform products, billing must be enabled
    /// on your account, and all requests must include a valid API key. To fix
    /// this, take the following steps:
    ///
    /// * [Get an API key](https://developers.google.com/maps/documentation/roads/errors?hl=en#new-key)
    ///
    /// * [Enable billing](https://console.cloud.google.com/project/_/billing/enable)
    ///   on your account.
    ///
    /// * [Adjust your usage cap](https://developers.google.com/maps/documentation/roads/errors?hl=en#usage-cap)
    ///   to increase your daily limit (if applicable).
    #[error("over daily limit")]
    #[diagnostic(
        code(google_maps::directions::status::over_daily_limit),
        url("https://developers.google.com/maps/faq#over-limit-key-error"),
        help("either the API key is missing or invalid, billing has not been \
            enabled on your account, a self-imposed usage cap has been \
            exceeded, or the provided method of payment is no longer valid \
            (for example, a credit card has expired)")
    )]
    OverDailyLimit,

    /// Overy query limit indicates any of the following:
    ///
    /// * You have exceeded the QPS limits.
    ///
    /// * Billing has not been enabled on your account.
    ///
    /// * The monthly $200 credit, or a self-imposed usage cap, has been
    ///   exceeded.
    ///
    /// * The provided method of payment is no longer valid (for example, a
    ///   credit card has expired).
    ///
    /// See the [Maps FAQ](https://developers.google.com/maps/faq#over-limit-key-error)
    /// for more information about how to resolve this error.
    #[error("over query limit")]
    #[diagnostic(
        code(google_maps::directions::status::over_query_limit),
        url("https://developers.google.com/maps/faq#over-limit-key-error"),
        help("either you have exceeded the QPS limits, billing has not been \
            enabled on your account, a self-imposed usage cap has been \
            exceeded or the provided method of payment is no longer valid")
    )]
    OverQueryLimit,

    /// Request denied by the server.
    #[error("request denied")]
    #[diagnostic(
        code(google_maps::directions::status::request_denied),
        url("https://developers.google.com/maps/documentation/directions/get-directions#DirectionsStatus"),
        help("indicates that the service denied use of the directions service \
            by your application")
    )]
    RequestDenied,

    /// Unknown error from the server.
    #[error("unknown error")]
    #[diagnostic(
        code(google_maps::directions::status::unknown_error),
        url("https://developers.google.com/maps/documentation/directions/get-directions#DirectionsStatus"),
        help("indicates a directions request could not be processed due to a \
            server error. The request may succeed if you try again")
    )]
    UnknownError,
} // enum Error

// -----------------------------------------------------------------------------

use crate::ClassifiedError;

impl crate::traits::ClassifiableError<'_, Self> for Error {
    /// Classifies an API error as a `Transient` error or `Permanent` error.
    ///
    /// This classification will, in turn, be used to decide whether the HTTP
    /// request should be retried or not.
    fn classify(&self) -> ClassifiedError<'_, Self> {
        match self {
            Self::UnknownError => ClassifiedError::Transient(self),
            _ => ClassifiedError::Permanent(self),
        } // match
    } // fn
} // impl