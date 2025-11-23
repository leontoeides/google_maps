//! `serde` helpers

mod country_code;

mod locale;
mod optional_locale;

mod optional_day;
mod optional_jiff_date;
mod optional_month;
mod optional_year;
mod weekday;

pub use crate::places_new::serde::country_code::deserialize_country_code;
pub use crate::places_new::serde::country_code::serialize_country_code;
pub use crate::places_new::serde::locale::deserialize_locale;
pub use crate::places_new::serde::locale::serialize_locale;
pub use crate::places_new::serde::optional_locale::deserialize_optional_locale;
pub use crate::places_new::serde::optional_locale::serialize_optional_locale;

pub use crate::places_new::serde::optional_day::deserialize_optional_day;
pub use crate::places_new::serde::optional_day::serialize_optional_day;
pub use crate::places_new::serde::optional_jiff_date::deserialize_optional_jiff_date;
pub use crate::places_new::serde::optional_jiff_date::serialize_optional_jiff_date;
pub use crate::places_new::serde::optional_month::deserialize_optional_month;
pub use crate::places_new::serde::optional_month::serialize_optional_month;
pub use crate::places_new::serde::optional_year::deserialize_optional_year;
pub use crate::places_new::serde::optional_year::serialize_optional_year;
pub use crate::places_new::serde::weekday::deserialize_weekday;
pub use crate::places_new::serde::weekday::serialize_weekday;

#[cfg(any(feature = "places-new-autocomplete", feature = "places-new-nearby-search", feature = "places-new-place-details", feature = "places-new-text-search"))]
mod optional_country_code;
#[cfg(any(feature = "places-new-autocomplete", feature = "places-new-nearby-search", feature = "places-new-place-details", feature = "places-new-text-search"))]
pub use crate::places_new::serde::optional_country_code::serialize_optional_country_code;

#[cfg(feature = "places-new-autocomplete")]
mod vec_country_code;
#[cfg(feature = "places-new-autocomplete")]
pub use crate::places_new::serde::vec_country_code::serialize_vec_country_code;