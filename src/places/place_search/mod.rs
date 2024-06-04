//! The **Place Search** service lets you search for place information using a
//! variety of categories, including establishments, prominent points of
//! interest, and geographic locations. You can search for places either by
//! proximity or a text string. A Place Search returns a list of places along
//! with summary information about each place; additional information is
//! available via a Place Details query.
//!
//! ## Note: Server-side and client-side libraries
//!
//! * The Places API is also available with the [Java Client, Python Client, Go
//!   Client and Node.js Client for Google Maps
//!   Services](https://developers.google.com/maps/documentation/places/web-service/client-library).
//!   The Places API and the client libraries are for use in server
//!   applications.
//!
//! * If you're building a client-side application, take a look at the [Places
//!   SDK for Android](https://developers.google.com/maps/documentation/places/android-sdk),
//!   the [Places SDK for iOS](https://developers.google.com/maps/documentation/places/ios-sdk),
//!   and the [Places Library, Maps JavaScript API](https://developers.google.com/maps/documentation/javascript/places).
//!
//! # [Compare search types](https://developers.google.com/maps/documentation/places/web-service/search#compare-search-types)
//!
//! There are three search endpoints available with different characteristics.
//! The following table highlights some of these differences.
//!
//! | Type | [Field selection](https://developers.google.com/maps/documentation/places/web-service/search#field-selection) | Text search | [Ambiguous text](https://developers.google.com/maps/documentation/places/web-service/search#ambiguous-text) | Location filter | [Additional filters](https://developers.google.com/maps/documentation/places/web-service/search#additional-filters) |
//! |---|---|---|---|---|---|
//! | [Find Place](https://developers.google.com/maps/documentation/places/web-service/search-find-place) | ✔️ | ✔️ `input` | ✔️ | ✔️ | ❌ |
//! | [Nearby Search](https://developers.google.com/maps/documentation/places/web-service/search-nearby) | ❌ | ✔️ `keyword` | ❌ | ✔️ | ✔️ |
//! | [Text Search](https://developers.google.com/maps/documentation/places/web-service/search-text) | ❌ | ✔️ `query` | ✔️ | ✔️ | ✔️ |
//!
//! ### [Field selection](https://developers.google.com/maps/documentation/places/web-service/search#field-selection)
//!
//! There is no way to constrain Nearby Search or Text Search to only return
//! specific fields. To keep from requesting (and paying for) data that you
//! don't need, use a Find Place request instead.
//!
//! ### [Ambiguous text](https://developers.google.com/maps/documentation/places/web-service/search#ambiguous-text)
//!
//! Find Place and Text Search are optimized for matching ambiguous text across
//! a number of fields while Nearby Search is constrained to strict matches on a
//! subset of fields.
//!
//! ### [Additional filters](https://developers.google.com/maps/documentation/places/web-service/search#additional-filters)
//!
//! Nearby Search and Text Search allow additional parameters to filter results,
//! (e.g. `minprice`, `maxprice`, `opennow`, and `type`).

pub mod nearby_search;
pub mod text_search;
