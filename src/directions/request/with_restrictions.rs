use crate::directions::request::{
    avoid::Avoid,
    Request,
}; // use

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
    /// * Avoid tolls and ferries:
    /// ```rust
    /// .with_restrictions(vec![
    ///     Avoid::Tolls,
    ///     Avoid::Ferries,
    /// ])
    ///
    /// ```
    /// * Only avoid highways:
    /// ```rust
    /// .with_restrictions(vec![Avoid::Highways])
    /// ```

    pub fn with_restriction(&'a mut self, restriction: Avoid) -> &'a mut Request {
        // Add restriction to Request struct.
        match &mut self.restrictions {
            // If there are no restrictions in the request struct, initialize:
            None => self.restrictions = Some(vec![restriction]),
            // If there are already restrictions, append to them:
            Some(restrictions) => restrictions.push(restriction),
        } // match
        // Return modified Request struct to caller.
        self
    } // fn

    /// Specify features that routes should avoid.
    ///
    /// # Example:
    ///
    /// * Alternatively, multiple restrictions may be passed in a single method
    /// call by passing a Vec. This example avoids tolls and ferries:
    ///
    /// ```rust
    /// .with_restrictions(vec![
    ///     Avoid::Tolls,
    ///     Avoid::Ferries,
    /// ])
    /// ```

    pub fn with_restrictions(&'a mut self, restrictions_slice: &[Avoid]) -> &'a mut Request {
        // Add restrictions to Request struct.
        match &mut self.restrictions {
            // If there are no filters in the request struct, initialize field:
            None => self.restrictions = Some(restrictions_slice.to_vec()),
            // If there are already filters, append to them:
            Some(restrictions) => for restriction in restrictions_slice {
                restrictions.push(restriction.to_owned())
            } // case
        } // match
        // Return modified Request struct to caller.
        self
    } // fn

} // impl