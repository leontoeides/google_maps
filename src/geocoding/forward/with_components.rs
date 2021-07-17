use crate::geocoding::forward::{component::Component, ForwardRequest};

impl<'a> ForwardRequest<'a> {

    /// Restricts the results from the geocoder to the specified component
    /// type(s).
    ///
    /// ## Arguments:
    ///
    /// * `component` - A single component filter of `Component` type.
    ///
    /// ## Description:
    ///
    /// [Component
    /// Filtering](https://developers.google.com/maps/documentation/geocoding/intro#ComponentFiltering)
    ///
    /// In a Geocoding response, the Geocoding API can return address results
    /// restricted to a specific area. You can specify the restriction using the
    /// components filter. Filter values support the same methods of spelling
    /// correction and partial matching as other Geocoding requests. If the
    /// geocoder finds a partial match for a component filter, the response will
    /// contain a `partial_match` field.
    ///
    /// The components that can be filtered include:
    ///
    /// * `Component::Route` matches the long or short name of a route.
    ///
    /// * `Component::Locality` matches against `locality` and `sublocality`
    /// types.
    ///
    /// * `Component::AdministrativeArea` matches all the `administrative_area`
    /// levels.
    ///
    /// Notes about component filtering:
    ///
    /// * If the request contains multiple component filters, the API evaluates
    /// them as an AND, not an OR. For example, if the request includes multiple
    /// countries `components=country:GB|country:AU`, the API looks for
    /// locations where country=GB AND country=AU, and returns `ZERO_RESULTS`.
    ///
    /// * Results are consistent with Google Maps, which occasionally yields
    /// unexpected `ZERO_RESULTS` responses. Using Place Autocomplete may
    /// provide better results in some use cases. To learn more, see [this
    /// FAQ](https://developers.google.com/maps/documentation/geocoding/faq#trbl_component_filtering).
    ///
    /// * For each address component, either specify it in the `address`
    /// parameter or in a `components` filter, but not both. Specifying the same
    /// values in both may result in `ZERO_RESULTS`.
    ///
    /// ## Examples:
    ///
    /// * A single component filter. This example restricts results to Toronto:
    ///
    /// ```rust
    /// .with_component(GeocodingComponent::Locality(String::from("Toronto")))
    /// ```
    ///
    /// * Multiple component filters may be stacked together. This example
    /// restricts results to a street in a city:
    ///
    /// ```rust
    /// .with_component(GeocodingComponent::Route(String::from("Downing Street")))
    /// .with_component(GeocodingComponent::Locality(String::from("London"))))
    /// ```

    pub fn with_component(
        &'a mut self,
        component: Component
    ) -> &'a mut ForwardRequest {
        // Add component to ForwardRequest struct.
        match &mut self.components {
            // If there are no components in the request struct, initialize:
            None => self.components = Some(vec![component]),
            // If there are already components, append to them:
            Some(components) => components.push(component),
        } // match
          // Return modified ForwardRequest struct to caller.
        self
    } // fn

    /// Restricts the results from the geocoder to the specified component
    /// type(s).
    ///
    /// # Example:
    ///
    /// * Alternatively, multiple component filters may be passed in a single
    /// method call by passing a Vec. This example restricts results to a street
    /// in a city:
    ///
    /// ```rust
    /// .with_components(&vec![
    ///     GeocodingComponent::Route(String::from("Downing Street")),
    ///     GeocodingComponent::Locality(String::from("London")),
    /// ])
    /// ```

    pub fn with_components(
        &'a mut self,
        components_slice: &[Component]
    ) -> &'a mut ForwardRequest {
        // Add components to ForwardRequest struct.
        match &mut self.components {
            // If there are no filters in the request struct, initialize field:
            None => self.components = Some(components_slice.to_vec()),
            // If there are already filters, append to them:
            Some(components) => components_slice.iter().for_each(|component| components.push(component.to_owned())),
        } // match
        // Return modified ForwardRequest struct to caller.
        self
    } // fn

} // impl