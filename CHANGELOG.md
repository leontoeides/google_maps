# Change Log

* Release notes are available on
  [GitHub](https://github.com/leontoeides/google_maps/releases).

# 3.5.3

* 2024-06-09: Increased flexibility of interface by using more `impl Into<Type>`
  parameters.

* 2024-06-09: Clean-up of `tracing` messages, instrumentation and log levels.

# 3.5.2

* 2024-06-05: New, optional `polyline` feature. This allows for easy conversion
  of `google_maps` types (such as the `Route` and the `Step`) to a
  [geo](https://crates.io/crates/geo)
  [LineString](https://docs.rs/geo-types/0.7.13/geo_types/geometry/struct.LineString.html).
  Both the `polyline` and `geo` features must be enabled for these traits and
  methods to be available.

* 2024-06-05: Improved support for rust's [geo](https://crates.io/crates/geo)
  eco-system with more type conversion traits.

# 3.5.1

* 2024-06-04: Fixed some `Vec` fields by ensuring that the serde `default` field
  attribute is applied to all `Vec` fields in response structs. Thanks to
  [chunhui2001](https://github.com/chunhui2001) for the pull request
  and to [PrinceOfBorgo](https://github.com/PrinceOfBorgo) for
  [issue #27](https://github.com/leontoeides/google_maps/issues/27).

* 2024-05-20: Added `duration` helper method to the `PlaceOpeningHoursPeriod`
  type which will measure the length of the opening hours period.

* 2024-05-19: Improved `GoogleMapsClient` interface ergonomics. Client methods
  now use `impl Into<Type>` and generics extensively. This means that rust will
  automatically take care of many type conversions needed to build your Google
  Maps requests.

* 2024-05-11: Added an `Other` variant to most enums. The will future-proof the
  crate for when Google Maps adds additional variants. Previously,
  [serde](https://crates.io/crates/serde) would return an error when
  encountering unknown variants.

# 3.5.0

* 2024-05-03: Improved ergonomics surrounding `Location` and `Waypoint` types.
  New functions: `from_address`, `from_place_id`, `try_from_f32` and
  `try_from_f64`.

* 2024-05-03: `reqwest-maybe-middleware` was forked and integrated into
  `google_maps` for now, until the crate can be updated. maybe-middleware has
  also been properly feature-gated and is turned-off by default.

* 2024-03-10: Addressed deprecated functions in the new `chrono` version
  `0.4.37`.

* 2024-03-10: The ol' `cargo clippy --fix`

* 2024-03-03: Fixes for certain feature gate combinations.

* 2024-02-28: Improved string-passing ergonomics.

* 2024-02-22: ⚠ **Breaking change**: Replaced all instances of `Option<Vec<_>>`
  with a `Vec<_>`.

	The `Option<Vec<_>>` was originally employed to get `serde` to
	deserialize Google Maps data without any extra options. However, working
	around an `Option` is extra, unnecessary work.

	`#[serde(default)]` is now used to return an empty `Vec` when there
	are no elements, rather than returning a `None`.

# 3.4.2

* 3.4.2: 2024-02-08: `google_maps::GoogleMapsClient::new` has been deprecated
  in favour of `google_maps::GoogleMapsClient::try_new`

* 3.4.2: 2024-02-08: Minor breaking change:
  `google_maps::GoogleMapsClient.with_rate` now uses a reference to the selected
  API (i.e. `.with_rate(&google_maps::Api::All)` instead of
  `.with_rate(google_maps::Api::All)`)

* 3.4.2: 2024-02-07: `README.md` makeover. Thank you
  [seanpianka](https://github.com/seanpianka)!

* 3.4.2: 2024-02-07: Applied many `clippy` suggestions.

# 3.4.1

* 3.4.1: 2023-12-23: Added `Landmark` and `Other` variants to the `PlaceType`
  enum.

* 3.4.1: 2023-12-10: Added default timeouts for the `reqwest` client.

# 3.4.0

* 3.4.0: 2023-11-15: Add Copy to types where it makes semantic sense and is
  efficient.

* 3.4.0: 2023-11-15: Switch over to use reqwest-maybe-middleware crate to enable
  users to pass in a request client with middleware.

* 3.4.0: 2023-11-15: Run `rustfmt`

	Thanks for the contributions,
	[ChristianGoldapp](https://github.com/ChristianGoldapp)!

# 3.3.2

* 3.3.2: 2023-08-08: Add support for distance_meters in Places Autocomplete
  response. Thanks for the contribution,
  [victorcrimea](https://github.com/victorcrimea)!

* 3.3.2: 2023-08-08: Use u64 type to accomodate all possible values for
  `user_ratings_total` from Google Maps API. Thanks for the contribution,
  [victorcrimea](https://github.com/victorcrimea)!

# 3.3.1

* 3.3.1: 2023-09-01: Place Details does not always return a result, now an
  `Option`.

# 3.3.0

* 3.3.0: 2023-07-23: ⚠ **Breaking change**: Corrections to Google Maps
  Places API “Text Search” implementation.

* 3.3.0: 2023-07-23: Implemented to Google Maps Places API “Nearby Search”
  interface.
```rust
use google_maps::prelude::*;
use rust_decimal_macros::dec;

let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE");

let search_results = google_maps_client.nearby_search(LatLng::try_from_dec(dec!(53.540_989), dec!(-113.493_768))?, 1_000)
    .with_type(PlaceType::Restaurant)
    .execute()
    .await?;

println!("{:#?}", search_results);
```

* 3.3.0: 2023-07-23: Improvements to documentation.

# 3.2.4

* 3.2.4: 2023-06-17: Emergency update. Important types were moved.

# 3.2.3

* 3.2.3: 2023-06-17: Fixes for using this crate's optional feature flags.

# 3.2.2

* 3.2.2: 2023-06-16: `time 0.1` dependency was removed using `chrono` feature
  flags. Thanks for the contribution, [popen2](https://github.com/popen2)!

* 3.2.2: 2023-06-16: More streamlining of crate's `Error` types. Not expected to
  have much impact to end-users.

* 3.2.2: 2023-06-16: Fixes for `geo` feature.

# 3.2.1

* 3.2.1: 2023-06-13: By default, `google_maps` will now use the `rust_decimal`
  crate's `serde` feature. To switch back to the explicit `serde-float` format,
  use the `google_maps` crate's `decimal-serde-float` feature. Thanks for the
  contribution, [popen2](https://github.com/popen2)!

# 3.2.0

* 3.2.0: 2023-06-01: ⚠ **Breaking change**: `google_maps` types will now
  round-trip through strings.

	This crate previously “took advantage” of the `String::from` and
	`ToString` traits being able to have different outputs. However, this
	clever setup did not play nice with other crates.

	This is a “breaking change” because the `Display` and `ToString` traits
	both now have different outputs compared to previous versions of the
	`google_maps` crate:

	* Previously: `println!("{}", Language::ChineseHongKong)` would result
	in `Chinese (Hong Kong)`.

	* Now: `println!("{}", Language::ChineseHongKong)` will result in
	`zh-HK`.

	* Now, to see the `Chinese (Hong Kong)` name, use the `display` method.
	For example: `println!("{}", Language::ChineseHongKong.display())`

	This update applies to all `google_maps` crate `enum` types, including
	`Country`, `PlaceType`, and so on.

* 3.2.0: 2023-05-31: ⚠ **Breaking change**: All `GoogleMapsClient` methods will
  now return the same error type. Previously, each API would return a different
  error type. This could be tedious or annoying when using several different
  APIs.

* 3.2.0: 2023-05-31: Adjusted `tracing` log levels.

* 3.2.0: 2023-05-31: Some house-keeping.

# 3.1.1

* 3.1.1: 2023-01-29: Update to Chinese languages. Thanks
  [rick68](https://github.com/rick68)!

# 3.1.0

* 3.1.0: 2022-11-27: ⚠ **Breaking change**: `Geometry.location_type` is now an
  `Option`.

* 3.1.0: 2022-11-27: Add basic support for Google Maps Places _Text Search_ and
  _Places Details_.

# 3.0.1

* 3.0.1: 2022-10-01: Added `UNKNOWN_ERROR` variant to Directions API's geocoder
  status.

# 3.0.0

* 3.0.0: 2022-09-03: ⚠ **Breaking change**: `LatLng::try_from` had to be
  renamed to `LatLng::try_from_dec` to fix name collision with the
  [TryFrom](https://doc.rust-lang.org/std/convert/trait.TryFrom.html) trait.
  Added `try_from_f32` and `try_from_f64` methods for the `LatLng` type.

* 3.0.0: 2022-09-04: Initial support for Google Maps Roads API: the **Snap To
  Roads** and the **Nearest Roads** services have been implemented. Unsure about
  supporting **Speed Limits** since, according to the documentation, it requires
  a special Google Maps plan.

* 3.0.0: 2022-09-03: Optional basic support for the
  [geo](https://crates.io/crates/geo) crate and [GeoRust](https://georust.org/)
  ecosystem. This support may be enabled using the `geo` feature flag. When the
  `geo` feature is enabled, some types may loose support for `serde`
  serialization & deserialization. If I've missed something you want or if you
  think of a better way of doing this, feel free to reach out.

* 3.0.0: 2022-09-03: This crate's `Waypoint` and `Location` types now have
  variants that represent the [geo](https://crates.io/crates/geo) crate's
  `Coordinate` and `Point` types. `Locations` type now has variants that
  represent the geo crate's `Line` and `LineString` types. It's now possible to
  make most Google Maps API requests using geo types, using the provided special
  helper methods when the `geo` feature flag is enabled.

* 3.0.0: 2022-08-27: Optional type conversion support for the
  [geo](https://crates.io/crates/geo) crate. This feature can be enabled with the
  `geo` feature flag. It makes using these crates together a little less
  burdensome. Includes some unidirectional and some bidirectional
  [TryFrom](https://doc.rust-lang.org/std/convert/trait.TryFrom.html)
  conversions between this crate's `LatLng`, `Waypoint`, `Bounds`, and geo's
  `Coordinate`, `Point`, `Rect`, `Polygon` types.

* 3.0.0: 2022-09-04: `ClientSettings` renamed to `GoogleMapsClient`.

* 3.0.0: 2022-08-27: Adjusted `tracing` log levels.

# 2.1.7

* 2.1.7: 2022-08-27: `String` to `enum` table look-ups are now powered by the
  [phf](https://crates.io/crates/phf) (perfect hash functions) crate. Added
  manual implementations of `serde` deserializers for Google Maps client types,
  which take advantage of the new `phf` tables.

* 2.1.7: 2022-08-27: Google Maps client types now implement `FromStr` which
  gives access to `parse`. For example:
  `let the_golden_boy: LatLng = "49.8845224,-97.1469436".parse()?;`

* 2.1.7: 2022-08-22: Added debug logging message to show Google Maps client's
  request activity.

# 2.1.6

* 2.1.6: 2022-08-19: Support for geocoding from Google Maps
  [Place IDs](https://developers.google.com/maps/documentation/places/web-service/place-id).
  Thank you [E-gy](https://github.com/E-gy)!

* 2.1.6: 2022-04-10: `country` was moved up the hierarchy because it's now being
  shared amongst several APIs. Made `google_maps::country` module public.

# 2.1.5

* 2.1.5: 2022-03-23: Partial support for the `Google Maps` `Places API`.
  Implemented the `Place Autocomplete` and `Query Autocomplete` services. Example
  of basic usage:
```rust
let google_maps_client = GoogleMapsClient::new("YOUR_API_KEY_HERE");

let predictions = google_maps_client.place_autocomplete("51".to_string())
    .with_location_and_radius(LatLng::try_from_dec(dec!(54), dec!(-114))?, 1_000)
    .with_type(AutocompleteType::Address)
    .execute()
    .await?;

println!("{:#?}", predictions);
```

# 2.1.3

* 2.1.3: 2021-07-22: Web Assembly (WASM) support: if Google Maps API Client's
  `default-features` are set to false, all desired reqwest features (`brotli`,
  `rustls`, etc.) must be manually added to the `Cargo.toml` file. Now, the
  `enable-reqwest` feature starts with no reqwest features so that Web Assembly
  users may rely on reqwest's JS fetch API. Also, changed `query_string()` to
  `query_url()`. Example `query_url()` usage:
```rust
use google_maps::prelude::*;

let google_maps_client = GoogleMapsClient::new("YOUR_GOOGLE_API_KEY_HERE");

// Get query string from builder pattern:
let query_url = google_maps_client.time_zone(
     LatLng::try_from_dec(dec!(50.090_903), dec!(14.400_512))?,
     Utc::now()
).query_url();

// Insert your favourite HTTP client here:
let json = reqwest::get(query_url).await?.text().await?;

// Parse JSON string into a TimeZoneResponse structure:
let time_zone: TimeZoneResponse = json.parse()?;

// Dump entire response:
println!("{:#?}", time_zone);

```

# 2.1.2

* 2.1.2: 2021-07-18: Sorry for all of the updates. Made more dependencies
  optional. This adds the ability to slim down this client when needed. Also,
  spruced up the `query_string()` methods.

# 2.1.1

* 2.1.1: 2021-07-18: House-keeping. Fixed issue with Google Maps API `features`.
  Added support for using your own HTTP client.

# 2.1.0

* 2.1.0: 2021-07-17: Transitioned from an in-house retry/backoff implementation
  to the `backoff` crate. Google Maps APIs are now optional through the use of
  feature flags. Improved examples.

# 2.0.2

* 2.0.2: 2021-07-16: Added support for using rustls-tls in reqwest dependency -
  thanks [seanpianka](https://github.com/seanpianka)! Transitioned from `log`
  crate to the `tracing` crate.

# 2.0.1

* 2.0.1: 2022-07-15: Now supports a user-configured Reqwest client in the Google
  Maps client builder.
  `GoogleMapsClient::new("YOUR_API_KEY_HERE").with_reqwest_client(your_reqwest_client).build();`

# 2.0.0

* 2.0.0: 2022-07-13: The Rust Google Maps client is now async thanks to
  [seanpianka](https://github.com/seanpianka)!

# 1.0.3

* 1.0.3: 2021-01-06: Updated dependencies. A few minor corrections. Async
  support is planned for the next month or two.

# 1.0.2

* 1.0.2: 2020-08-07: Corrected error where string formatted for display were
  being sent to the Google Maps Platform API. Thanks
  [victorct-pronto](https://github.com/victorct-pronto)!

# 1.0.1

* 1.0.1: 2020-05-25: Ensuring all public structures use Serde's serialize and
  deserialize traits. Thanks [qrayven](https://github.com/qrayven)!

# 1.0.0

* 1.0.0: 2020-05-16: Interface stable. (This was a lie!)

# 0.7.3

* 0.7.3: 2020-04-25: For the Distance-Matrix API, some response fields that
  should have been public weren't. Fixed. Thanks
  [sroebuck](https://github.com/sroebuck)!

# 0.7.2

* 0.7.2: 2020-04-21: Small bug fixes. Also, some logging was causing stack
  overflows, so it had to be disabled.

# 0.7.1

* 0.7.1: 2020-03-10: Added in as many derivable traits as possible. Changed
  transit fare amount from `f64` to `rust_decimal::Decimal`. Clean-ups as
  commanded by Clippy.

* 0.7.1: 2020-03-10: For Time Zone API requests from this crate has moved from
  expressing the time in `chrono::NaiveDateTime` to `chrono::DateTime<Utc>`. See
  the updated time zone example.

# 0.7.0

* 0.7.0: 2020-03-08: Transitioned from `f64` to `rust_decimal::Decimal` for
  latitude and longitude coordinates. This eliminates rounding errors. The
  `Decimal` type is also hashable. Nice. `LatLng`, `Waypoint`, `Location` types
  can now be used as keys for hash maps. **To define a `Decimal` value in your
  code, currently you must add the `rust_decimal_macros` dependency into your
  `Cargo.toml` file**. Use the `dec!()` macro like so: `dec!(12.345)`. This is
  the preferred way to define latitude and longitude coordinates. If you do not
  want to add this line to your `Cargo.toml` file, you may also create a
  `Decimal` from a `&str` like so: `Decimal::from_str("12.345").unwrap()`. See
  the new examples. Also, see the
  [rust_decimal](https://crates.io/crates/rust_decimal) crate for more
  information.

* 0.7.0: 2020-03-08: To better align this crate with Rust conventions, I've
  converted many `String` parameters to `&str` parameters. If you're receiving
  new compilations errors like `the trait bound google_maps::directions::response::
  driving_maneuver::DrivingManeuver: std::convert::From<std::string::String> is
  not satisfied` you will have to change your code to borrow the string. For
  example, change `TransitCurrency::try_from(currency)` to
  `TransitCurrency::try_from(&currency)` or to
  `TransitCurrency::try_from(&*currency)` if its a `String` type.

# 0.6.0

* 0.6.0: 2020-02-29: Cleaned up the `mod` and `use` declarations. To glob import
  everything from google_maps into your project, you can use the
  `use google_maps::prelude::*` convention now.

# 0.5.2

* 0.5.2: 2020-02-29: I'm a procedural programmer at heart, so using handles is
  second nature to me. In an oversight, I was forcing library users to use
  handles without being consciously aware of it. I have improved the ergonomics of
  the library. Check out the new examples.

* 0.5.2: 2020-02-29: There were inaccuracies in the rate limiting examples.
  Sorry if these poor examples caused you any frustration.

# 0.5.0

* 0.5.0: 2020-02-23: The `time` crate has deprecated the `PrimitiveDateTime`
  struct. This crate has moved from the `time` crate to the `chrono` crate.
  Since there is no reasonable way for this crate to always know which time zone
  is intended in every context, this crate relies on the `NaiveDateTime` struct.
  That means that _time_ and _time zone_ considerations must be tracked and
  handled by you, the programmer. Check into the `chrono-tz` crate which
  integrates nicely with the `chrono` crate.

# 0.4.6

* 0.4.6: 2020-02-19: Emergency update! Case conflict for TransitMode. Had to
  force to lower case in URL query string builder.

* 0.4.6: 2020-02-19: Connected Travis CI.

* 0.4.6: 2020-02-19: Added support for sub-steps in Directions API.

# 0.4.5

* 0.4.5: 2020-02-19: Emergency update! Custom deserializer for Durations was
  not included in the 0.4.4 release.

# 0.4.4

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

# 0.4.3

* 0.4.3: 2020-02-09: [Happy 15th birthday to Google
  Maps](https://www.blog.google/products/maps/maps-15th-birthday/)!

* 0.4.3: 2020-02-09: Ensured request rate limiting was applied to all API
  requests.

# 0.4.2

* 0.4.2: 2020-02-06: Unix timestamps received from the Google Maps Platform are
  now automatically deserialized into `time::PrimitiveDateTime` structs for
  convenience.

* 0.4.2: 2020-02-06: Removed precision limit for Google Maps Platform requests.

# 0.4.1

* 0.4.1: 2020-02-06: Added time zone and currency enumerations for look-up
  tables, conversions, and additional handling to be added in the future.

* 0.4.1: 2020-02-06: Fixed some errors in the examples.

* 0.4.1: 2020-02-05: Some internal restructuring to make the library more
  consistent. Improved many comments, better documentation.

# 0.4.0

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
  library will now attempt to query the Google Cloud Platform several times
  before giving up and returning an error. Temporary network hiccups will no
  longer cause your program to fail.

* 0.4.0: Implemented request rate limiting. Each API can have different request
  rate limits.

* 0.4.0: Now implements the `log` crate with some logging messages for
  debugging.