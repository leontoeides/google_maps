/// Specifies the assumptions to use when calculating time in traffic.
///
/// [Traffic Models](https://developers.google.com/maps/documentation/directions/intro#optional-parameters)
///
/// This setting affects the value returned in the `duration_in_traffic` field
/// in the response, which contains the predicted time in traffic based on
/// historical averages. The `traffic_model` parameter may only be specified for
/// driving directions where the request includes a `departure_time`, and only
/// if the request includes an API key or a Google Maps Platform Premium Plan
/// client ID.
///
/// The default value of `best_guess` will give the most useful predictions for
/// the vast majority of use cases. It is possible the `best_guess` travel time
/// prediction may be _shorter_ than `optimistic`, or alternatively, _longer_
/// than `pessimistic`, due to the way the `best_guess` prediction model
/// integrates live traffic information.

#[derive(Clone, Debug)]
pub enum TrafficModel {
    /// Use historical traffic data to best estimate the time spent in traffic.
    BestGuess,
    /// Use historical traffic data to make an optimistic estimate of what the
    /// duration in traffic will be.
    Optimistic,
    /// Use historical traffic data to make a pessimistic estimate of what the
    /// duration in traffic will be.
    Pessimistic,
} // enum

impl From<&TrafficModel> for String {
    /// Converts a `TrafficModel` enum to a `String` that contains a [traffic model](https://developers.google.com/maps/documentation/javascript/reference/directions#TrafficModel) code.
    fn from(traffic_model: &TrafficModel) -> String {
        match traffic_model {
            TrafficModel::BestGuess => String::from("best_guess"),
            TrafficModel::Pessimistic => String::from("pessimistic"),
            TrafficModel::Optimistic => String::from("optimistic"),
        } // match
    } // fn
} // impl

impl From<String> for TrafficModel {
    /// Gets a `TrafficModel` enum from a `String` that contains a valid [traffic model](https://developers.google.com/maps/documentation/javascript/reference/directions#TrafficModel) code.
    fn from(traffic_model: String) -> TrafficModel {
        match traffic_model.as_ref() {
            "best_guess" => TrafficModel::BestGuess,
            "pessimistic" => TrafficModel::Pessimistic,
            "optimistic" => TrafficModel::Optimistic,
            _ => panic!("'{}' is not a valid traffic model code. Valid codes are 'best_guess', 'pessimistic', and 'optimistic'.", traffic_model),
        } // match
    } // fn
} // impl