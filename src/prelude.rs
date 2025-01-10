//! Put `use google_maps::prelude::*;` in your code will to get more convenient
//! access to everything you need. If you're not concerned with name space
//! collisions or conflicts, you can glob import all `google_maps` structs and
//! enums by using this module.

// Re-export dependencies

#[cfg(any(
    feature = "directions",
    feature = "distance_matrix",
    feature = "time_zone"
))]
pub use chrono::{offset::TimeZone, DateTime, Duration, Local, NaiveDate, NaiveDateTime, Utc};

#[cfg(any(feature = "directions", feature = "time_zone"))]
pub use chrono_tz::Tz;

pub use rust_decimal::Decimal;
pub use rust_decimal_macros::dec;

// =============================================================================
//
// Common structures:

pub use crate::{
    client::Client,
    error::Error,
    types::error::Error as TypeError,
};

#[deprecated(note = "use `google_maps::Client` instead", since = "3.8.0")]
pub use crate::client::Client as ClientSettings;

#[deprecated(note = "use `google_maps::Client` instead", since = "3.8.0")]
pub use crate::client::Client as GoogleMapsClient;

#[deprecated(note = "use `google_maps::Error` instead", since = "3.8.0")]
pub use crate::error::Error as GoogleMapsError;

#[cfg(any(feature = "geocoding", feature = "places"))]
pub use crate::types::address_component::AddressComponent;

#[cfg(any(
    feature = "directions",
    feature = "distance_matrix",
    feature = "geocoding",
    feature = "places"
))]
pub use crate::types::bounds::Bounds;

#[cfg(any(
    feature = "autocomplete",
    feature = "directions",
    feature = "geocoding"
))]
pub use crate::types::country::Country;

#[cfg(any(feature = "geocoding", feature = "places"))]
pub use crate::types::geometry::Geometry;

#[cfg(any(
    feature = "autocomplete",
    feature = "directions",
    feature = "distance_matrix",
    feature = "geocoding",
    feature = "places",
    feature = "time_zone"
))]
pub use crate::types::language::Language;

#[cfg(any(
    feature = "address_validation",
    feature = "autocomplete",
    feature = "directions",
    feature = "distance_matrix",
    feature = "elevation",
    feature = "geocoding",
    feature = "places",
    feature = "roads",
    feature = "time_zone"
))]
pub use crate::types::latlng::LatLng;

#[cfg(any(feature = "geocoding", feature = "places"))]
pub use crate::types::location_type::LocationType;

#[cfg(any(
    feature = "autocomplete",
    feature = "directions",
    feature = "distance_matrix",
    feature = "geocoding",
    feature = "places"
))]
pub use crate::types::place_type::PlaceType;

#[cfg(any(
    feature = "autocomplete",
    feature = "directions",
    feature = "distance_matrix",
    feature = "geocoding",
    feature = "places"
))]
pub use crate::types::region::Region;

// =============================================================================
//
// Optional structures:

// -----------------------------------------------------------------------------

#[cfg(feature = "reqwest")]
pub use crate::request_rate::api::Api;

// -----------------------------------------------------------------------------

#[cfg(any(feature = "directions", feature = "distance_matrix"))]
pub use crate::directions::{
    request::{
        avoid::Avoid,
        departure_time::DepartureTime,
        location::Location,
        traffic_model::TrafficModel,
        transit_mode::TransitMode,
        transit_route_preference::TransitRoutePreference,
        unit_system::UnitSystem,
        waypoint::Waypoint,
    },
    response::{
        driving_maneuver::DrivingManeuver,
        leg::Leg,
        overview_polyline::OverviewPolyline,
        polyline::Polyline,
        Response as DirectionsResponse,
        route::Route,
        status::Status as DirectionsStatus,
        step::Step,
        transit_agency::TransitAgency,
        transit_currency::TransitCurrency,
        transit_details::TransitDetails,
        transit_fare::TransitFare,
        transit_line::TransitLine,
        transit_stop::TransitStop,
        transit_time::TransitTime,
        transit_vehicle::TransitVehicle,
    },
    travel_mode::TravelMode,
    vehicle_type::VehicleType,
};

// -----------------------------------------------------------------------------

#[cfg(feature = "directions")]
pub use crate::directions::{
    error::Error as DirectionsError,
    request::Request as DirectionsRequest,
    response::{
        directions_distance::DirectionsDistance,
        directions_duration::DirectionsDuration
    },
};

// -----------------------------------------------------------------------------

#[cfg(feature = "distance_matrix")]
pub use crate::distance_matrix::{
    error::Error as DistanceMatrixError,
    request::Request as DistanceMatrixRequest,
    response::{
        Response as DistanceMatrixResponse,
        status::Status as DistanceMatrixStatus
    },
};

// -----------------------------------------------------------------------------

#[cfg(feature = "elevation")]
pub use crate::elevation::{
    error::Error as ElevationError,
    request::{
        locations::Locations as ElevationLocations,
        Request as ElevationRequest
    },
    response::{
        point::Point,
        status::Status as ElevationStatus,
        Response as ElevationResponse
    },
};

// -----------------------------------------------------------------------------

#[cfg(feature = "geocoding")]
pub use crate::geocoding::{
    error::Error as GeocodingError,
    forward::{
        component::Component as GeocodingComponent,
        ForwardRequest as GeocodingRequest
    },
    response::{
        geocoding::Geocoding,
        plus_code::PlusCode,
        Response as GeocodingResponse,
        status::Status as GeocodingStatus,
    }, // response
    reverse::ReverseRequest as GeocodingReverseRequest,
};

// -----------------------------------------------------------------------------

#[cfg(feature = "time_zone")]
pub use crate::time_zone::{
    error::Error as TimeZoneError,
    request::Request as TimeZoneRequest,
    response::{
        status::Status as TimeZoneStatus,
        Response as TimeZoneResponse
    },
};

// -----------------------------------------------------------------------------

#[cfg(feature = "autocomplete")]
pub use crate::places::Error as AutocompleteError;

#[cfg(feature = "autocomplete")]
pub use crate::places::place_autocomplete::{
    request::{
        autocomplete_type::AutocompleteType,
        Request as PlaceAutocompleteRequest
    },
    response::{
        matched_substring::MatchedSubstring,
        prediction::Prediction,
        Response as AutocompleteResponse,
        status::Status as AutocompleteStatus,
        structured_format::StructuredFormat,
        term::Term,
    },
};

// -----------------------------------------------------------------------------

#[cfg(feature = "autocomplete")]
pub use crate::places::place_autocomplete::request::Request as QueryAutocompleteRequest;

// -----------------------------------------------------------------------------

#[cfg(feature = "places")]
pub use crate::places::{
    business_status::BusinessStatus,
    error::Error as PlacesError,
    place::Place,
    place_editorial_summary::PlaceEditorialSummary,
    place_opening_hours::PlaceOpeningHours,
    place_opening_hours_period::PlaceOpeningHoursPeriod,
    place_opening_hours_period_detail::PlaceOpeningHoursPeriodDetail,
    place_photo::PlacePhoto,
    place_review::PlaceReview,
    place_special_day::PlaceSpecialDay,
    secondary_hours_type::SecondaryHoursType,
    status::Status as PlacesStatus,
};

// -----------------------------------------------------------------------------

#[cfg(feature = "places")]
pub use crate::places::place_search::text_search::{
    request::Request as TextSearchRequest,
    response::Response as TextSearchResponse,
};

// -----------------------------------------------------------------------------

#[cfg(feature = "places")]
pub use crate::places::place_details::{
    field::Field,
    request::Request as PlaceDetailsRequest,
    response::Response as PlaceDetailsResponse,
    sort_order::SortOrder,
};

// -----------------------------------------------------------------------------

#[cfg(feature = "roads")]
pub use crate::roads::{
    error::Error as RoadsError,
    error_response::ErrorResponse as RoadsErrorResponse,
    snapped_point::SnappedPoint,
    status::Status as RoadsStatus,
};

// -----------------------------------------------------------------------------

#[cfg(feature = "roads")]
pub use crate::roads::snap_to_roads::{
    request::Request as SnapToRoadsRequest,
    response::Response as SnapToRoadsResponse,
};

// -----------------------------------------------------------------------------

#[cfg(feature = "roads")]
pub use crate::roads::nearest_roads::{
    request::Request as NearestRoadsRequest,
    response::Response as NearestRoadsResponse,
};

// -----------------------------------------------------------------------------

#[cfg(feature = "address_validation")]
pub use crate::address_validation::{
    Address,
    AddressComponent as ValidationAddressComponent,
    AddressMetadata,
    ComponentName,
    ConfirmationLevel,
    Geocode,
    Granularity,
    LanguageOptions,
    PlusCode as ValidationPlusCode,
    PostalAddress,
    ProvideValidationFeedbackRequest,
    ProvideValidationFeedbackRequestQuery,
    ProvideValidationFeedbackResponse,
    UspsAddress,
    UspsData,
    ValidateAddressRequest,
    ValidateAddressRequestQuery,
    ValidateAddressResponse,
    ValidationConclusion,
    ValidationResult,
    Verdict,
    Viewport,
};