use crate::directions::vehicle_type::VehicleType;
use serde::{Serialize, Deserialize};

/// Contains the type of vehicle used on this line.

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransitVehicle {

    /// Contains the URL for an icon associated with this vehicle type.
    pub icon: String,

    /// Contains the URL for the icon associated with this vehicle type, based
    /// on the local transport signage.
    pub local_icon: String,

    /// Contains the name of the vehicle on this line. eg. "Subway."
    pub name: String,

    /// Contains the type of vehicle that runs on this line. See the [Vehicle
    /// Type](https://developers.google.com/maps/documentation/directions/intro#VehicleType)
    /// documentation for a complete list of supported values.
    #[serde(alias = "type")]
    pub vehicle_type: VehicleType,

} // struct