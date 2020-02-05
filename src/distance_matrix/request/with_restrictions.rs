use crate::directions::request::avoid::Avoid;
use crate::distance_matrix::request::Request;

impl<'a> Request<'a> {

    /// Specify features that routes should avoid.
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
    /// ```
    /// .with_restrictions(vec![
    ///     Avoid::Tolls,
    ///     Avoid::Ferries,
    /// ])
    ///
    /// ```
    /// * Only avoid highways:
    /// ```
    /// .with_restrictions(vec![Avoid::Highways])
    /// ```

    pub fn with_restrictions(&'a mut self, restrictions: Vec<Avoid>) -> &'a mut Request {
        self.restrictions = Some(restrictions);
        self
    } // fn

} // impl