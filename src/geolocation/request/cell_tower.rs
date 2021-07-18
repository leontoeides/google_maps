use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct CellTower {
    // Required parameters:
    // --------------------

    /// Unique identifier of the cell. On GSM, this is the Cell ID (CID); CDMA
    /// networks use the Base Station ID (BID). WCDMA networks use the
    /// UTRAN/GERAN Cell Identity (UC-Id), which is a 32-bit value concatenating
    /// the Radio Network Controller (RNC) and Cell ID. Specifying only the
    /// 16-bit Cell ID value in WCDMA networks may return inaccurate results.
    #[serde(alias = "cellId")]
    pub cell_id: u32,

    /// The Location Area Code (LAC) for GSM and WCDMA networks. The Network ID
    /// (NID) for CDMA networks.
    #[serde(alias = "locationAreaCode")]
    pub location_area_code: u16,

    /// The cell tower's Mobile Country Code (MCC).
    #[serde(alias = "mobileCountryCode")]
    pub mobile_country_code: u16,

    /// The cell tower's Mobile Network Code. This is the MNC for GSM and WCDMA;
    /// CDMA uses the System ID (SID).
    #[serde(alias = "mobileNetworkCode")]
    pub mobile_network_code: u16,

    // Optional parameters:
    // --------------------

    /// The number of milliseconds since this cell was primary. If age is 0, the
    /// `cellId` represents a current measurement.
    pub age: Option<u16>,

    /// Radio signal strength measured in dBm.
    #[serde(alias = "signalStrength")]
    pub signal_strength: Option<i16>,

    /// The [timing advance](https://en.wikipedia.org/wiki/Timing_advance)
    /// value.
    #[serde(alias = "timingAdvance")]
    pub timing_advance: Option<u16>,
} // struct