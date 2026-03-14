//! Resources (enums, structs) for processing the _Time Zone API_ response from
//! the Google Maps Platform. Look in here for more information about the data
//! returned from Google's server and how to parse it with your program.

pub mod status;

// -----------------------------------------------------------------------------

use crate::time_zone::{Error, Status};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// The response from the Google Maps Time Zone API will be stored in this
/// structure.
///
/// [Time Zone Responses](https://developers.google.com/maps/documentation/timezone/intro#Responses)
/// ------------------------------------------------------------------------------------------------
/// For each valid request, the time zone service will return a response in the
/// format indicated within the request URL.
///
/// [Calculating the Local Time](https://developers.google.com/maps/documentation/timezone/intro#CalculatingTime)
///
/// The local time of a given location is the sum of the timestamp parameter,
/// and the dstOffset and rawOffset fields from the result.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Response {
    /// The offset for daylight-savings time in seconds. This will be zero if
    /// the time zone is not in Daylight Savings Time during the specified
    /// `time`.
    #[serde(rename = "dstOffset")]
    #[serde(alias = "dst_offset")]
    #[serde(default)]
    pub dst_offset: Option<i32>,

    /// More detailed information about the reasons behind the given status
    /// code, if other than `OK`.
    ///
    /// **Note**: This field is not guaranteed to be always present, and its
    /// content is subject to change.
    #[serde(rename = "errorMessage")]
    #[serde(alias = "error_message")]
    #[serde(default)]
    pub error_message: Option<String>,

    /// The offset from UTC (in seconds) for the given location. This does not
    /// take into effect daylight savings.
    #[serde(rename = "rawOffset")]
    #[serde(alias = "raw_offset")]
    #[serde(default)]
    pub raw_offset: Option<i32>,

    /// The status of the response.
    pub status: Status,

    /// A string containing the ID of the time zone, such as
    /// `America/Los_Angeles` or `Australia/Sydney`. These IDs are defined by
    /// Unicode [Common Locale Data Repository (CLDR)
    /// project](http://cldr.unicode.org/), and currently available in file
    /// [timezone.xml](http://unicode.org/repos/cldr/trunk/common/bcp47/timezone.xml).
    /// When a timezone has several IDs, the canonical one is returned. In
    /// timezone.xml, this is the first rename of each timezone. For example,
    /// `Asia/Calcutta` is returned, not `Asia/Kolkata`.
    #[serde(rename = "timeZoneId")]
    #[serde(alias = "time_zone_id")]
    #[serde(default)]
    pub time_zone_id: Option<chrono_tz::Tz>,

    /// A string containing the long form name of the time zone. This field will
    /// be localized if the language parameter is set. eg. "Pacific Daylight
    /// Time" or "Australian Eastern Daylight Time"
    #[serde(rename = "timeZoneName")]
    #[serde(alias = "time_zone_name")]
    #[serde(default)]
    pub time_zone_name: Option<String>,
} // struct

// -----------------------------------------------------------------------------

impl std::convert::TryFrom<String> for Response {
    type Error = serde_json::Error;
    /// Convert a Google Maps API [JSON](https://en.wikipedia.org/wiki/JSON)
    /// `String` response into a `Response` struct.
    fn try_from(s: String) -> Result<Self, Self::Error> {
        serde_json::from_slice(&s.into_bytes())
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::str::FromStr for Response {
    type Err = serde_json::Error;
    /// Converts a Google Maps API [JSON](https://en.wikipedia.org/wiki/JSON)
    /// `&str` response into a `Response` struct.
    ///
    /// # Notes
    ///
    /// * It's recommended to use the implemented `TryFrom` trait instead.
    ///
    /// * The [serde_json](https://crates.io/crates/simd-json)'s `from_str`
    ///   function implementation is unsafe and it's `from_slice` function
    ///   requires a mutable reference. Therefore this trait clones the `&str`
    ///   into a `String` to give `from_slice` mutable access to the string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.to_string().into_bytes();
        serde_json::from_slice(&bytes)
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl std::convert::From<Response> for Result<Response, crate::time_zone::Error> {
    /// Converts a Google Maps API `Response` into a `Result<Response, Error>`
    /// by examining the `status` field inside of the response.
    ///
    /// If the status indicates a success, then an `Ok(response)` will be
    /// returned. If the status indicates an error, then an `Err(error)` will be
    /// returned.
    fn from(response: Response) -> Self {
        match response.status {
            Status::Ok => Ok(response),
            Status::InvalidRequest => Err(Error::InvalidRequest),
            Status::OverQueryLimit => Err(Error::OverQueryLimit),
            Status::OverDailyLimit => Err(Error::OverDailyLimit),
            Status::RequestDenied => Err(Error::RequestDenied),
            Status::UnknownError => Err(Error::UnknownError),
            Status::ZeroResults => Err(Error::ZeroResults),
        } // match
    } // fn
} // impl
