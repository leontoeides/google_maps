//! The **Elevation API** provides elevation data for all locations on the
//! surface of the earth, including depth locations on the ocean floor (which
//! return negative values).
//!
//! # [Get Started](https://developers.google.com/maps/documentation/elevation/start)
//!
//! * **Notice**: New customers will need to contact
//!   [sales](https://cloud.google.com/contact/)
//!   for access to the Elevation API. You must also sign up and create a billing
//!   account. See [Get Started with Google Maps
//!   Platform](https://developers.google.com/maps/gmp-get-started).
//!
//! * This service is also available as part of the client-side [Maps JavaScript
//!   API](https://developers.google.com/maps/documentation/javascript/elevation),
//!   or for server-side use with the [Java Client, Python Client, Go Client and
//!   Node.js Client for Google Maps
//!   Services](https://developers.google.com/maps/documentation/elevation/client-library).
//!
//! # [Start coding with our client libraries](https://developers.google.com/maps/documentation/elevation/start#client-library)
//!
//! Client libraries make developing with the Google Maps web service APIs
//! easier by providing simple, native implementations of common tasks, such as
//! authentication, request throttling and automatic retry. The Elevation API is
//! available in the [Java Client, Python Client, Go Client and Node.js Client
//! for Google Maps
//! Services](https://developers.google.com/maps/documentation/elevation/client-library)
//!
//! # [Authentication, quotas, pricing, and policies](https://developers.google.com/maps/documentation/elevation/start#auth)
//!
//! ## [Activate the API and get an API key](https://developers.google.com/maps/documentation/elevation/start#get-a-key)
//!
//! To use the Elevation API, you must first activate the API in the Google
//! Cloud Platform Console and obtain the proper authentication credentials.
//! You need to provide an **API key** in each request (or a [client ID if you
//! have a Premium
//! Plan](https://developers.google.com/maps/documentation/elevation/get-api-key#premium-auth)).
//!
//! Click the button below to flow through a process where you will:
//! 1. Create or select a project
//! 2. Enable the API
//! 3. Get an API key
//!
//! [Get Started](https://cloud.google.com/maps-platform/#get-started)
//!
//! [Learn more about authentication
//! credentials](https://developers.google.com/maps/documentation/elevation/get-api-key).
//!
//! ## [Quotas and pricing](https://developers.google.com/maps/documentation/elevation/start#quotas)
//!
//! Review the [usage and billing](https://developers.google.com/maps/documentation/elevation/usage-limits)
//! page for details on the quotas and pricing set for the Elevation API.
//!
//! ## [Policies](https://developers.google.com/maps/documentation/elevation/start#policies)
//!
//! Use of the Elevation API must be in accordance with the [API
//! policies](https://developers.google.com/maps/documentation/elevation/policies).
//!
//! # [Learn more](https://developers.google.com/maps/documentation/elevation/start#learn-more)
//!
//! There’s more you can do with the Elevation API, like [requesting elevation
//! data from multiple
//! locations](https://developers.google.com/maps/documentation/elevation/intro#PointElevation)
//! and [creating elevation
//! charts](https://developers.google.com/maps/documentation/elevation/intro#CreatingElevationCharts).
//! See the [Elevation API developer
//! guide](https://developers.google.com/maps/documentation/elevation/intro)
//! for more examples and other details.
//!
//! The [Elevation API developer
//! guide](https://developers.google.com/maps/documentation/elevation/intro)
//! is intended for website and mobile developers who want to use elevation data
//! within maps provided by one of the Google Maps Platform APIs. It provides an
//! introduction to using the API and reference material on the available
//! parameters.

pub mod error;
pub mod request;
pub mod response;

// -----------------------------------------------------------------------------

const SERVICE_URL: &str = "https://maps.googleapis.com/maps/api/elevation";
const OUTPUT_FORMAT: &str = "json"; // json or xml

// -----------------------------------------------------------------------------

pub use crate::elevation::{
    error::Error as ElevationError,
    request::{locations::Locations as ElevationLocations, Request as ElevationRequest}, // request
    response::{point::Point, status::Status as ElevationStatus, Response as ElevationResponse}, // response
}; // crate::elevation
