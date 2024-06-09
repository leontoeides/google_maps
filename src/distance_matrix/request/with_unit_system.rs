use crate::directions::request::unit_system::UnitSystem;
use crate::distance_matrix::request::Request;

impl<'a> Request<'a> {
    /// Specifies the unit system to use when displaying results.
    ///
    /// ## Arguments
    ///
    /// * `unit_system` ‧ The measurement system the distance matrix service
    ///   should supply in the response, _imperial_ or _metric_?
    ///
    /// ## Description
    ///
    /// [Unit Systems](https://developers.google.com/maps/documentation/distance-matrix/intro#unit_systems)
    ///
    /// Directions results contain `text` within `distance` fields that may be
    /// displayed to the user to indicate the distance of a particular "step" of
    /// the route. By default, this text uses the unit system of the origin's
    /// country or region.
    ///
    /// For example, a route from "Chicago, IL" to "Toronto, ONT" will display
    /// results in miles, while the reverse route will display results in
    /// kilometers. You may override this unit system by setting one explicitly
    /// within the request's `unit_system` parameter, passing one of the
    /// following values:
    ///
    /// * `UnitSystem::Metric` specifies usage of the metric system. Textual
    ///   distances are returned using kilometers and meters.
    ///
    /// * `UnitSystem::Imperial` specifies usage of the Imperial (English)
    ///   system. Textual distances are returned using miles and feet.
    ///
    /// *Note*: this unit system setting only affects the `text` displayed
    /// within `distance` fields. The `distance` fields also contain `values`
    /// which are always expressed in meters.
    ///
    /// ## Example
    ///
    /// * Force unit system to Metric:
    /// ```rust
    /// .with_unit_system(UnitSystem::Metric)
    /// ```

    pub fn with_unit_system(
        &'a mut self,
        unit_system: impl Into<UnitSystem>
    ) -> &'a mut Self {
        self.unit_system = Some(unit_system.into());
        self
    } // fn
} // impl
