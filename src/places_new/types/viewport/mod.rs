//! Contains the `Viewport` struct and its associated traits.
//!
//! A viewport represents a rectangular geographic area defined by two diagonally opposite corners
//! (low and high). Used to specify map display bounds and geographic search areas in Google Maps
//! APIs.

#[cfg(feature = "geo")]
mod geo;

use crate::places_new::{Error, LatLng};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

// -------------------------------------------------------------------------------------------------
//
/// Represents a latitude-longitude viewport as a rectangular geographic area.
///
/// Defined by two diagonally opposite points: a low (southwest) corner and a high (northeast)
/// corner. The viewport is a closed region that includes its boundaries. Used to specify map
/// display bounds, search areas, and geographic constraints in Google Maps APIs.
///
/// Both `low` and `high` points must be populated, and the viewport cannot be empty according to
/// Google's validation rules.
///
/// # Special Cases
///
/// * If `low` == `high`, the viewport consists of a single point
/// * If `low.longitude` > `high.longitude`, the viewport crosses the 180° meridian
/// * If `low.longitude` == -180° and `high.longitude` == 180°, includes all longitudes
/// * If `low.longitude` == 180° and `high.longitude` == -180°, the longitude range is empty
/// * If `low.latitude` > `high.latitude`, the latitude range is empty
#[derive(
    //std
    Clone,
    Debug,
    Eq,
    PartialEq,
    // serde
    serde::Deserialize,
    serde::Serialize,
    // getset
    getset::Getters,
    getset::CopyGetters,
    getset::MutGetters,
    getset::Setters,
)]
pub struct Viewport {
    /// The low (southwest) point of the viewport.
    ///
    /// This represents the minimum latitude and minimum longitude boundary of the viewport,
    /// typically the southwest corner in standard orientation.
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub low: LatLng,
    
    /// The high (northeast) point of the viewport.
    ///
    /// This represents the maximum latitude and maximum longitude boundary of the viewport,
    /// typically the northeast corner in standard orientation.
    #[getset(get = "pub", set = "pub", get_mut = "pub")]
    pub high: LatLng,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Viewport {
    /// Creates a new `Viewport` from low and high corner points with validation.
    ///
    /// Validates that the viewport is not empty according to Google Maps API rules.
    ///
    /// A viewport is considered empty if the latitude range is inverted (low > high) or if the
    /// longitude range is empty (low == 180° and high == -180°). Use this constructor when defining
    /// map display bounds or search areas.
    ///
    /// # Errors
    ///
    /// Returns an error if the viewport would be invalid.
    pub fn try_new(low: LatLng, high: LatLng) -> Result<Self, Error> {
        let lat_empty = low.latitude > high.latitude;
        let lng_empty = low.longitude == dec!(180.0) && high.longitude == dec!(-180.0);
        
        if lat_empty || lng_empty {
            let debug = format!(
                "low: ({}, {}), high: ({}, {})",
                low.latitude,
                low.longitude,
                high.latitude,
                high.longitude
            );

            let span = (0, debug.len());

            Err(Error::InvalidViewport {
                low,
                high,
                debug,
                span,
            })
        } else {
            Ok(Self { low, high })
        }
    }
    
    /// Creates a viewport from coordinate values with validation.
    ///
    /// Convenience constructor that takes individual latitude and longitude values for both
    /// corners.
    ///
    /// Use this when you have raw coordinate values from calculations or API responses.
    ///
    /// # Errors
    ///
    /// Returns an error if any coordinate is out of valid range or if the resulting viewport would
    /// be empty.
    pub fn try_from_coords(
        low_lat: Decimal,
        low_lng: Decimal,
        high_lat: Decimal,
        high_lng: Decimal,
    ) -> Result<Self, Error> {
        let low = LatLng::try_from_dec(low_lat, low_lng)?;
        let high = LatLng::try_from_dec(high_lat, high_lng)?;
        Self::try_new(low, high)
    }

    /// Returns both corners as a tuple `(low, high)`.
    ///
    /// Use this when you need to work with both corner points simultaneously or when destructuring
    /// the viewport bounds.
    #[must_use]
    pub const fn bounds(&self) -> (&LatLng, &LatLng) {
        (&self.low, &self.high)
    }
    
    /// Returns the center point of the viewport.
    ///
    /// Calculates the geographic center by averaging the latitude and longitude coordinates of the
    /// two corners. Note that for viewports crossing the 180° meridian, special handling may be
    /// needed (not currently implemented).
    ///
    /// Use this to find the focal point for map centering or distance calculations.
    #[must_use]
    #[allow(clippy::missing_panics_doc, reason = "panic not possible")]
    pub fn center(&self) -> LatLng {
        let center_lat = (self.low.latitude + self.high.latitude) / dec!(2.0);
        let center_lng = (self.low.longitude + self.high.longitude) / dec!(2.0);
        
        // Note: unwrap is safe here because center coordinates are guaranteed to be within valid
        // ranges if the corners are valid.
        LatLng::try_from_dec(center_lat, center_lng)
            .expect("center coordinates should be valid if corners are valid")
    }
    
    /// Checks if this viewport contains a given point.
    ///
    /// Returns `true` if the point falls within or on the boundaries of the viewport. Handles
    /// standard cases but does not currently handle viewports that cross the 180° meridian.
    ///
    /// Use this for geographic containment checks or filtering locations by bounds.
    #[must_use]
    pub fn contains(&self, point: &LatLng) -> bool {
        let lat_in_range = point.latitude >= self.low.latitude
            && point.latitude <= self.high.latitude;

        let lng_in_range = point.longitude >= self.low.longitude
            && point.longitude <= self.high.longitude;

        lat_in_range && lng_in_range
    }
    
    /// Checks if this viewport crosses the 180° meridian (antimeridian).
    ///
    /// Returns `true` if the viewport wraps around the international date line, which occurs when
    /// the low longitude is greater than the high longitude.
    ///
    /// This is important for correctly handling global viewports and coordinate wrapping in map
    /// displays.
    #[must_use]
    pub fn crosses_antimeridian(&self) -> bool {
        self.low.longitude > self.high.longitude
    }

    /// Formats viewport as a human-readable string.
    ///
    /// Converts to an absolute value format with N/S/E/W indicators, making coordinates easier to
    /// read and understand.
    ///
    /// For example, returns `37.7749° N 122.4194° W` instead of `37.7749,-122.4194`. Use this
    /// when displaying coordinates to end users in UI elements.
    #[must_use]
    pub fn display(&self) -> String {
        format!(
            "[{} to {}]",
            self.low.display(),
            self.high.display()
        )
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl TryFrom<(f64, f64, f64, f64)> for Viewport {
    type Error = Error;

    /// Creates a viewport from a tuple of `(low_lat, low_lng, high_lat, high_lng)`.
    ///
    /// Convenience conversion that allows creating a viewport from raw f64 coordinates without
    /// explicitly constructing `LatLng` points first.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let viewport = Viewport::try_from((37.0, -122.5, 38.0, -122.0))?;
    /// ```
    fn try_from((low_lat, low_lng, high_lat, high_lng): (f64, f64, f64, f64)) -> Result<Self, Self::Error> {
        let low = LatLng::try_from((low_lat, low_lng))?;
        let high = LatLng::try_from((high_lat, high_lng))?;
        Self::try_new(low, high)
    }
}

impl TryFrom<(f32, f32, f32, f32)> for Viewport {
    type Error = Error;

    /// Creates a viewport from a tuple of `(low_lat, low_lng, high_lat, high_lng)`.
    ///
    /// Convenience conversion using f32 precision. Useful when working with lower-precision data or
    /// memory-constrained environments.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let viewport = Viewport::try_from((37.0f32, -122.5f32, 38.0f32, -122.0f32))?;
    /// ```
    fn try_from((low_lat, low_lng, high_lat, high_lng): (f32, f32, f32, f32)) -> Result<Self, Self::Error> {
        let low = LatLng::try_from((low_lat, low_lng))?;
        let high = LatLng::try_from((high_lat, high_lng))?;
        Self::try_new(low, high)
    }
}

impl TryFrom<(Decimal, Decimal, Decimal, Decimal)> for Viewport {
    type Error = Error;

    /// Creates a viewport from a tuple of `(low_lat, low_lng, high_lat, high_lng)`.
    ///
    /// Highest precision conversion using `Decimal` types throughout. Preferred when working
    /// directly with decimal values to avoid floating-point conversions.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let viewport = Viewport::try_from((
    ///     dec!(37.0), dec!(-122.5),
    ///     dec!(38.0), dec!(-122.0)
    /// ))?;
    /// ```
    fn try_from((low_lat, low_lng, high_lat, high_lng): (Decimal, Decimal, Decimal, Decimal)) -> Result<Self, Self::Error> {
        Self::try_from_coords(low_lat, low_lng, high_lat, high_lng)
    }
}

impl std::fmt::Display for Viewport {
    /// Formats the viewport as axis-aligned bounding box `[xmin, ymin, xmax, ymax]` Treats it as a
    /// 2D rectangle in Cartesian space `[-122.5, 37.7, -122.4, 37.8]`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}, {}, {}, {}]",
            self.low.longitude,
            self.low.latitude,
            self.high.longitude,
            self.high.latitude
        )
    }
}

impl std::default::Default for Viewport {
    /// Returns a default viewport representing a single point at Null Island.
    ///
    /// Creates a degenerate viewport where both corners are at (0°N, 0°E). This default is useful
    /// for initialization but typically shouldn't represent real-world map bounds. Consider using
    /// `try_new` with meaningful bounds for actual use cases.
    fn default() -> Self {
        Self {
            low: LatLng::default(),
            high: LatLng::default(),
        }
    }
}

#[cfg(any(
    feature = "directions",
    feature = "distance_matrix",
    feature = "geocoding",
    feature = "places"
))]
impl std::convert::From<crate::Bounds> for Viewport {
    /// Converts an owned `crate::Bounds` struct to a `crate::places_new::Viewport` struct.
    fn from(bounds: crate::Bounds) -> Self {
        Self {
            low: bounds.southwest.into(),
            high: bounds.northeast.into()
        }
    }
}

#[cfg(any(
    feature = "directions",
    feature = "distance_matrix",
    feature = "geocoding",
    feature = "places"
))]
impl std::convert::From<&crate::Bounds> for Viewport {
    /// Converts a borrowed `&crate::Bounds` struct to a `crate::places_new::Viewport` struct.
    fn from(bounds: &crate::Bounds) -> Self {
        Self {
            low: bounds.southwest.into(),
            high: bounds.northeast.into()
        }
    }
}

#[cfg(any(
    feature = "directions",
    feature = "distance_matrix",
    feature = "geocoding",
    feature = "places"
))]
impl std::convert::From<Viewport> for crate::Bounds {
    /// Converts an owned `crate::places_new::Viewport` struct to a `crate::Bounds` struct.
    fn from(viewport: Viewport) -> Self {
        Self {
            southwest: viewport.low.into(),
            northeast: viewport.high.into(),
        }
    }
}

#[cfg(any(
    feature = "directions",
    feature = "distance_matrix",
    feature = "geocoding",
    feature = "places"
))]
impl std::convert::From<&Viewport> for crate::Bounds {
    /// Converts a borrowed `&crate::places_new::Viewport` struct to a `crate::Bounds` struct.
    fn from(viewport: &Viewport) -> Self {
        Self {
            southwest: viewport.low.into(),
            northeast: viewport.high.into(),
        }
    }
}