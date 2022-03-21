//! The **Places API** is a service that returns information about places using
//! HTTP requests. Places are defined within this API as establishments,
//! geographic locations, or prominent points of interest.
//!
//! ## Before you begin
//!
//! * **Before you begin**: Before you start using the Places API, you need a
//! project with a billing account and the Places API enabled. We recommend
//! creating multiple Project Owners and Billing Administrators, so that you'll
//! always have someone with these roles available to your team. To learn more,
//! see [Set up in Cloud Console](https://developers.google.com/maps/documentation/places/web-service/cloud-setup).
//!
//! ## Note: Server-side and client-side libraries
//!
//! * The Places API is also available with the [Java Client, Python Client, Go
//! Client and Node.js Client for Google Maps Services](https://developers.google.com/maps/documentation/places/web-service/client-library).
//! The Places API and the client libraries are for use in server applications.
//!
//! * If you're building a client-side application, take a look at the [Places
//! SDK for Android](https://developers.google.com/maps/documentation/places/android-sdk),
//! the [Places SDK for iOS](https://developers.google.com/maps/documentation/places/ios-sdk),
//! and the [Places Library, Maps JavaScript API](https://developers.google.com/maps/documentation/javascript/places).
//!
//! # [Introducing the API](https://developers.google.com/maps/documentation/places/web-service/overview#Introduction)
//!
//! The following place requests are available:
//!
//! * [Place Search](https://developers.google.com/maps/documentation/places/web-service/search)
//! returns a list of places based on a user's location or searchstring.
//!
//! * [Place Details](https://developers.google.com/maps/documentation/places/web-service/details)
//! returns more detailed information about a specific place, including user
//! reviews.
//!
//! * [Place Photos](https://developers.google.com/maps/documentation/places/web-service/photos)
//! provides access to the millions of place-related photos stored in Google's
//! Place database.
//!
//! * [Place Autocomplete](https://developers.google.com/maps/documentation/places/web-service/autocomplete)
//! automatically fills in the name and/or address of a place as users type.
//!
//! * [Query Autocomplete](https://developers.google.com/maps/documentation/places/web-service/query)
//! provides a query prediction service for text-based geographic searches,
//! returning suggested queries as users type.
//!
//! Each of the services is accessed as an HTTP request, and returns either an
//! JSON or XML response. All requests to a Places service must use the https://
//! protocol, and include an API key.
//!
//! The Places API uses a place ID to uniquely identify a place. For details
//! about the format and usage of this identifier across the Places API and
//! other APIs, see the [Place IDs](https://developers.google.com/maps/documentation/places/web-service/place-id)
//! documentation.
//!
//! # [Policies and Terms](https://developers.google.com/maps/documentation/places/web-service/overview#Requirements)
//!
//! All applications that use the Places API or Place Autocomplete service must
//! adhere to the requirements described in the [Places API
//! Policies](https://developers.google.com/maps/documentation/places/web-service/policies#terms_of_use_and_privacy_policy_requirements)
//! and the [Google Maps Platform Terms of
//! Service](https://cloud.google.com/maps-platform/terms).
//! The Places API and Google Place Autocomplete share the usage quota as
//! described in the [Places API Usage and
//! Billing](https://developers.google.com/maps/documentation/places/web-service/usage)
//! documentation.
//!
//! For quota and pricing information for the Places API and Place Autocomplete
//! service, see the [Places API Usage and
//! Billing](https://developers.google.com/maps/documentation/places/web-service/usage-and-billing)
//! documentation.

pub mod place_autocomplete;