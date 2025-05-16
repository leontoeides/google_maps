use crate::address_validation::UspsAddress;
use getset::{CopyGetters, Getters, MutGetters, Setters};
use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// The USPS data for the address. `uspsData` is not guaranteed to be fully
/// populated for every US or PR address sent to the Address Validation API.
/// It's recommended to integrate the backup address fields in the response if
/// you utilize uspsData as the primary part of the response.
///
/// This property provides useful information for United States postal
/// addresses. However, it's not guaranteed to be fully populated for every
/// address validated by the service. For that reason, you shouldn't rely on
/// this property as the sole means to validate addresses, but instead check the
/// verdict and address as well.
///
/// ## Key Point
///
/// * Use the `uspsData` property to obtain delivery confidence levels and other
///   USPS details about US addresses.
#[allow(clippy::struct_excessive_bools, clippy::doc_markdown)]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize, CopyGetters, Getters, MutGetters, Setters)]
#[serde(rename_all = "camelCase")]
pub struct UspsData {
    /// USPS standardized address.
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub standardized_address: UspsAddress,

    /// 2 digit delivery point code
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub delivery_point_code: Option<String>,

    /// The delivery point check digit. This number is added to the end of the
    /// `delivery_point_barcode` for mechanically scanned mail. Adding all the
    /// digits of the `delivery_point_barcode`, `deliveryPointCheckDigit`,
    /// postal code, and ZIP+4 together should yield a number divisible by 10.
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub delivery_point_check_digit: Option<String>,

    /// The possible values for DPV confirmation. Returns a single character or
    /// returns no value.
    ///
    /// * `N`: Primary and any secondary number information failed to DPV
    ///   confirm.
    ///
    /// * `D`: Address was DPV confirmed for the primary number only, and the
    ///   secondary number information was missing.
    ///
    /// * `S`: Address was DPV confirmed for the primary number only, and the
    ///   secondary number information was present but not confirmed.
    ///
    /// * `Y`: Address was DPV confirmed for primary and any secondary numbers.
    ///
    /// * Empty: If the response does not contain a dpvConfirmation value, the
    ///   address was not submitted for DPV confirmation.
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub dpv_confirmation: Option<char>,

    /// The footnotes from delivery point validation. Multiple footnotes may be
    /// strung together in the same string.
    ///
    /// * `AA`: Input address matched to the ZIP+4 file
    /// * `A1`: Input address was not matched to the ZIP+4 file
    /// * `BB`: Matched to DPV (all components)
    /// * `CC`: Secondary number not matched and not required
    /// * `C1`: Secondary number not matched but required
    /// * `N1`: High-rise address missing secondary number
    /// * `M1`: Primary number missing
    /// * `M3`: Primary number invalid
    /// * `P1`: Input address PO, RR or HC box number missing
    /// * `P3`: Input address PO, RR, or HC Box number invalid
    /// * `F1`: Input address matched to a military address
    /// * `G1`: Input address matched to a general delivery address
    /// * `U1`: Input address matched to a unique ZIP code
    /// * `PB`: Input address matched to PBSA record
    /// * `RR`: DPV confirmed address with PMB information
    /// * `R1`: DPV confirmed address without PMB information
    /// * `R7`: Carrier Route R777 or R779 record
    /// * `IA`: Informed Address identified
    /// * `TA`: Primary number matched by dropping a trailing alpha
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub dpv_footnote: String,

    /// Indicates if the address is a CMRA (Commercial Mail Receiving Agency)--a
    /// private business receiving mail for clients. Returns a single character.
    ///
    /// * `Y`: The address is a CMRA
    /// * `N`: The address is not a CMRA
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub dpv_cmra: Option<char>,

    /// Is this place vacant? Returns a single character.
    ///
    /// * `Y`: The address is vacant
    /// * `N`: The address is not vacant
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub dpv_vacant: Option<char>,

    /// Is this a no stat address or an active address? No stat addresses are
    /// ones which are not continuously occupied or addresses that the USPS does
    /// not service. Returns a single character.
    ///
    /// * `Y`: The address is not active
    /// * `N`: The address is active
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub dpv_no_stat: Option<char>,

    /// Indicates the NoStat type. Returns a reason code as int.
    ///
    /// * `1`: IDA (Internal Drop Address) – Addresses that do not receive mail
    ///   directly from the USPS but are delivered to a drop address that
    ///   services them.
    ///
    /// * `2`: CDS - Addresses that have not yet become deliverable. For
    ///   example, a new subdivision where lots and primary numbers have been
    ///   determined, but no structure exists yet for occupancy.
    ///
    /// * `3`: Collision - Addresses that do not actually DPV confirm.
    ///
    /// * `4`: CMZ (College, Military and Other Types) - ZIP + 4 records USPS
    ///   has incorporated into the data.
    ///
    /// * `5`: Regular - Indicates addresses not receiving delivery and the
    ///   addresses are not counted as possible deliveries.
    ///
    /// * `6`: Secondary Required - The address requires secondary information.
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub dpv_no_stat_reason_code: Option<u8>,

    /// Flag indicates mail is delivered to a single receptable at a site.
    /// Returns a single character.
    ///
    /// * `Y`: The mail is delivered to a single receptable at a site.
    /// * `N`: The mail is not delivered to a single receptable at a site.
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub dpv_drop: Option<char>,


    /// Indicates that mail is not delivered to the street address. Returns a
    /// single character.
    ///
    /// * `Y`: The mail is not delivered to the street address.
    /// * `N`: The mail is delivered to the street address.
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub dpv_throwback: Option<char>,

    /// Flag indicates mail delivery is not performed every day of the week.
    /// Returns a single character.
    ///
    /// * `Y`: The mail delivery is not performed every day of the week.
    /// * `N`: No indication the mail delivery is not performed every day of the
    ///   week.
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub dpv_non_delivery_days: Option<char>,

    /// Integer identifying non-delivery days. It can be interrogated using bit
    /// flags:
    ///
    /// * `0x40` – Sunday is a non-delivery day
    /// * `0x20` – Monday is a non-delivery day
    /// * `0x10` – Tuesday is a non-delivery day
    /// * `0x08` – Wednesday is a non-delivery day
    /// * `0x04` – Thursday is a non-delivery day
    /// * `0x02` – Friday is a non-delivery day
    /// * `0x01` – Saturday is a non-delivery day
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub dpv_non_delivery_days_values: Option<u8>,

    /// Flag indicates door is accessible, but package will not be left due to
    /// security concerns. Returns a single character.
    ///
    /// * `Y`: The package will not be left due to security concerns.
    /// * `N`: No indication the package will not be left due to security
    ///   concerns.
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub dpv_no_secure_location: Option<char>,

    /// Indicates the address was matched to PBSA record. Returns a single
    /// character.
    ///
    /// * `Y`: The address was matched to PBSA record.
    /// * `N`: The address was not matched to PBSA record.
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub dpv_pbsa: Option<char>,

    /// Flag indicates addresses where USPS cannot knock on a door to deliver
    /// mail. Returns a single character.
    ///
    /// * `Y`: The door is not accessible.
    /// * `N`: No indication the door is not accessible.
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub dpv_door_not_accessible: Option<char>,

    /// Indicates that more than one DPV return code is valid for the address.
    /// Returns a single character.
    ///
    /// * `Y`: Address was DPV confirmed for primary and any secondary numbers.
    ///
    /// * `N`: Primary and any secondary number information failed to DPV
    ///   confirm.
    ///
    /// * `S`: Address was DPV confirmed for the primary number only, and the
    ///   secondary number information was present but not confirmed, or a
    ///   single trailing alpha on a primary number was dropped to make a DPV
    ///   match and secondary information required.
    ///
    /// * `D`: Address was DPV confirmed for the primary number only, and the
    ///   secondary number information was missing.
    ///
    /// * `R`: Address confirmed but assigned to phantom route R777 and R779 and
    ///   USPS delivery is not provided.
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub dpv_enhanced_delivery_code: Option<char>,

    /// The carrier route code. A four character code consisting of a one letter
    /// prefix and a three digit route designator.
    ///
    /// Prefixes:
    ///
    /// * `C`: Carrier route (or city route)
    /// * `R`: Rural route
    /// * `H`: Highway Contract Route
    /// * `B`: Post Office Box Section
    /// * `G`: General delivery unit
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub carrier_route: Option<String>,

    /// Carrier route rate sort indicator.
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub carrier_route_indicator: Option<String>,

    /// The delivery address is matchable, but the EWS file indicates that an
    /// exact match will be available soon.
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub ews_no_match: Option<bool>,

    /// Main post office city.
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub post_office_city: String,

    /// Main post office state.
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub post_office_state: String,

    /// Abbreviated city.
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub abbreviated_city: Option<String>,

    /// FIPS county code.
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub fips_county_code: Option<String>,

    /// County name.
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub county: Option<String>,

    /// Enhanced Line of Travel (eLOT) number.
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub elot_number: Option<String>,

    /// eLOT Ascending/Descending Flag (A/D).
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub elot_flag: Option<String>,

    /// LACSLink return code.
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub lacs_link_return_code: Option<String>,

    /// LACSLink indicator.
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub lacs_link_indicator: Option<String>,

    /// PO Box only postal code.
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub po_box_only_postal_code: Option<bool>,

    /// Footnotes from matching a street or highrise record to suite
    /// information. If business name match is found, the secondary number is
    /// returned.
    ///
    /// * `A`: SuiteLink record match, business address improved.
    /// * `00`: No match, business address is not improved.
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub suitelink_footnote: Option<String>,

    /// PMB (Private Mail Box) unit designator.
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub pmb_designator: Option<String>,

    /// PMB (Private Mail Box) number.
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub pmb_number: Option<String>,

    /// Type of the address record that matches the input address.
    ///
    /// * `F`: FIRM. This is a match to a Firm Record, which is the finest level
    ///   of match available for an address.
    ///
    /// * `G`: GENERAL DELIVERY. This is a match to a General Delivery record.
    ///
    /// * `H`: BUILDING / APARTMENT. This is a match to a Building or Apartment
    ///   record.
    ///
    /// * `P`: POST OFFICE BOX. This is a match to a Post Office Box.
    ///
    /// * `R`: RURAL ROUTE or HIGHWAY CONTRACT: This is a match to either a
    ///   Rural Route or a Highway Contract record, both of which may have
    ///   associated Box Number ranges.
    ///
    /// * `S`: STREET RECORD: This is a match to a Street record containing a
    ///   valid primary number range.
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub address_record_type: Option<String>,

    /// Indicator that a default address was found, but more specific addresses
    /// exists.
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub default_address: Option<bool>,

    /// Error message for USPS data retrieval. This is populated when USPS
    /// processing is suspended because of the detection of artificially created
    /// addresses.
    ///
    /// The USPS data fields might not be populated when this error is present.
    #[serde(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    pub error_message: Option<String>,

    /// Indicator that the request has been CASS processed.
    #[serde(default)]
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    pub cass_processed: Option<bool>,
} // struct UspsData