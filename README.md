🗺 An unofficial Google Maps Platform client library for the Rust programming
language. This client currently implements the Directions API, Distance Matrix
API, Elevation API, Geocoding API, and Time Zone API.

![alt text](https://www.arkiteq.ca/crates/google_maps/banner.jpg "Unofficial Google Maps Platform Client for Rust")

# Welcome

This crate is expected to work well and have the more important Google Maps
features implemented. It should work well because
[Serde](https://crates.io/crates/serde) and, by default,
[reqwest](https://crates.io/crates/reqwest) do most of the heavy lifting!

I created this client library because I needed several Google Maps Platform
features for a project that I'm working on. So, I've decided to spin my library
off into a public crate. This is a very small token of gratitude and an attempt
to give back to the Rust community. I hope it saves someone out there some work.

# Before You Begin

* In your project's `Cargo.toml` file, under the `[dependencies]` section:

	* Add `google_maps = "2.1"`. Check
		[crates.io](https://crates.io/crates/google_maps) for the latest
		version number.

	* Optionally, add `rust_decimal = "1"` and `rust_decimal_macros = "1"`
		for access to the dec! macro. This macro can be used to define
		decimal numbers in your program. This is useful for efficiently
		hard-coding latitudes and longitudes in your code.

* The full documentation is available at [docs.rs](https://docs.rs/google_maps/)

# What's new?

* 2.1.7: 2022-08-27: `str` to `enum` table look-ups are now powered by
[phf](https://crates.io/crates/phf) (perfect hash functions.)

* 2.1.7: 2022-08-27: Manual implementation `serde` deserializers for Google Maps
types, which can utilize the new `phf` tables.

* 2.1.7: 2022-08-27: Google Maps client types now implement `FromStr` which
gives access to `parse`.

* 2.1.7: 2022-08-22: Added debug logging message to show Google Maps client's
request activity.

* 2.1.6: 2022-08-19: Support for geocoding from Google Maps
[Place IDs](https://developers.google.com/maps/documentation/places/web-service/place-id).
Thank you [E-gy](https://github.com/E-gy)!

* 2.1.6: 2022-04-10: `country` was moved up the hierarchy because it's now being
shared amongst several APIs. Made `google_maps::country` module public.

* 2.1.5: 2022-03-23: Partial support for the `Google Maps` `Places API`.
Implemented the `Place Autocomplete` and `Query Autocomplete` services.

* 2.1.3: 2021-07-22: Web Assembly (WASM) support: if Google Maps API Client's
`default-features` are set to false, all desired reqwest features (`brotli`,
`rustls`, etc.) must be manually added to the `Cargo.toml` file. Now, the
`enable-reqwest` feature starts with no reqwest features so that Web Assembly
users may rely on reqwest's JS fetch API. Also, changed `query_string()` to
`query_url()`. See
[CHANGELOG.md](https://github.com/leontoeides/google_maps/blob/master/CHANGELOG.md)
for example usage.

* 2.1.2: 2021-07-18: Made more dependencies optional. This adds the ability to
slim down this client when needed. Also, spruced up the `query_string()`
methods.

* 2.1.1: 2021-07-18: House-keeping. Fixed issue with Google Maps API `features`.
Added support for using your own HTTP client.

* 2.1.0: 2021-07-17: Transitioned from an in-house retry/backoff implementation
to the `backoff` crate. Google Maps APIs are now optional through the use of
feature flags. Improved examples.

* 2.0.2: 2021-07-16: Added support for using rustls-tls in reqwest dependency -
thanks [seanpianka](https://github.com/seanpianka)! Transitioned from `log`
crate to the `tracing` crate.

* 2.0.1: 2021-07-15: Now supports a user-configured Reqwest client in the Google
Maps client builder. `ClientSettings::new("YOUR_API_KEY_HERE").with_reqwest_client(your_reqwest_client).finalize();`

* 2.0.0: 2021-07-13: The Rust Google Maps client is now async thanks to
[seanpianka](https://github.com/seanpianka)!

* The full [change
log](https://github.com/leontoeides/google_maps/blob/master/CHANGELOG.md) is
available on GitHub.

## Example Directions API Request

The Directions API is a service that calculates directions between locations.
You can search for directions for several modes of transportation, including
transit, driving, walking, or cycling.

```rust
use google_maps::prelude::*;
use rust_decimal_macros::dec;

let google_maps_client = ClientSettings::new("YOUR_GOOGLE_API_KEY_HERE");

// Example request:

let directions = google_maps_client.directions(
    // Origin: Canadian Museum of Nature
    Location::Address(String::from("240 McLeod St, Ottawa, ON K2P 2R1")),
    // Destination: Canada Science and Technology Museum
    Location::LatLng(LatLng::try_from(dec!(45.403_509), dec!(-75.618_904))?),
)
.with_travel_mode(TravelMode::Driving)
.execute()
.await?;

// Dump entire response:

println!("{:#?}", directions);
```

## Example Distance Matrix API Request

The Distance Matrix API is a service that provides travel distance and time for
a matrix of origins and destinations, based on the recommended route between
start and end points.

```rust
use google_maps::prelude::*;
use rust_decimal_macros::dec;

let google_maps_client = ClientSettings::new("YOUR_GOOGLE_API_KEY_HERE");

// Example request:

let distance_matrix = google_maps_client.distance_matrix(
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
        Waypoint::LatLng(LatLng::try_from(dec!(37.387_316), dec!(-122.060_008))?),
    ],
).execute().await?;

// Dump entire response:

println!("{:#?}", distance_matrix);
```

## Example Elevation API Positional Request

The Elevation API provides elevation data for all locations on the surface of
the earth, including depth locations on the ocean floor (which return negative
values).

```rust
use google_maps::prelude::*;
use rust_decimal_macros::dec;

let google_maps_client = ClientSettings::new("YOUR_GOOGLE_API_KEY_HERE");

// Example request:

let elevation = google_maps_client.elevation()
    // Denver, Colorado, the "Mile High City"
    .for_positional_request(LatLng::try_from(dec!(39.739_154), dec!(-104.984_703))?)
    .execute()
    .await?;

// Dump entire response:

println!("{:#?}", elevation);

// Display all results:

if let Some(results) = &elevation.results {
    for result in results {
        println!("Elevation: {} meters", result.elevation)
    }
}
```

## Example Geocoding API Request

The Geocoding API is a service that provides geocoding and reverse geocoding of
addresses. Geocoding is the process of converting addresses (like a street
address) into geographic coordinates (like latitude and longitude), which you
can use to place markers on a map, or position the map.

```rust
use google_maps::prelude::*;

let google_maps_client = ClientSettings::new("YOUR_GOOGLE_API_KEY_HERE");

// Example request:

let location = google_maps_client.geocoding()
    .with_address("10 Downing Street London")
    .execute()
    .await?;

// Dump entire response:

println!("{:#?}", location);

// Print latitude & longitude coordinates:

for result in location.results {
    println!("{}", result.geometry.location)
}
```

## Example Reverse Geocoding API Request

The Geocoding API is a service that provides geocoding and reverse geocoding of
addresses. Reverse geocoding is the process of converting geographic coordinates
into a human-readable address.

```rust
use google_maps::prelude::*;
use rust_decimal_macros::dec;

let google_maps_client = ClientSettings::new("YOUR_GOOGLE_API_KEY_HERE");

// Example request:

let location = google_maps_client.reverse_geocoding(
    // 10 Downing St, Westminster, London
    LatLng::try_from(dec!(51.503_364), dec!(-0.127_625))?,
)
.with_result_type(PlaceType::StreetAddress)
.execute()
.await?;

// Dump entire response:

println!("{:#?}", location);

// Display all results:

for result in location.results {
    println!(
        "{}",
        result.address_components.iter()
            .map(|address_component| address_component.short_name.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
}
```

## Example Time Zone API Request

The Time Zone API provides time offset data for locations on the surface of the
earth. You request the time zone information for a specific latitude/longitude
pair and date. The API returns the name of that time zone, the time offset from
UTC, and the daylight savings offset.

```rust
use google_maps::prelude::*;
use rust_decimal_macros::dec;

let google_maps_client = ClientSettings::new("YOUR_GOOGLE_API_KEY_HERE");

// Example request:

let time_zone = google_maps_client.time_zone(
     // St. Vitus Cathedral in Prague, Czechia
     LatLng::try_from(dec!(50.090_903), dec!(14.400_512))?,
     // The time right now in UTC (Coordinated Universal Time)
     Utc::now()
).execute().await?;

// Dump entire response:

println!("{:#?}", time_zone);

// Usage example:

println!("Time at your computer: {}", Local::now().to_rfc2822());

if let Some(time_zone_id) = time_zone.time_zone_id {
    println!(
    	"Time in {}: {}",
        time_zone_id.name(),
        Utc::now().with_timezone(&time_zone_id).to_rfc2822()
    );
}
```

## [Geolocation API](https://developers.google.com/maps/documentation/geolocation/intro)

Google's Geolocation API seems to be offline. While the online documentation
is still available and the API appears configurable through the Google Cloud
Platform console, the Geolocation API responds Status code `404 Not Found` with
an empty body to all requests. This API cannot be implemented until the server
responds as expected.

## Example Client Settings

The Google Maps client settings can be used to change the request rate and
automatic retry parameters.

```rust
use google_maps::prelude::*;

let google_maps_client = ClientSettings::new("YOUR_GOOGLE_API_KEY_HERE")
    // For all Google Maps Platform APIs, the client will limit 2 sucessful
    // requests for every 10 seconds:
    .with_rate(Api::All, 2, std::time::Duration::from_secs(10))
    // Returns the `ClientSettings` struct to the caller. This struct is used to
    // make Google Maps Platform requests.
    .finalize();
```

## Feature Flags

It is possible to change the Reqwest features that are in turn used by the
Google Maps API client through feature flags. It is also possible to only
include desired Google Maps APIs by using Cargo.toml feature flags.

Google Maps Client feature flags:

* directions
* distance_matrix
* elevation
* geocoding
* time_zone
* places
* enable-reqwest

Reqwest feature flags (for use with `enable-reqwest` only):

* native-tls
* rustls
* gzip
* brotli

**Feature flag usage example**: This example will only include the Google Maps
Directions API. Reqwest will secure the connection using the Rustls library, and
has brotli compression enabled.

```toml
google_maps = {
	version = "2.1",
	default-features = false,
	features = [
		"directions",
		"enable-reqwest",
		"rustls",
		"brotli"
	]
}
```

**Default feature flag configuration**: By default, the Google Maps client
includes all implemented Google Maps APIs. Reqwest will secure the connection
using the system-native TLS (`native-tls`), and has gzip compression enabled
(`gzip`).

```toml
default = [
	"directions",
	"distance_matrix",
	"elevation",
	"geocoding",
	"time_zone",
	"autocomplete",
	"enable-reqwest",
	"reqwest/default-tls",
	"reqwest/gzip"
]
```

# Feedback

I would like for you to be successful with your project! If this crate is not
working for you, doesn't work how you think it should, or if you have requests,
or suggestions - please [report them to
me](https://github.com/leontoeides/google_maps/issues)! I'm not always fast at
responding but I will respond. Thanks!

# To do

1. Track both _requests_ and request _elements_ for rate limiting.
2. Make a generic get() function for that can be used by all APIs.
3. Convert explicit query validation to session types wherever reasonable.
4. [Places API](https://developers.google.com/places/web-service/intro). Only
partly implemented. If you would like to have any missing pieces implemented,
please contact me.
5. [Roads API](https://developers.google.com/maps/documentation/roads/intro).
There are no immediate plans for supporting this API. It's quite big and I have
no current need for it. If you would like to have to implemented, please contact
me.
