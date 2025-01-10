use crate::places::place_autocomplete::AutocompleteType;

// -----------------------------------------------------------------------------

impl crate::places::place_autocomplete::Request<'_> {
    /// Adds the types parameter to the Place API _Place Autocomplete_ query.
    ///
    /// ## Arguments
    ///
    /// * `types` ‧ You may restrict results from a Place Autocomplete request to
    ///   be of a certain type by passing a types parameter. The parameter
    ///   specifies a type or a type collection, as listed in the supported
    ///   types below. If nothing is specified, all types are returned. In
    ///   general only a single type is allowed. The exception is that you can
    ///   safely mix the geocode and establishment types, but note that this
    ///   will have the same effect as specifying no types.
    ///
    /// * Multiple result type filters may be stacked together.
    #[must_use] pub fn with_type(
        mut self,
        autocomplete_type: impl Into<AutocompleteType>
    ) -> Self {
        // Set types in Request struct.
        self.types.push(autocomplete_type.into());
        // Return modified Request struct to caller.
        self
    } // fn

    /// Adds the types parameter to the Place API _Place Autocomplete_ query.
    ///
    /// ## Arguments
    ///
    /// * `types` ‧ You may restrict results from a Place Autocomplete request to
    ///   be of a certain type by passing a types parameter. The parameter
    ///   specifies a type or a type collection, as listed in the supported
    ///   types below. If nothing is specified, all types are returned. In
    ///   general only a single type is allowed. The exception is that you can
    ///   safely mix the geocode and establishment types, but note that this
    ///   will have the same effect as specifying no types.
    ///
    /// * Multiple result type filters may be stacked together.
    ///
    /// # Generics
    ///
    /// This method uses generics to improve ergonomics. The `C` generic is
    /// intended to represent any collection that can be iterated over, and the
    /// `A` generic is for any type that can be converted to the
    /// `AutocompleteType` type.
    #[must_use] pub fn with_types<C, A>(
        mut self,
        types: C
    ) -> Self
    where
        C: IntoIterator<Item = A>,
        A: Into<AutocompleteType> {
        // Set types in Request struct.
        self.types.extend(types.into_iter().map(Into::into));
        // Return modified Request struct to caller.
        self
    } // fn
} // impl