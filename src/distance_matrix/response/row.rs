use crate::distance_matrix::response::element::Element;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Row {

    pub elements: Vec<Element>,

} // struct