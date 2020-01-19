use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ManeuverType {
    #[serde(alias = "ferry")]
    Ferry,
    #[serde(alias = "ferry-train")]
    FerryTrain,
    #[serde(alias = "fork-left")]
    ForkLeft,
    #[serde(alias = "fork-right")]
    ForkRight,
    #[serde(alias = "keep-left")]
    KeepLeft,
    #[serde(alias = "keep-right")]
    KeepRight,
    #[serde(alias = "merge")]
    Merge,
    #[serde(alias = "ramp-left")]
    RampLeft,
    #[serde(alias = "ramp-right")]
    RampRight,
    #[serde(alias = "roundabout-left")]
    RoundaboutLeft,
    #[serde(alias = "roundabout-right")]
    RoundaboutRight,
    #[serde(alias = "straight")]
    Straight,
    #[serde(alias = "turn-left")]
    TurnLeft,
    #[serde(alias = "turn-right")]
    TurnRight,
    #[serde(alias = "turn-sharp-left")]
    TurnSharpLeft,
    #[serde(alias = "turn-sharp-right")]
    TurnSharpRight,
    #[serde(alias = "turn-slight-left")]
    TurnSlightLeft,
    #[serde(alias = "turn-slight-right")]
    TurnSlightRight,
    #[serde(alias = "uturn-left")]
    UturnLeft,
    #[serde(alias = "uturn-right")]
    UturnRight,
} // enum