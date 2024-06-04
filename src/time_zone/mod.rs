//! The **Time Zone API** provides time offset data for locations on the surface
//! of the earth. You request the time zone information for a specific
//! latitude/longitude pair and date. The API returns the name of that time
//! zone, the time offset from UTC, and the daylight savings offset.
//!
//! # [Get Started](https://developers.google.com/maps/documentation/timezone/starthttps://developers.google.com/maps/documentation/timezone/start)
//!
//! * **New Users**: Before you can start using the Google Maps Platform APIs
//!   and SDKs, you must sign up and create a billing account. To learn more,
//!   see [Get Started with Google Maps
//!   Platform](https://developers.google.com/maps/gmp-get-started).
//!
//! # [Start coding with our client libraries](https://developers.google.com/maps/documentation/timezone/start#client-library)
//!
//! Client libraries make developing with the Google Maps web service APIs
//! easier by providing simple, native implementations of common tasks, such as
//! authentication, request throttling and automatic retry. The Time Zone API is
//! available in the [Java Client, Python Client, Go Client and Node.js Client
//! for Google Maps
//! Services](https://developers.google.com/maps/documentation/timezone/client-library).
//!
//! # [Authentication, quotas, pricing, and policies](https://developers.google.com/maps/documentation/timezone/start#auth)
//!
//! ## [Activate the API and get an API key](https://developers.google.com/maps/documentation/timezone/start#get-a-key)
//!
//! To use the Time Zone API, you must first activate the API in the Google
//! Cloud Maps Platform and obtain the proper authentication credentials. You
//! need to provide an **API key** in each request (or a [client ID if you have
//! a Premium
//! Plan](https://developers.google.com/maps/documentation/timezone/get-api-key#premium-auth)).
//!
//! Click the button below to flow through a process where you will:
//! 1. Create or select a project
//! 2. Enable the API
//! 3. Get an API key
//!
//! [Get Started](https://cloud.google.com/maps-platform/#get-started)
//!
//! [Learn more about authentication
//! credentials](https://developers.google.com/maps/documentation/timezone/get-api-key).
//!
//! ## [Quotas and pricing](https://developers.google.com/maps/documentation/timezone/start#quotas)
//!
//! Review the [usage and
//! billing](https://developers.google.com/maps/documentation/timezone/usage-limits)
//! page for details on the quotas and pricing set for the Time Zone API.
//!
//! ## [Policies](https://developers.google.com/maps/documentation/timezone/start#policies)
//!
//! Use of the Time Zone API must be in accordance with the [API
//! policies](https://developers.google.com/maps/documentation/timezone/policies).
//!
//! # [Learn more](https://developers.google.com/maps/documentation/timezone/start#learn-more)
//!
//! In the Time Zone API, you can also set a language parameter to return
//! results in languages other than the default of English. See the [Time Zone
//! API Developer
//! Guide](https://developers.google.com/maps/documentation/timezone/intro)
//! for more examples and other details.
//!
//! The [Time Zone API Developer
//! Guide](https://developers.google.com/maps/documentation/timezone/intro)
//! is intended for website and mobile developers who want to include time data
//! on maps provided by one of the Google Maps Platform APIs. It provides an
//! introduction to using the API and reference material on the available parameters.

pub mod error;
pub mod request;
pub mod response;

// -----------------------------------------------------------------------------

const SERVICE_URL: &str = "https://maps.googleapis.com/maps/api/timezone";
const OUTPUT_FORMAT: &str = "json"; // json or xml

// -----------------------------------------------------------------------------

pub use crate::time_zone::{
    error::Error as TimeZoneError,
    request::Request as TimeZoneRequest,
    response::{status::Status as TimeZoneStatus, Response as TimeZoneResponse}, // reponse
}; // crate::time_zone
