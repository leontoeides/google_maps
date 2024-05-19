use crate::directions::request::{avoid::Avoid, Request};

impl<'a> Request<'a> {
    /// Specify a feature that routes should avoid.
    ///
    /// ## Arguments
    ///
    /// * `restrictions` ‧ A Vec containing a list of features that should be
    /// avoided when possible when calculating the route, such as _ferries_,
    /// _highways_, _indoor_ steps, and/or _tolls_.
    ///
    /// ## Description
    ///
    /// Indicates that the calculated route(s) should avoid the indicated
    /// features. This parameter supports the following arguments:
    ///
    /// * `Avoid::Tolls` indicates that the calculated route should avoid toll
    /// roads/bridges.
    ///
    /// * `Avoid::Highways` indicates that the calculated route should avoid
    /// highways.
    ///
    /// * `Avoid::Ferries` indicates that the calculated route should avoid
    /// ferries.
    ///
    /// * `Avoid::Indoor` indicates that the calculated route should avoid
    /// indoor steps for walking and transit directions. Only requests that
    /// include an API key or a Google Maps Platform Premium Plan client ID will
    /// receive indoor steps by default.
    ///
    /// [Route Restrictions](https://developers.google.com/maps/documentation/directions/intro#Restrictions)
    ///
    /// Directions may be calculated that adhere to certain restrictions.
    /// Restrictions are indicated by use of the avoid parameter, and an
    /// argument to that parameter indicating the restriction to avoid.
    ///
    /// It's possible to request a route that avoids any combination of tolls,
    /// highways and ferries by passing both restrictions.
    ///
    /// _Note_: the addition of restrictions does not preclude routes that
    /// include the restricted feature; it simply biases the result to more
    /// favorable routes.
    ///
    /// ## Examples:
    ///
    /// * Only avoid highways:
    ///
    /// ```rust
    /// .with_restriction(Avoid::Highways)
    /// ```
    ///
    /// * Multiple restrictions may be stacked together. This example avoids
    /// tolls and ferries:
    ///
    /// ```rust
    /// .with_restriction(Avoid::Tolls)
    /// .with_restriction(Avoid::Ferries)
    /// ```

    pub fn with_restriction(&'a mut self, restriction: Avoid) -> &'a mut Self {
        // Add restriction to Request struct.
        self.restrictions = vec![restriction];
        // Return modified Request struct to caller.
        self
    } // fn

    /// Specify features that routes should avoid.
    ///
    /// # Example:
    ///
    /// * Alternatively, multiple restrictions may be passed in a single method
    /// call by passing a slice. This example avoids tolls and ferries:
    ///
    /// ```rust
    /// .with_restrictions(&[
    ///     Avoid::Tolls,
    ///     Avoid::Ferries,
    /// ])
    /// ```

    pub fn with_restrictions(
        &'a mut self,
        restrictions: impl IntoIterator<Item = Avoid>,
    ) -> &'a mut Self {
        // Add restrictions to Request struct.
        self.restrictions = restrictions.into_iter().map(Into::into).collect();
        // Return modified Request struct to caller.
        self
    } // fn
} // impl
