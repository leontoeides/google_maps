//! The **Geolocation API** returns a location and accuracy radius based on
//! information about cell towers and WiFi nodes that the mobile client can
//! detect.
//!
//! # [Introduction](https://developers.google.com/maps/documentation/geolocation/intro#introduction)
//!
//! This document describes the protocol used to send this data to the server
//! and to return a response to the client.
//!
//! ## [Before you begin](https://developers.google.com/maps/documentation/geolocation/intro#before-you-begin)
//!
//! Before you start developing with the Geolocation API, review the
//! [authentication
//! requirements](https://developers.google.com/maps/documentation/geolocation/get-api-key)
//! (you need an API key) and the [API usage and
//! billing](https://developers.google.com/maps/documentation/geolocation/usage-and-billing)
//! information (you need to enable billing on your project).
//!
//! [Frequently asked questions](https://developers.google.com/maps/documentation/geolocation/intro#troubleshooting)
//!
//! * Why am I getting a very large `accuracy` radius in my Geolocation response?
//!
//! If your Geolocation response shows a very high value in the `accuracy`
//! field, the service may be geolocating based on the request IP, instead of
//! WiFi points or cell towers. This can happen if no cell towers or access
//! points are valid or recognized.
//!
//! To confirm that this is the issue, set `considerIp` to `false` in your
//! request. If the response is a `404`, you've confirmed that your
//! `wifiAccessPoints` and `cellTowers` objects could not be geolocated.

pub mod request;
pub mod response;