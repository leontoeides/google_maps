# google_maps

An unofficial Google Maps Platform API for the Rust programming language.

# Welcome

As of version 0.1.0 this crate is expected to work well, work reliably, and
have the most important features implemented. There are some creature
comforts and specialized APIs not implemented yet.

While an early release, for most people this crate should work fine as is.

# Example Request:

```
use google_maps::*;

let directions = DirectionsRequest::new(
    // Canadian Museum of Nature
    Location::Address(String::from("240 McLeod St, Ottawa, ON K2P 2R1")),
    // Canada Science and Technology Museum
    Location::Address(String::from("1867 St Laurent Blvd, Ottawa, ON K1G 5A3")),
    GOOGLE_API_KEY
)
.with_travel_mode(TravelMode::Transit)
.with_arrival_time(PrimitiveDateTime::new(
    Date::try_from_ymd(2020, 1, 20).unwrap(),
    Time::try_from_hms(13, 00, 0).unwrap()
))
.validate().unwrap()
.build()
.get().unwrap();

println!("{:#?}", directions);
```

To do:
1. [Distance Matrix API](https://developers.google.com/maps/documentation/distance-matrix/start)
2. [Geolocation API](https://developers.google.com/maps/documentation/geolocation/intro)
3. [Places API](https://developers.google.com/places/web-service/intro)
4. [Roads API](https://developers.google.com/maps/documentation/roads/intro)
5. Automatic Rate Limiting
6. Retry on Failure
7. Asynchronous