//! A `LatLng` utility function that has been split off into its own file
//! because its not a trait.

/// Converts a `LatLng` an `f64` containing latitude or longitude into a
/// `String` containing a decimal degrees coordinate. This function contrains
/// the floating-point number to seventh decimal place precision, and removes
/// insignificant digits to save bandwidth & improve readability.

pub fn f64_to_decdeg(float: f64) -> String {

    // The seventh decimal place is worth up to 11 mm: this is good for much
    // surveying and is near the limit of what GPS-based techniques can
    // achieve:
    // https://gis.stackexchange.com/questions/8650/measuring-accuracy-of-latitude-and-longitude

    let mut decdeg_str = format!("{:.7}", float);

    // Remove insignificant digits and possibly the fractional portion of the
    // string altogether:

    if decdeg_str.contains('.') {
        decdeg_str = decdeg_str.trim_end_matches('0').to_string();
        decdeg_str = decdeg_str.trim_end_matches('.').to_string();
    } // if

    // Return formatted latitude or longitude in decimal degrees:
    decdeg_str

} // fn