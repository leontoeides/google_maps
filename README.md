An unofficial Google Maps Platform client for the Rust programming language.
This client currently implements the Directions API, Elevation API, Geocoding
API, and Time Zone API.

# Welcome

This crate is expected to work well and have the more important Google Maps
features implemented. It should work well because Reqwest and Serde do most of
the heavy lifting! While it's an early release, this crate should work fine as
is for most people.

I created this library because I needed several Google Maps Platform features
for a project that I'm working on. So, I've decided to spin my library off into
a public crate. This is a very small token of gratitude and an attempt to give
back to the Rust community. I hope it saves someone out there some work.

## Example Directions API Request

```rust
use google_maps::*;

let directions = DirectionsRequest::new(
    YOUR_GOOGLE_API_KEY_HERE,
    // Canadian Museum of Nature
    Location::Address(String::from("240 McLeod St, Ottawa, ON K2P 2R1")),
    // Canada Science and Technology Museum
    Location::LatLng { lat: 45.403509, lng: -75.618904 },
)
.with_travel_mode(TravelMode::Transit)
.with_arrival_time(PrimitiveDateTime::new(
    Date::try_from_ymd(2020, 1, 20).unwrap(),
    Time::try_from_hms(13, 00, 0).unwrap()
))
.execute().unwrap();

println!("{:#?}", directions);
```

## Example Elevation API Positional Request

```rust
use google_maps::*;

let elevation = ElevationRequest::new(YOUR_GOOGLE_API_KEY_HERE)
.positional_request(ElevationLocations::LatLngs(vec![
    // Denver, Colorado, the "Mile High City"
    LatLng { lat: 39.7391536, lng: -104.9847034 },
]))
.execute().unwrap();

println!("{:#?}", elevation);
```

## Example Geocoding API Request

```rust
use google_maps::*;

// Example request:

let location = GeocodingRequest::new(YOUR_GOOGLE_API_KEY_HERE)
.with_address("10 Downing Street London")
.execute().unwrap();

// Dump response:

println!("{:#?}", location);

// Parsing example:

for result in &location.results {
    println!(
        "Latitude: {:.7}, Longitude: {:.7}",
        result.geometry.location.lat, result.geometry.location.lng
    );
}
```

## Example Reverse Geocoding API Request

```rust
use google_maps::*;

// Example request:

let location = GeocodingReverseRequest::new(
    YOUR_GOOGLE_API_KEY_HERE,
    // 10 Downing St, Westminster, London
    LatLng { lat: 51.5033635, lng: -0.1276248 }
)
.with_result_type(PlaceType::StreetAddress)
.execute().unwrap();

// Dump response:

println!("{:#?}", location);

// Parsing example:

for result in &location.results {
    for address_component in &result.address_components {
        print!("{} ", address_component.short_name);
    }
    println!("");
}
```

## Example Time Zone API Request

```rust
use google_maps::*;

let time_zone = TimeZoneRequest::new(
    YOUR_GOOGLE_API_KEY_HERE,
    // St. Vitus Cathedral in Prague, Czechia
    LatLng { lat: 50.090903, lng: 14.400512 },
    PrimitiveDateTime::new(
        // Tuesday February 15, 2022
        Date::try_from_ymd(2022, 2, 15).unwrap(),
        // 6:00:00 pm
        Time::try_from_hms(18, 00, 0).unwrap(),
    ),
).execute().unwrap();

println!("{:#?}", time_zone);
```

# To do

1. [Distance Matrix API](https://developers.google.com/maps/documentation/distance-matrix/start)
2. [Geolocation API](https://developers.google.com/maps/documentation/geolocation/intro)
3. [Places API](https://developers.google.com/places/web-service/intro)
4. [Roads API](https://developers.google.com/maps/documentation/roads/intro)
5. Automatic Rate Limiting
6. Retry on Failure