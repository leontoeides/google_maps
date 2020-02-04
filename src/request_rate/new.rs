use crate::request_rate::RequestRate;

impl RequestRate {

    pub fn new() -> RequestRate {
        RequestRate {
            all: None,
            directions: None,
            distance_matrix: None,
            elevation: None,
            geocoding: None,
            time_zone: None,
        } // RequestRate
    } // fn

} // impl