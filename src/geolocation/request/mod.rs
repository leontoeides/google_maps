mod cell_tower;
mod radio_type;
mod wifi_access_point;

use crate::geolocation::request::cell_tower::CellTower;
use crate::geolocation::request::radio_type::RadioType;
use crate::geolocation::request::wifi_access_point::WiFiAccessPoint;

/// The request body must be formatted as JSON. All fields are optional.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Request {
    // Required parameters:
    // --------------------

    /// Your application's API key. This key identifies your application for
    /// purposes of quota management. Learn how to
    /// [get a key](https://developers.google.com/maps/documentation/timezone/get-api-key).
    key: String,

    // Optional parameters:
    // --------------------

    /// The carrier name.
    pub carrier: Option<String>,

    /// An array of cell tower objects. See the [Cell Tower
    /// Objects](https://developers.google.com/maps/documentation/geolocation/intro#cell_tower_object)
    /// section.
    pub cell_towers: Vec<CellTower>,

    /// Specifies whether to fall back to IP geolocation if wifi and cell tower
    /// signals are not available. Defaults to `true`. Set `considerIp` to
    /// `false` to disable fall back.
    pub consider_ip: Option<bool>,

    /// The mobile country code (MCC) for the device's home network.
    pub home_mobile_country_code: Option<u16>,

    /// The mobile network code (MNC) for the device's home network.
    pub home_mobile_network_code: Option<u16>,

    /// The mobile radio type. Supported values are `lte`, `gsm`, `cdma`, and
    /// `wcdma`. While this field is optional, it should be included if a value
    /// is available, for more accurate results.
    pub radio_type: Option<RadioType>,

    /// An array of WiFi access point objects. See the [WiFi Access Point
    /// Objects](https://developers.google.com/maps/documentation/geolocation/intro#wifi_access_point_object)
    /// section.
    pub wifi_access_points: Vec<WiFiAccessPoint>,

    // Internal use only:
    // ------------------

    /// Request body that is to be submitted to the Google Cloud Maps Platform.
    body: Option<String>,

    /// Query string that is to be submitted to the Google Cloud Maps Platform.
    query: Option<String>,

    /// Has the request been validated?
    validated: bool,
} // struct