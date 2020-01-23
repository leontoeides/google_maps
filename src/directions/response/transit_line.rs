use crate::directions::response::transit_agency::TransitAgency;
use serde::{Serialize, Deserialize};

/// Contains the type of vehicle used on this line.

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransitLine {

    /// Contains the full name of this transit line. eg. "7 Avenue Express"
    pub name: Option<String>,

    /// Contains the short name of this transit line. This will normally be a
    /// line number, such as "M7" or "355".
    pub short_name: String,

    /// Contains the color commonly used in signage for this transit line. The
    /// color will be specified as a hex string such as: #FF0033.
    pub color: Option<String>,

    /// An array containing a single `TransitAgency` object. The `TransitAgency`
    /// object provides information about the operator of the line
    pub agencies: Vec<TransitAgency>,

} // struct