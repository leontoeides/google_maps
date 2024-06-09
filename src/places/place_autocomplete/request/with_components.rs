use crate::places::place_autocomplete::request::Request;
use crate::Country;

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {
    /// Adds the components parameter to the Place API _Place Autocomplete_
    /// query.
    ///
    /// ## Arguments
    ///
    /// * `component` ‧ A grouping of places to which you would like to restrict
    ///   your results. Currently, you can use components to filter by up to 5
    ///   countries.
    ///
    /// * Multiple components may be stacked together.

    pub fn with_component(
        &'a mut self,
        component: impl Into<Country>
    ) -> &'a mut Self {
        // Set components in Request struct.
        self.components.extend(vec![component.into()]);
        // Return modified Request struct to caller.
        self
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl<'a> Request<'a> {
    /// Adds the components parameter to the Place API _Place Autocomplete_
    /// query.
    ///
    /// ## Arguments
    ///
    /// * `components` ‧ A grouping of places to which you would like to restrict
    ///   your results. Currently, you can use components to filter by up to 5
    ///   countries.
    ///
    /// * Multiple components may be stacked together.
    ///
    /// # Generics
    ///
    /// This method uses generics to improve ergonomics. The `C` generic is
    /// intended to represent any collection that can be iterated over, and the
    /// `O` generic is for any type that can be converted to the `Country`
    /// type.

    pub fn with_components<C, O>(
        &'a mut self,
        components: C
    ) -> &'a mut Self
    where
        C: IntoIterator<Item = O>,
        O: Into<Country> {
        // Set components in Request struct.
        self.components.extend(components.into_iter().map(Into::<Country>::into));
        // Return modified Request struct to caller.
        self
    } // fn
} // impl
