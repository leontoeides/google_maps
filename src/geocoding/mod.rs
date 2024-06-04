//! The **Geocoding API** is a service that provides geocoding and reverse
//! geocoding of addresses. It can be used to convert a street address to
//! geographic coordinates (latitude & longitude), or vice versa.
//!
//! **Geocoding** is the process of converting addresses (like a street address)
//! into geographic coordinates (like latitude and longitude), which you can use
//! to place markers on a map, or position the map.
//!
//! **Reverse geocoding** is the process of converting geographic coordinates
//! into a human-readable address.
//!
//! You can also use the Geocoding API to find the address for a given [place
//! ID](https://developers.google.com/maps/documentation/geocoding/intro#place-id).
//!
//! # [Get Started](https://developers.google.com/maps/documentation/geocoding/start)
//!
//! * **New Users**: Before you can start using the Google Maps Platform APIs
//!   and SDKs, you must sign up and create a billing account. To learn more,
//!   see [Get Started with Google Maps
//!   Platform](https://developers.google.com/maps/gmp-get-started).
//!
//! * This service is also available as part of the client-side [Google Maps
//!   JavaScript
//!   API](https://developers.google.com/maps/documentation/javascript/geocoding),
//!   or for server-side use with the [Java Client, Python Client, Go Client and
//!   Node.js Client for Google Maps
//!   Services](https://developers.google.com/maps/documentation/geocoding/client-library).
//!
//! [Start coding with our client libraries](https://developers.google.com/maps/documentation/geocoding/start#client-library)
//!
//! Client libraries make developing with the Google Maps web service APIs
//! easier by providing simple, native implementations of common tasks, such as
//! authentication, request throttling and automatic retry. The Geocoding API is
//! available in the [Java Client, Python Client, Go Client and Node.js Client
//! for Google Maps
//! Services](https://developers.google.com/maps/documentation/geocoding/client-library).
//!
//! # [Authentication, quotas, pricing, and policies](https://developers.google.com/maps/documentation/geocoding/start#auth)
//!
//! ## [Activate the API and get an API key](https://developers.google.com/maps/documentation/geocoding/start#get-a-key)
//!
//! To use the Geocoding API, you must first activate the API in the Google
//! Cloud Platform Console and obtain the proper authentication credentials.
//! You need to provide an API key in each request (or a [client ID if you have
//! a Premium
//! Plan](https://developers.google.com/maps/documentation/geocoding/get-api-key#premium-auth).
//!
//! Click the button below to flow through a process where you will:
//! 1. Create or select a project
//! 2. Enable the API
//! 3. Get an API key
//!
//! [Get Started](https://cloud.google.com/maps-platform/#get-started)
//!
//! [Learn more about authentication
//! credentials](https://developers.google.com/maps/documentation/geocoding/get-api-key).
//!
//! ## [Quotas and pricing](https://developers.google.com/maps/documentation/geocoding/start#quotas)
//!
//! Review the [usage and
//! billing](https://developers.google.com/maps/documentation/geocoding/usage-and-billing)
//! page for details on the quotas and pricing set for the Geocoding API.
//!
//! ## [Policies](https://developers.google.com/maps/documentation/geocoding/policies)
//!
//! Use of the Geocoding API must be in accordance with the [API
//! policies](https://developers.google.com/maps/documentation/geocoding/policies).
//!
//! # [Learn more](https://developers.google.com/maps/documentation/geocoding/start#learn-more)
//!
//! There’s more you can do with the Geocoding API. See the [Geocoding API
//! developer
//! guide](https://developers.google.com/maps/documentation/geocoding/intro) for
//! additional demos, examples, available parameters, status codes and error
//! messages, and other details.
//!
//! The [Geocoding API developer
//! guide](https://developers.google.com/maps/documentation/geocoding/intro)
//! describes the Geocoding API web service. It is intended for website and
//! mobile developers who want to use geocoding data within maps provided by one
//! of the Google Maps Platform APIs.

pub mod error;
pub mod forward;
pub mod response;
pub mod reverse;

// -----------------------------------------------------------------------------

const SERVICE_URL: &str = "https://maps.googleapis.com/maps/api/geocode";
const OUTPUT_FORMAT: &str = "json"; // json or xml

// -----------------------------------------------------------------------------

pub use crate::geocoding::{
    error::Error as GeocodingError,
    forward::{component::Component as GeocodingComponent, ForwardRequest as GeocodingRequest}, // forward
    response::{
        geocoding::Geocoding, plus_code::PlusCode, status::Status as GeocodingStatus,
        Response as GeocodingResponse,
    }, // response
    reverse::ReverseRequest as GeocodingReverseRequest,
}; // geocoding

pub use crate::types::{AddressComponent, Geometry, LocationType};
