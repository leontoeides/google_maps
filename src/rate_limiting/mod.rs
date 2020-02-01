//! Contains the `ClientSettings` struct and its associated traits.
//! `ClientSettings` is used to set your API key and the settings for governing
//! your requests.

use log::{info, trace};
use serde::{Serialize, Deserialize};
use time::{Duration, Instant};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Api {
    All,
    Directions,
    DistanceMatrix,
    Elevation,
    Geocoding,
    TimeZone,
} // enum

impl std::convert::From<Api> for String {
    /// Converts a `Api` enum to a `String` that contains an API name.
    fn from(api: Api) -> String {
        match api {
            Api::All => String::from("All"),
            Api::Directions => String::from("Directions"),
            Api::DistanceMatrix => String::from("Distance Matrix"),
            Api::Elevation => String::from("Elevation"),
            Api::Geocoding => String::from("Geocoding"),
            Api::TimeZone => String::from("Time Zone"),
        } // match
    } // fn
} // impl


enum TimeUnit {
    Milliseconds,
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
    Years,
} // enum

enum RateType {
    Requests,
} // enum

fn duration_human_display(time_in_secs: f64) -> String {

    let adjusted_units = if time_in_secs < 1.0 {
        (time_in_secs / 0.001, TimeUnit::Milliseconds)
    } else if time_in_secs < 60.0 {
        (time_in_secs, TimeUnit::Seconds)
    } else if time_in_secs < 3_600.0 {
        (time_in_secs / 60.0, TimeUnit::Minutes)
    } else if time_in_secs < 86_400.0 {
        (time_in_secs / 3_600.0, TimeUnit::Hours)
    } else if time_in_secs < 604_800.0 {
        (time_in_secs / 86_400.0, TimeUnit::Days)
    } else if time_in_secs < 2_629_746.0 {
        (time_in_secs / 604_800.0, TimeUnit::Weeks)
    } else if time_in_secs < 31_557_600.0 {
        (time_in_secs / 2_629_746.0, TimeUnit::Months)
    } else {
        (time_in_secs / 31_557_600.0, TimeUnit::Years)
    }; // if

    let quantity_string = if adjusted_units.0 < 1.0 {
        format!("{:.3}", adjusted_units.0)
    } else if time_in_secs < 10.0 {
        format!("{:.2}", adjusted_units.0)
    } else if time_in_secs < 100.0 {
        format!("{:.1}", adjusted_units.0)
    } else {
        format!("{:.0}", adjusted_units.0)
    }; // if

    let units_string = if quantity_string == "1" {
        match adjusted_units.1 {
            TimeUnit::Milliseconds => String::from("millisecond"),
            TimeUnit::Seconds => String::from("second"),
            TimeUnit::Minutes => String::from("minute"),
            TimeUnit::Hours => String::from("hour"),
            TimeUnit::Days => String::from("day"),
            TimeUnit::Weeks => String::from("week"),
            TimeUnit::Months => String::from("month"),
            TimeUnit::Years => String::from("year"),
        } // match
    } else {
        match adjusted_units.1 {
            TimeUnit::Milliseconds => String::from("milliseconds"),
            TimeUnit::Seconds => String::from("seconds"),
            TimeUnit::Minutes => String::from("minutes"),
            TimeUnit::Hours => String::from("hours"),
            TimeUnit::Days => String::from("days"),
            TimeUnit::Weeks => String::from("weeks"),
            TimeUnit::Months => String::from("months"),
            TimeUnit::Years => String::from("years"),
        } // match
    }; // if

    format!("{} {}", quantity_string, units_string)

} // fn

fn rate_human_display(requests: u64, time_in_secs: f64, rate_type: RateType) -> String {

    let adjusted_units = if time_in_secs < 0.01 {
        (requests as f64 / time_in_secs * 0.001, TimeUnit::Milliseconds)
    } else if time_in_secs < 60.0 {
        (requests as f64 / time_in_secs, TimeUnit::Seconds)
    } else if time_in_secs < 3_600.0 {
        (requests as f64 / time_in_secs * 60.0, TimeUnit::Minutes)
    } else if time_in_secs < 86_400.0 {
        (requests as f64 / time_in_secs * 3_600.0, TimeUnit::Hours)
    } else if time_in_secs < 604_800.0 {
        (requests as f64 / time_in_secs * 86_400.0, TimeUnit::Days)
    } else if time_in_secs < 2_629_746.0 {
        (requests as f64 / time_in_secs * 604_800.0, TimeUnit::Weeks)
    } else if time_in_secs < 31_556_952.0 {
        (requests as f64 / time_in_secs * 2_629_746.0, TimeUnit::Months)
    } else {
        (requests as f64 / time_in_secs * 31_556_952.0, TimeUnit::Years)
    }; // if

    let quantity_string = if adjusted_units.0 < 1.0 {
        format!("{:.3}", adjusted_units.0)
    } else if time_in_secs < 10.0 {
        format!("{:.2}", adjusted_units.0)
    } else if time_in_secs < 100.0 {
        format!("{:.1}", adjusted_units.0)
    } else {
        format!("{:.0}", adjusted_units.0)
    }; // if

    let units_string = match adjusted_units.1 {
        TimeUnit::Milliseconds => String::from("millisecond"),
        TimeUnit::Seconds => String::from("second"),
        TimeUnit::Minutes => String::from("minute"),
        TimeUnit::Hours => String::from("hour"),
        TimeUnit::Days => String::from("day"),
        TimeUnit::Weeks => String::from("week"),
        TimeUnit::Months => String::from("month"),
        TimeUnit::Years => String::from("year"),
    }; // if

    let rate_type_string = if quantity_string == "1" {
        match rate_type {
            RateType::Requests => String::from("request"),
        } // match
    } else {
        match rate_type {
            RateType::Requests => String::from("requests"),
        } // match
    }; // if

    format!("{} {} per {}", quantity_string, rate_type_string, units_string)

} // fn


#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ApiLimit {
    first_request: Option<time::Instant>,
    total_request_count: Option<u64>,
    requests_limit: u16,
    per_duration: Duration,
} // struct

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RateLimiting {
    pub all: Option<ApiLimit>,
    pub directions: Option<ApiLimit>,
    pub distance_matrix: Option<ApiLimit>,
    pub elevation: Option<ApiLimit>,
    pub geocoding: Option<ApiLimit>,
    pub time_zone: Option<ApiLimit>,
} // struct

impl RateLimiting {

    pub fn new() -> RateLimiting {
        RateLimiting {
            all: None,
            directions: None,
            distance_matrix: None,
            elevation: None,
            geocoding: None,
            time_zone: None,
        } // RateLimiting
    } // fn

    pub fn with_rate_limit(&mut self, api: Api, requests_limit: u16, per_duration: Duration) -> &mut RateLimiting {
        // Select RateLimiting field from the Api specified by the caller.
        let api_ref = match api {
            Api::All => &mut self.all,
            Api::Directions => &mut self.directions,
            Api::DistanceMatrix => &mut self.distance_matrix,
            Api::Elevation => &mut self.elevation,
            Api::Geocoding => &mut self.geocoding,
            Api::TimeZone => &mut self.time_zone,
        }; // api
        // Has the ApiLimit been set already?
        match api_ref {
            // If so, preserve the "first_request" and "total_request_count"
            // fields:
            Some(api_limit) => *api_ref = Some(ApiLimit {
                first_request: api_limit.first_request,
                total_request_count: api_limit.total_request_count,
                requests_limit,
                per_duration,
            }), // ApiLimit
            // If not, initialize the structure:
            None => *api_ref = Some(ApiLimit {
                first_request: None,
                total_request_count: None,
                requests_limit,
                per_duration,
            }), // ApiLimit
        } // match
        self
    } // fn

    pub fn limit(&mut self, api: Api) {
        // Select the ApiLimit requested by the caller:
        let api_ref = match api {
            Api::All => &mut self.all,
            Api::Directions => &mut self.directions,
            Api::DistanceMatrix => &mut self.distance_matrix,
            Api::Elevation => &mut self.elevation,
            Api::Geocoding => &mut self.geocoding,
            Api::TimeZone => &mut self.time_zone,
        }; // api
        match *api_ref {
            // No rate-limiting is defined for caller's API, do nothing:
            None => { trace!("Rate limiting is not enabled for the `{} `API.", String::from(api)) },
            //
            Some(ref mut rate) => {
                if rate.first_request == None || rate.total_request_count == None {
                    // If this is the first request for the API - initialize fields,
                    // no rate limiting required:
                    trace!("Rate limiting is enabled for the `{}` API. No rate limiting is required for the first request.", String::from(api));
                    rate.first_request = Some(Instant::now());
                    rate.total_request_count = Some(1);
                } else {
                    trace!("Rate limiting is enabled for the `{}` API.", String::from(api));

                    // Calculate the current rate and target rate:

                    let total_request_count = rate.total_request_count.unwrap();
                    let seconds_since_first_request = rate.first_request.unwrap().elapsed().as_seconds_f64();

                    let current_rate = total_request_count as f64 / seconds_since_first_request;
                    let target_rate = rate.requests_limit as f64 / rate.per_duration.as_seconds_f64();

                    let difference = current_rate - target_rate;

                    trace!(
                        "{} requests to the API this session. This API's session began {} ago.",
                        rate.total_request_count.unwrap(),
                        duration_human_display(rate.first_request.unwrap().elapsed().as_seconds_f64())
                    );

                    trace!("Current rate: {}. Target rate: {}.",
                        rate_human_display(
                            total_request_count,
                            seconds_since_first_request,
                            RateType::Requests
                        ),
                        rate_human_display(
                            rate.requests_limit as u64,
                            rate.per_duration.as_seconds_f64(),
                            RateType::Requests
                        ),
                    );

                    if difference > 0.0 {
//                        if difference < 1.0 {
                            let inversion = ((1.0 / target_rate) + difference).round() as u64;
                            info!("Throttling. Sleeping for {}.", duration_human_display(inversion as f64));
                            std::thread::sleep(std::time::Duration::from_secs(inversion));
  //                      } else {
    //                        info!("Throttling. Sleeping for {} seconds.", difference.round() as u64);
      //                      std::thread::sleep(std::time::Duration::from_secs(difference.round() as u64));
        //                }
                    } else {
                        info!("Rate is below the maximum. No limiting required.");
                    } // if

                    rate.total_request_count = Some(rate.total_request_count.unwrap() + 1);

                } // if
            } // case
        } // match

    } // fn

} // impl