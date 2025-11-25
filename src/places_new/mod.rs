//! **Places API (New)** - Get location data for over 200 million places, and add place details,
//! search, and autocomplete to your apps.
//!
//! # [Migration overview](https://developers.google.com/maps/documentation/places/web-service/legacy/migrate-overview)
//!
//! > **European Economic Area (EEA) developers** If your billing address is in the European
//! > Economic Area, effective on 8 July 2025, the
//! > [Google Maps Platform EEA Terms of Service](https://cloud.google.com/terms/maps-platform/eea)
//! > will apply to your use of the Services. Functionality varies by region.
//! > [Learn more](https://developers.google.com/maps/comms/eea/faq).
//!
//! ## [Introduction](https://developers.google.com/maps/documentation/places/web-service/legacy/migrate-overview#introduction)
//!
//! Places API (New) provides improved performance and a new pricing model, making it worthwhile to
//! update apps that use the Places API (Legacy). For more details on comparing features, see
//! [Choose your API](https://developers.google.com/maps/documentation/places/web-service/choose-api).
//!
//! Use this guide to understand key differences between Places API (New) compared to Places API
//! (Legacy), along with how to handle necessary changes.
//!
//! ## [Billing Best Practices for Migration](https://developers.google.com/maps/documentation/places/web-service/legacy/migrate-overview#billing-best-practices-for-migration)
//!
//! ⚠️ **Warning:** This guidance applies if your API usage is high enough to move into second-tier
//! pricing. When migrating to a newer version of an API, you're also being billed for a different
//! SKU. To avoid increased costs during the month of your transition, we recommend switching to the
//! new APIs in production as close to the beginning of the month as possible. This will ensure that
//! you reach the most cost-effective monthly pricing tiers during the migration month. For
//! information about pricing tiers, see the
//! [pricing page](https://developers.google.com/maps/billing-and-pricing/pricing) and the
//! [pricing FAQ](https://developers.google.com/maps/billing-and-pricing/faq).
//!
//! ## [Enable Places API (New)](https://developers.google.com/maps/documentation/places/web-service/legacy/migrate-overview#enable-new)
//!
//! To use the features of Places API (New), you must first enable **Places API (New)** in your
//! Google Cloud project. For more information, see [Set up your Google Cloud
//! project](https://developers.google.com/maps/documentation/places/web-service/cloud-setup).
//!
//! You then must ensure that you have added **Places API (New)** to the API key used by your app.
//! For more information, see [Use API
//! Keys](https://developers.google.com/maps/documentation/places/web-service/get-api-key).
//!
//! > **Note:** If you are using both Places API (Legacy) and Places API (New), we recommend that
//! > you create a separate API key for each API version.
//!
//! ## [General Changes](https://developers.google.com/maps/documentation/places/web-service/legacy/migrate-overview#general_changes)
//!
//! Some general changes that apply to multiple APIs include:
//!
//! - All new APIs support both
//!   [API keys](https://developers.google.com/maps/documentation/places/web-service/get-api-key) and
//!   [OAuth](https://developers.google.com/maps/documentation/places/web-service/oauth-token)
//!   tokens as the authentication mechanism.
//!
//! - Only JSON is supported as a response format.
//!
//! - Field masking is required by Place Details (New), Nearby Search (New), and Text Search (New)
//!   to specify which fields you want returned in the response. For more information, see
//!   [FieldMask](https://developers.google.com/maps/documentation/places/web-service/choose-fields).
//!
//! - The [Nearby Search (New)](https://developers.google.com/maps/documentation/places/web-service/nearby-search)
//!   and [Text Search (New)](https://developers.google.com/maps/documentation/places/web-service/text-search)
//!   APIs now return the full place details to match the place details returned by
//!   [Place Details (New)](https://developers.google.com/maps/documentation/places/web-service/place-details).
//!
//! - The JSON response format for the
//!   [Place Details (New)](https://developers.google.com/maps/documentation/places/web-service/place-details),
//!   [Nearby Search (New)](https://developers.google.com/maps/documentation/places/web-service/nearby-search), and
//!   [Text Search (New)](https://developers.google.com/maps/documentation/places/web-service/text-search) APIs
//!   has changed from the format of the existing APIs. For more details, see
//!   [Migrate the Places API response](https://developers.google.com/maps/documentation/places/web-service/migrate-response).
//!
//! ## [API-Specific Changes](https://developers.google.com/maps/documentation/places/web-service/legacy/migrate-overview#api-specific_changes)
//!
//! This section includes the following migration guides for each API:
//!
//! - [Migrate to Nearby Search (New)](https://developers.google.com/maps/documentation/places/web-service/migrate-nearby)
//! - [Migrate to Text Search (New)](https://developers.google.com/maps/documentation/places/web-service/migrate-text)
//! - [Migrate to Place Details (New)](https://developers.google.com/maps/documentation/places/web-service/migrate-details)
//! - [Migrate to Place Photos (New)](https://developers.google.com/maps/documentation/places/web-service/migrate-photo)
//! - [Migrate to Autocomplete (New)](https://developers.google.com/maps/documentation/places/web-service/migrate-autocomplete)

// =================================================================================================
//
// Requests

#[cfg(feature = "places-new-autocomplete")]
pub mod autocomplete;

#[cfg(feature = "places-new-nearby-search")]
pub mod nearby_search;

#[cfg(all(feature = "reqwest", feature = "places-new-place-details"))]
pub mod place_details;

#[cfg(feature = "places-new-place-photos")]
pub mod place_photos;

#[cfg(feature = "places-new-text-search")]
pub mod text_search;

// =================================================================================================
//
// Response

pub(crate) mod serde;

mod error;
pub use crate::places_new::error::Error;

mod types;

// -------------------------------------------------------------------------------------------------
//
// Structures

pub use crate::places_new::types::address_descriptor::AddressDescriptor;
pub use crate::places_new::types::address_descriptor::area::Area;
pub use crate::places_new::types::address_descriptor::landmark::Landmark;
pub use crate::places_new::types::author_attribution::AuthorAttribution;
pub use crate::places_new::types::content_block::ContentBlock;
pub use crate::places_new::types::date::Date;
pub use crate::places_new::types::error::Error as GoogleApiError;
pub use crate::places_new::types::ev_charging::ConnectorAggregation;
pub use crate::places_new::types::ev_charging::EvChargeOptions;
pub use crate::places_new::types::field_mask::FieldMask;
pub use crate::places_new::types::fuel_options::fuel_price::FuelPrice;
pub use crate::places_new::types::fuel_options::FuelOptions;
pub use crate::places_new::types::google_maps_links::GoogleMapsLinks;
pub use crate::places_new::types::lat_lng::LatLng;
pub use crate::places_new::types::localized_text::LocalizedText;
pub use crate::places_new::types::money::Money;
pub use crate::places_new::types::photo_info::PhotoInfo;
pub use crate::places_new::types::place::accessibility_options::AccessibilityOptions;
pub use crate::places_new::types::place::address_component::AddressComponent;
pub use crate::places_new::types::place::attribution::Attribution;
pub use crate::places_new::types::place::containing_place::ContainingPlace;
pub use crate::places_new::types::place::ev_charge_amenity_summary::EvChargeAmenitySummary;
pub use crate::places_new::types::place::generative_summary::GenerativeSummary;
pub use crate::places_new::types::place::neighborhood_summary::NeighborhoodSummary;
pub use crate::places_new::types::place::opening_hours::OpeningHours;
pub use crate::places_new::types::place::parking_options::ParkingOptions;
pub use crate::places_new::types::place::payment_options::PaymentOptions;
pub use crate::places_new::types::place::period::Period;
pub use crate::places_new::types::place::Place;
pub use crate::places_new::types::place::plus_code::PlusCode;
pub use crate::places_new::types::place::point::Point;
pub use crate::places_new::types::place::review_summary::ReviewSummary;
pub use crate::places_new::types::place::secondary_hours_type::SecondaryHoursType;
pub use crate::places_new::types::place::special_day::SpecialDay;
pub use crate::places_new::types::place::sub_destination::SubDestination;
pub use crate::places_new::types::place_type::PlaceType;
pub use crate::places_new::types::postal_address::PostalAddress;
pub use crate::places_new::types::price_range::PriceRange;
pub use crate::places_new::types::review::Review;
pub use crate::places_new::types::time_zone::TimeZone;
pub use crate::places_new::types::viewport::Viewport;

#[cfg(any(feature = "places-new-autocomplete", feature = "places-new-nearby-search", feature = "places-new-text-search"))]
pub use crate::places_new::types::request::Circle;
#[cfg(any(feature = "places-new-autocomplete", feature = "places-new-nearby-search"))]
pub use crate::places_new::types::request::PlaceTypeSet;
#[cfg(any(feature = "places-new-autocomplete", feature = "places-new-text-search"))]
pub use crate::places_new::types::request::LocationBias;
#[cfg(any(feature = "places-new-autocomplete", feature = "places-new-nearby-search", feature = "places-new-text-search"))]
pub use crate::places_new::types::request::LocationRestriction;
#[cfg(any(feature = "places-new-nearby-search", feature = "places-new-text-search"))]
pub use crate::places_new::types::request::RankPreference;

// -------------------------------------------------------------------------------------------------
//
// Enumerations

pub use crate::places_new::types::address_component_type::AddressComponentType;
pub use crate::places_new::types::address_descriptor::Containment;
pub use crate::places_new::types::address_descriptor::SpatialRelationship;
pub use crate::places_new::types::address_type::AddressType;
pub use crate::places_new::types::ev_charging::EvConnectorType;
pub use crate::places_new::types::field::Field;
pub use crate::places_new::types::fuel_options::fuel_type::FuelType;
pub use crate::places_new::types::place::business_status::BusinessStatus;
pub use crate::places_new::types::price_level::PriceLevel;