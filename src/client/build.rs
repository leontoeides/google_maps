use crate::client::GoogleMapsClient;

// =============================================================================

impl GoogleMapsClient {
    // -------------------------------------------------------------------------
    //
    /// Completes the builder pattern into a final structure.
    ///
    /// ## Arguments
    ///
    /// This method accepts no arguments.

    #[cfg(feature = "enable-reqwest")]
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

    #[cfg(not(feature = "enable-reqwest"))]
    pub fn build(&self) -> GoogleMapsClient {
        GoogleMapsClient {
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

    #[cfg(feature = "enable-reqwest")]
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

    #[cfg(not(feature = "enable-reqwest"))]
    pub fn finalize(&self) -> GoogleMapsClient {
        self.build()
    } // fn
} // impl
