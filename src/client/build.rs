// =============================================================================

impl crate::client::Client {
    // -------------------------------------------------------------------------
    //
    /// Completes the builder pattern into a final structure.
    ///
    /// ## Arguments
    ///
    /// This method accepts no arguments.
    #[cfg(feature = "reqwest")]
    #[must_use]
    pub fn build(&self) -> Self {
        self.clone()
    } // fn

    // -------------------------------------------------------------------------
    //
    /// Completes the builder pattern into a final structure.
    ///
    /// ## Arguments
    ///
    /// This method accepts no arguments.
    #[cfg(not(feature = "reqwest"))]
    #[must_use]
    pub fn build(&self) -> Self {
        Self {
            key: self.key.clone(),
        } // GoogleMapsClient
    } // fn

    // -------------------------------------------------------------------------
    //
    /// Completes the builder pattern into a final structure.
    ///
    /// `GoogleMapsClient::build()` is preferred. `finalize` has been kept for
    /// backward compatibility.
    ///
    /// ## Arguments
    ///
    /// This method accepts no arguments.
    #[cfg(feature = "reqwest")]
    #[deprecated(since = "3.4.3", note = "use `build` instead")]
    #[must_use]
    pub fn finalize(&self) -> Self {
        self.build()
    } // fn

    // -------------------------------------------------------------------------
    //
    /// Completes the builder pattern into a final structure.
    ///
    /// `GoogleMapsClient::build()` is preferred. `finalize` has been kept for
    /// backward compatibility.
    ///
    /// ## Arguments
    ///
    /// This method accepts no arguments.
    #[cfg(not(feature = "reqwest"))]
    #[deprecated(since = "3.4.3", note = "use `build` instead")]
    #[must_use]
    pub fn finalize(&self) -> Self {
        self.build()
    } // fn
} // impl