An unofficial Google Maps Platform client for the Rust programming language.
This client currently implements the Directions API, Elevation API, Geocoding
API, and Time Zone API.

# Welcome

As of version 0.1.0 this crate is expected to work well and have the more
important Google Maps features implemented. It should work well because Reqwest
and Serde do most of the heavy lifting! While it's an early release, for most
people this crate should work fine as is.

I created this crate because I needed the Google Maps Platform for a project
that I'm working on. So, I've decided to spin my library off into a public
crate. This is a small token of gratitude and an attempt to give back to the
Rust community. I hope it saves someone out there some work.

# Example Directions API Request

```
use google_maps::*;

let directions = DirectionsRequest::new(
    // Canadian Museum of Nature
    Location::Address(String::from("240 McLeod St, Ottawa, ON K2P 2R1")),
    // Canada Science and Technology Museum
    Location::Address(String::from("1867 St Laurent Blvd, Ottawa, ON K1G 5A3")),
    YOUR_GOOGLE_API_KEY_HERE
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

# To do

1. [Distance Matrix API](https://developers.google.com/maps/documentation/distance-matrix/start)
2. [Geolocation API](https://developers.google.com/maps/documentation/geolocation/intro)
3. [Places API](https://developers.google.com/places/web-service/intro)
4. [Roads API](https://developers.google.com/maps/documentation/roads/intro)
5. Automatic Rate Limiting
6. Retry on Failure