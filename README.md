An unofficial Google Maps Platform client library for the Rust programming
language. This client currently implements the Directions API, Distance Matrix
API, Elevation API, Geocoding API, and Time Zone API.

![alt text](https://www.arkiteq.ca/crates/google_maps/banner.jpg "Unofficial Google Maps Platform Client for Rust")

# Welcome

This crate is expected to work well and have the more important Google Maps
features implemented. It should work well because Reqwest and Serde do most of
the heavy lifting! While it's an early release, this crate should work fine as
is for most people.

I created this client library because I needed several Google Maps Platform
features for a project that I'm working on. So, I've decided to spin my library
off into a public crate. This is a very small token of gratitude and an attempt
to give back to the Rust community. I hope it saves someone out there some work.

## Example Directions API Request

```rust
use google_maps::*;
let mut my_settings = ClientSettings::new(YOUR_GOOGLE_API_KEY_HERE);

// Example request:

let directions = DirectionsRequest::new(
    &mut my_settings,
    // Origin: Canadian Museum of Nature
    Location::Address(String::from("240 McLeod St, Ottawa, ON K2P 2R1")),
    // Destination: Canada Science and Technology Museum
    Location::LatLng(LatLng::try_from(45.403_509, -75.618_904).unwrap()),
)
.with_travel_mode(TravelMode::Transit)
.with_arrival_time(PrimitiveDateTime::new(
    // Ensure this date is a weekday in the future or this query will return
    // zero results.
    Date::try_from_ymd(2020, 2, 6).unwrap(),
    Time::try_from_hms(13, 00, 0).unwrap()
))
.execute().unwrap();

// Dump entire response:

println!("{:#?}", directions);
```

## Example Distance Matrix API Request

```rust
use google_maps::*;
let mut my_settings = ClientSettings::new(YOUR_GOOGLE_API_KEY_HERE);

// Example request:

let distance_matrix = DistanceMatrixRequest::new(
    &mut my_settings,
    // Origins
    vec![
        // Microsoft
        Waypoint::Address(String::from("One Microsoft Way, Redmond, WA 98052, United States")),
        // Cloudflare
        Waypoint::Address(String::from("101 Townsend St, San Francisco, CA 94107, United States")),
    ],
    // Destinations
    vec![
        // Google
        Waypoint::PlaceId(String::from("ChIJj61dQgK6j4AR4GeTYWZsKWw")),
        // Mozilla
        Waypoint::LatLng(LatLng::try_from(37.387_316, -122.060_008).unwrap()),
    ],
)
.execute().unwrap();

// Dump entire response:

println!("{:#?}", distance_matrix);
```

## Example Elevation API Positional Request

```rust
use google_maps::*;
let mut my_settings = ClientSettings::new(YOUR_GOOGLE_API_KEY_HERE);

// Example request:

let elevation = ElevationRequest::new(&mut my_settings)
    // Denver, Colorado, the "Mile High City"
    .for_positional_request(LatLng::try_from(39.739_154, -104.984_703).unwrap())
    .execute().unwrap();

// Dump entire response:

println!("{:#?}", elevation);

// Parsing example:

println!("Elevation: {} meters", elevation.results.unwrap()[0].elevation);
```

## Example Geocoding API Request

```rust
use google_maps::*;
let mut my_settings = ClientSettings::new(YOUR_GOOGLE_API_KEY_HERE);

// Example request:

let location = GeocodingRequest::new(&mut my_settings)
    .with_address("10 Downing Street London")
    .execute().unwrap();

// Dump entire response:

println!("{:#?}", location);

// Parsing example:

for result in &location.results {
    println!("{}", result.geometry.location)
}
```

## Example Reverse Geocoding API Request

```rust
use google_maps::*;
let mut my_settings = ClientSettings::new(YOUR_GOOGLE_API_KEY_HERE);

// Example request:

let location = GeocodingReverseRequest::new(
    &mut my_settings,
    // 10 Downing St, Westminster, London
    LatLng::try_from(51.503_364, -0.127_625).unwrap(),
)
.with_result_type(PlaceType::StreetAddress)
.execute().unwrap();

// Dump entire response:

println!("{:#?}", location);

// Parsing example:

for result in &location.results {
    for address_component in &result.address_components {
        print!("{} ", address_component.short_name);
    }
    println!(""); // New line.
}
```

## Example Time Zone API Request

```rust
use google_maps::*;
let mut my_settings = ClientSettings::new(YOUR_GOOGLE_API_KEY_HERE);

// Example request:

let time_zone = TimeZoneRequest::new(
    &mut my_settings,
    // St. Vitus Cathedral in Prague, Czechia
    LatLng::try_from(50.090_903, 14.400_512).unwrap(),
    PrimitiveDateTime::new(
        // Tuesday February 15, 2022
        Date::try_from_ymd(2022, 2, 15).unwrap(),
        // 6:00:00 pm
        Time::try_from_hms(18, 00, 0).unwrap(),
    ),
).execute().unwrap();

// Dump entire response:

println!("{:#?}", time_zone);

// Parsing example:

use std::time::{SystemTime, UNIX_EPOCH};

let unix_timestamp =
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

println!("Time at your computer: {}", unix_timestamp);

println!("Time in {}: {}",
    time_zone.time_zone_id.unwrap(),
    unix_timestamp as i64 + time_zone.dst_offset.unwrap() as i64 +
        time_zone.raw_offset.unwrap() as i64
);
```

## [Geolocation API](https://developers.google.com/maps/documentation/geolocation/intro)

Google's Geolocation API seems to be offline. While the online documentation
is still available and the API appears configurable through the Google Cloud
Platform console, the Geolocation API responds Status code `404 Not Found` with
an empty body to all requests. This API cannot be implemented until the server
responds as expected.

# Feedback

I would like for you to be successful with your project! If this crate is not
working for you, doesn't work how you think it should, or if you have requests,
or suggestions - please [report them to
me](https://github.com/leontoeides/google_maps/issues)! I'm not always fast at
responding but I will respond. Thanks!

# Change Log

* 0.4.6: 2020-02-19: Emergency update! Case conflict for TransitMode. Had to
force to lower case in URL query string builder.

* 0.4.6: 2020-02-19: Connected Travis CI.

* 0.4.6: 2020-02-19: Added support for sub-steps in Directions API.

* 0.4.5: 2020-02-19: Emergency update! Custom deserializer for Durations was
not included in the 0.4.4 release.

* 0.4.4: 2020-02-19: Interface should be stablizing.

* 0.4.4: Added some helper functions for destructuring responses.

* 0.4.4: Ensured response structures are all declared as public.

* 0.4.4: 2020-02-18: Aliased `Distance` and `Duration` structs to
`DirectionsDistance` and `DirectionsDuration` respectively to prevent name
collisions.

* 0.4.4: 2020-02-18: Changed `DirectionsDuration.value` type from `u32` to
`time::Duration` type.

* 0.4.4: 2020-02-18: Dropped my custom Serde deserializer in favour of the
`time` crate's built-in _Serde_ feature.

* 0.4.4: 2020-02-17: Added support for waypoint optimization.

* 0.4.3: 2020-02-09: [Happy 15th birthday to Google
Maps](https://www.blog.google/products/maps/maps-15th-birthday/)!

* 0.4.3: 2020-02-09: Ensured request rate limiting was applied to all API
requests.

* 0.4.2: 2020-02-06: Unix timestamps received from the Google Maps Platform are
now automatically deserialized into `time::PrimitiveDateTime` structs for
convenience.

* 0.4.2: 2020-02-06: Removed precision limit for Google Maps Platform requests.

* 0.4.1: 2020-02-06: Added time zone and currency enumerations for look-up
tables, conversions, and additional handling to be added in the future.

* 0.4.1: 2020-02-06: Fixed some errors in the examples.

* 0.4.1: 2020-02-05: Some internal restructuring to make the library more
consistent. Improved many comments, better documentation.

* 0.4.0: ⚠ **Breaking change**: API keys are no longer passed directly to
Google Maps requests. Now, a structure containing your API key, and several
optional settings, is passed instead. For example:

Before:
```rust
let location = GeocodingReverseRequest::new(
    YOUR_GOOGLE_API_KEY_HERE,
    // 10 Downing St, Westminster, London
    LatLng { lat: 51.5033635, lng: -0.1276248 }
)
```

After. Note to Rust newbies: you may need to change the `?` to an `.unwrap()`
if you're running these examples in your `main()` function.
```rust
let my_settings = ClientSettings::new(YOUR_GOOGLE_API_KEY_HERE);
let location = GeocodingReverseRequest::new(
    &mut my_settings,
    // 10 Downing St, Westminster, London
    LatLng(LatLng::try_from(51.5033635, -0.1276248)?),
)
```

* 0.4.0: ⚠ **Breaking change**: All response structures, such as
`DirectionsResponse`, have been altered.

* 0.4.0: ⚠ **Breaking change**: All LatLng enum variants have had the
{ lat, lng } fields removed in favour of LatLng structs. Use
`LatLng::try_from(lat, lng)` to define latitude/longitude pairs. See the
updated examples.

* 0.4.0: ⚠ **Breaking change**: The Elevation API methods
`positional_request()` & `sampled_path_request()` have been renamed to
`for_positional_request()` & `for_sampled_path_request()` respectively. See the
updated examples.

* 0.4.0: ⚠ **Breaking change**: All `f32` fields have been increased to `f64`
fields.

* 0.4.0: Implemented automatic retry with exponential backoff. This client
library will now attempt to query the Google Cloud Platform several times before
giving up and returning an error. Temporary network hiccups will no longer cause
your program to fail.

* 0.4.0: Implemented request rate limiting. Each API can have different request
rate limits.

* 0.4.0: Now implements the `log` crate with some logging messages for
debugging.

# To do

1. Track both _requests_ and request _elements_ for rate limiting.
2. Make a generic get() function for that can be used by all APIs.
3. Look into making APIs optional, i.e. features. Possible? Desirable?
4. Look into the prelude::* convention.
5. Look into integrating [yaiouom](https://crates.io/crates/yaiouom).
6. Convert explicit query validation to session types wherever reasonable.
7. [Places API](https://developers.google.com/places/web-service/intro). There
are no immediate plans for supporting this API. It's quite big and I have no
current need for it. If you would like to have to implemented, please contact
me.
8. [Roads API](https://developers.google.com/maps/documentation/roads/intro).
There are no immediate plans for supporting this API. It's quite big and I have
no current need for it. If you would like to have to implemented, please contact
me.