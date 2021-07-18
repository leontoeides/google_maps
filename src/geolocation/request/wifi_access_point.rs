use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct WiFiAccessPoint {
    // Required parameters:
    // --------------------

    /// (required) The MAC address of the WiFi node. It's typically called a
    /// BSS, BSSID or MAC address. Separators must be `:` (colon).
    #[serde(alias = "macAddress")]
    pub mac_address: String,

    // Optional parameters:
    // --------------------

    /// The current signal strength measured in dBm.
    #[serde(alias = "signalStrength")]
    pub signal_strength: Option<i16>,

    /// The number of milliseconds since this access point was detected.
    #[serde(alias = "age")]
    pub age: Option<u16>,

    /// The channel over which the client is communicating with the access point.
    #[serde(alias = "channel")]
    pub channel: Option<u16>,

    /// The current signal to noise ratio measured in dB.
    #[serde(alias = "signalToNoiseRatio")]
    pub signal_to_noise_ratio: Option<i16>,
} // struct