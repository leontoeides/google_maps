# google_maps
![Crates.io Version](https://img.shields.io/crates/v/google_maps)
![Crates.io MSRV](https://img.shields.io/crates/msrv/google_maps)
![Crates.io License](https://img.shields.io/crates/l/google_maps)
![Crates.io Total Downloads](https://img.shields.io/crates/d/google_maps)

An unofficial Google Maps Platform client library for the Rust programming language. 

This client currently implements the Directions API, Distance Matrix API, Elevation API, Geocoding API, Time Zone API, and parts of the Places and Roads API.

<img src="https://www.arkiteq.ca/crates/google_maps/banner.jpg" alt="Unofficial Google Maps Platform Client for Rust" width="400"/>

# Installation

Configure the dependencies:

```toml
[dependencies]
google_maps = "3.5"
```

Optionally, add `rust_decimal = "1"` and `rust_decimal_macros = "1"` for access to the `dec!` macro. This macro can be used to define decimal numbers in your program. 

This is useful for hard-coding latitudes and longitudes into your code for testing.

## Feature Flags

The desired Google Maps APIs can be enabled individually via feature flags.

Additionally, usage of rustls for Reqwest is supported.

### Google Maps Client Feature Flags:

* `autocomplete` ‧ includes Google Maps Places autocomplete API
* `directions` ‧ includes Google Maps Directions API
* `distance_matrix` ‧ includes Google Maps Distance Matrix API
* `elevation` ‧ includes Google Maps Elevation API
* `geocoding` ‧ includes Google Maps Geocoding API
* `places` ‧ includes Google Maps Places API
* `roads` ‧ includes Google Maps Roads API
* `time_zone` ‧ includes Google Maps Time Zone API
* `enable-reqwest` ‧ uses [reqwest](https://crates.io/crates/reqwest) for
  querying the Google Maps API
* `enable-reqwest-middleware` ‧ uses [reqwest-middleware](https://crates.io/crates/reqwest-middleware)
  for querying the Google Maps API
* `geo` ‧ support for the rust [geo](https://crates.io/crates/geo-types)
  ecosystem
* `polyline` ‧ allows easy type conversions from a `Route` or `Step` to a geo
  [LineString](https://docs.rs/geo-types/0.7.13/geo_types/geometry/struct.LineString.html)

Note: the `autocomplete` feature covers the Places API autocomplete-related services:
[Place Autocomplete requests](https://docs.rs/google_maps/latest/google_maps/prelude/struct.ClientSettings.html#method.place_autocomplete)
and [Query Autocomplete requests](https://docs.rs/google_maps/latest/google_maps/prelude/struct.ClientSettings.html#method.query_autocomplete).
All other Places API services are covered by the `places` feature.

### reqwest Feature Flags

For use with `enable-reqwest` only.

* `native-tls`
* `rustls`
* `gzip`
* `brotli`

### Default Feature Flags

By default, the Google Maps client includes all implemented Google Maps APIs. Reqwest will secure the connection using the system-native TLS (`native-tls`), and has gzip compression enabled (`gzip`).

```toml
default = [
	# Google Maps crate features:
	"directions",
	"distance_matrix",
	"elevation",
	"geocoding",
	"time_zone",
	"autocomplete",
	"roads",
	"places",

	# reqwest features:
	"enable-reqwest",
	"reqwest/default-tls",
	"reqwest/gzip",

	# rust_decimal features:
	"rust_decimal/serde",
]
```

#### Feature flag usage example

This example will only include the Google Maps Directions API. Reqwest will secure the connection using the Rustls library, and has brotli compression enabled.

```toml
google_maps = {
	version = "3.5",
	default-features = false,
	features = [
		"directions",
		"enable-reqwest",
		"rustls",
		"brotli"
	]
}
```

# Release Notes

The [full changelog is available here](https://github.com/leontoeides/google_maps/blob/master/CHANGELOG.md).

Releases [are available on GitHub](https://github.com/leontoeides/google_maps/releases).

# Examples

## Directions API

The Directions API is a service that calculates directions between locations.
You can search for directions for several modes of transportation, including
transit, driving, walking, or cycling.

```rust
use google_maps::prelude::*;

let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE");

// Example request:

let directions = google_maps_client.directions(
    // Origin: Canadian Museum of Nature
    Location::from_address("240 McLeod St, Ottawa, ON K2P 2R1"),
    // Destination: Canada Science and Technology Museum
    Location::try_from_f32(45.403_509, -75.618_904)?,
)
.with_travel_mode(TravelMode::Driving)
.execute()
.await?;

// Dump entire response:

println!("{:#?}", directions);
```

## Distance Matrix API

The Distance Matrix API is a service that provides travel distance and time for
a matrix of origins and destinations, based on the recommended route between
start and end points.

```rust
use google_maps::prelude::*;

let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE");

// Example request:

let distance_matrix = google_maps_client.distance_matrix(
    // Origins
    vec![
        // Microsoft
        Waypoint::from_address("One Microsoft Way, Redmond, WA 98052, United States"),
        // Cloudflare
        Waypoint::from_address("101 Townsend St, San Francisco, CA 94107, United States"),
    ],
    // Destinations
    vec![
        // Google
        Waypoint::from_place_id("ChIJj61dQgK6j4AR4GeTYWZsKWw"),
        // Mozilla
        Waypoint::try_from_f32(37.387_316, -122.060_008)?,
    ],
).execute().await?;

// Dump entire response:

println!("{:#?}", distance_matrix);
```

## Elevation API (Positional)

The Elevation API provides elevation data for all locations on the surface of
the earth, including depth locations on the ocean floor (which return negative
values).

```rust
use google_maps::prelude::*;

let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE");

// Example request:

let elevation = google_maps_client.elevation()
    // Denver, Colorado, the "Mile High City"
    .for_positional_request(LatLng::try_from_dec(dec!(39.739_154), dec!(-104.984_703))?)
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

## Geocoding API

The Geocoding API is a service that provides geocoding and reverse geocoding of
addresses. Geocoding is the process of converting addresses (like a street
address) into geographic coordinates (like latitude and longitude), which you
can use to place markers on a map, or position the map.

```rust
use google_maps::prelude::*;

let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE");

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

## Reverse Geocoding API

The Geocoding API is a service that provides geocoding and reverse geocoding of
addresses. Reverse geocoding is the process of converting geographic coordinates
into a human-readable address.

```rust
use google_maps::prelude::*;

let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE");

// Example request:

let location = google_maps_client.reverse_geocoding(
    // 10 Downing St, Westminster, London
    LatLng::try_from_dec(dec!(51.503_364), dec!(-0.127_625))?,
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

## Time Zone API

The Time Zone API provides time offset data for locations on the surface of the
earth. You request the time zone information for a specific latitude/longitude
pair and date. The API returns the name of that time zone, the time offset from
UTC, and the daylight savings offset.

```rust
use google_maps::prelude::*;

let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE");

// Example request:

let time_zone = google_maps_client.time_zone(
     // St. Vitus Cathedral in Prague, Czechia
     LatLng::try_from_dec(dec!(50.090_903), dec!(14.400_512))?,
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

### [Geolocation API](https://developers.google.com/maps/documentation/geolocation/intro)

Google's Geolocation API seems to be offline. While the online documentation
is still available and the API appears configurable through the Google Cloud
Platform console, the Geolocation API responds Status code `404 Not Found` with
an empty body to all requests. This API cannot be implemented until the server
responds as expected.

### Controlling Request Settings

The Google Maps client settings can be used to change the request rate and automatic retry parameters.

```rust
use google_maps::prelude::*;

let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE")
    // For all Google Maps Platform APIs, the client will limit 2 sucessful
    // requests for every 10 seconds:
    .with_rate(Api::All, 2, std::time::Duration::from_secs(10))
    // Returns the `GoogleMapsClient` struct to the caller. This struct is used
    // to make Google Maps Platform requests.
    .build();
```

# Feedback

I would like for you to be successful with your project! If this crate is not
working for you, doesn't work how you think it should, or if you have requests,
or suggestions - please [report them to
me](https://github.com/leontoeides/google_maps/issues)! I'm not always fast at
responding but I will respond. Thanks!

# Roadmap

- [ ] Track both _requests_ and request _elements_ for rate limiting.
- [ ] Make a generic `get()` function for that can be used by all APIs.
- [ ] Convert explicit query validation to session types wherever reasonable.
- [ ] [Places API](https://developers.google.com/places/web-service/intro). Only
partly implemented. If you would like to have any missing pieces implemented,
please contact me.
- [ ] [Roads API](https://developers.google.com/maps/documentation/roads/intro).
Only partly implemented. If you would like to have any missing pieces
implemented, please contact me.

# Author's Note

This crate is expected to work well and have the more important Google Maps
features implemented. It should work well because
[serde](https://crates.io/crates/serde) and, by default,
[reqwest](https://crates.io/crates/reqwest) do most of the heavy lifting!

I created this client library because I needed several Google Maps Platform
features for a project that I'm working on. So, I've decided to spin my library
off into a public crate. This is a very small token of gratitude and an attempt
to give back to the Rust community. I hope it saves someone out there some work.
