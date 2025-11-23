// -------------------------------------------------------------------------------------------------
//
/// A photo response that's returned by the Place Photos (New) service. It contains the raw image
/// data.
///
/// This is not typically used by end-users. It's an intermediary type that's used to build a
/// `PhotoData` type.
///
/// > 🛑 Caution: You cannot cache a photo name. Ensure you always get
/// > the name from a response to a request to
/// > [Place Details (New)](https://developers.google.com/maps/documentation/places/web-service/place-details),
/// > [Nearby Search (New)](https://developers.google.com/maps/documentation/places/web-service/nearby-search),
/// > or [Text Search (New)](https://developers.google.com/maps/documentation/places/web-service/text-search).
/// > For more info, see the caching restrictions in Section 3.2.3(b)(No Caching) of the
/// > [Google Maps Platform Terms of Service](https://cloud.google.com/maps-platform/terms).
#[derive(
    //std
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    // getset
    getset::Getters,
    getset::CopyGetters,
    // serde
    serde::Deserialize,
    serde::Serialize
)]
pub struct GooglePhotoData(Vec<u8>);