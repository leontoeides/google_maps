//! Contains the type of vehicle used on this line.

use crate::directions::response::{transit_agency::TransitAgency, transit_vehicle::TransitVehicle}; // use
use serde::{Deserialize, Serialize};

/// Contains the type of vehicle used on this line.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct TransitLine {
    /// Contains the full name of this transit line. eg. "7 Avenue Express"
    pub name: Option<String>,
    /// Contains the short name of this transit line. This will normally be a
    /// line number, such as "M7" or "355".
    pub short_name: Option<String>,
    /// Contains the color commonly used in signage for this transit line. The
    /// color will be specified as a hex string such as: #FF0033.
    pub color: Option<String>,
    /// An array containing a single `TransitAgency` object. The `TransitAgency`
    /// object provides information about the operator of the line
    #[serde(default)]
    pub agencies: Vec<TransitAgency>,
    /// Contains the URL for this transit line as provided by the transit
    /// agency.
    pub url: Option<String>,
    /// Contains the URL for the icon associated with this line.
    pub icon: Option<String>,
    /// Contains the color of text commonly used for signage of this line. The
    /// color will be specified as a hex string.
    pub text_color: Option<String>,
    // Contains the type of vehicle used on this line.
    pub vehicle: TransitVehicle,
} // struct
