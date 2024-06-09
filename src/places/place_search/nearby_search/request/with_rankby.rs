use crate::places::place_search::nearby_search::request::Request;
use crate::places::RankBy;

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {
    /// Adds the rank-by order parameter to the Places API _Nearby Search_
    /// query.
    ///
    /// ## Arguments
    ///
    /// * `rankby` ‧ Specifies the order in which results are listed. Possible
    ///   values are:
    ///
    /// * `prominence` (default). This option sorts results based on their
    ///   importance. Ranking will favor prominent places within the set radius
    ///   over nearby places that match but that are less prominent. Prominence
    ///   can be affected by a place's ranking in Google's index, global
    ///   popularity, and other factors. When prominence is specified, the
    ///   `radius` parameter is required.
    ///
    /// * `distance`. This option biases search results in ascending order by
    ///   their distance from the specified location. When `distance` is
    ///   specified, one or more of `keyword`, `name`, or `type` is required and
    ///   radius is disallowed.

    pub fn with_rankby(
        &'a mut self,
        rankby: impl Into<RankBy>
    ) -> &'a mut Self {
        // Set rannk by order in Request struct.
        self.rankby = Some(rankby.into());
        // Return modified Request struct to caller.
        self
    } // fn
} // impl
