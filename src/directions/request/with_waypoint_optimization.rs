use crate::directions::request::Request;

impl<'a> Request<'a> {
    /// Specifies whether the waypoint order should be optimized or not.
    ///
    /// ## Arguments
    ///
    /// * `waypoint_optimization` ‧ Specifies whether the waypoints should be
    ///   rearranged into the most time-efficient order or not.
    ///
    /// ## Description
    ///
    /// [Optimize your waypoints](https://developers.google.com/maps/documentation/directions/intro#OptimizeWaypoints)
    /// ----------------------------------------------------------------------------------------
    ///
    /// By default, the Directions service calculates a route through the
    /// provided waypoints in their given order. Optionally, you may pass
    /// `optimize:true` as the first argument within the `waypoints` parameter
    /// to allow the Directions service to optimize the provided route by
    /// rearranging the waypoints in a more efficient order. (This optimization
    /// is an application of the [traveling salesperson
    /// problem](https://en.wikipedia.org/wiki/Travelling_salesman_problem).)
    /// Travel time is the primary factor which is optimized, but other factors
    /// such as distance, number of turns and many more may be taken into
    /// account when deciding which route is the most efficient. All waypoints
    /// must be stopovers for the Directions service to optimize their route.
    ///
    /// If you instruct the Directions service to optimize the order of its
    /// waypoints, their order will be returned in the `waypoint_order` field
    /// within the
    /// `[routes](https://developers.google.com/maps/documentation/directions/intro#Routes)`
    /// object. The `waypoint_order` field returns values which are zero-based.
    ///
    /// The following example calculates a road journey from Adelaide, South
    /// Australia to each of South Australia's main wine regions using route
    /// optimization.
    ///
    /// ## Example
    ///
    /// ```rust
    /// .with_waypoint_optimization(true)
    /// ```

    pub fn with_waypoint_optimization(
        &'a mut self,
        waypoint_optimization: impl Into<bool>
    ) -> &'a mut Self {
        self.waypoint_optimization = waypoint_optimization.into();
        self
    } // fn
} // impl
