#![allow(clippy::ref_option)]

// -------------------------------------------------------------------------------------------------
//
/// Information about data providers of this place.
///
/// Attribution provides credit and source information for place data, ensuring proper recognition
/// of data contributors and compliance with data usage requirements.
///
/// This information should be displayed when showing place data to users, helping maintain
/// transparency about data sources and provider relationships.
#[derive(
    //std
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    // getset
    getset::Getters,
    // serde
    serde::Deserialize,
    serde::Serialize
)]
#[serde(rename_all = "camelCase")]
pub struct Attribution {
    /// Name of the place's data provider.
    ///
    /// The display name or organization name of the entity that provided this place data. This
    /// should be shown to users to give proper credit to data sources and maintain transparency
    /// about information origins.
    #[getset(get = "pub")]
    pub provider: String,

    /// URI to the place's data provider.
    ///
    /// Optional link to the data provider's website, profile, or more information about the source.
    /// This can be used to create clickable attribution links that direct users to learn more about
    /// the data provider.
    #[serde(default)]
    #[getset(get = "pub")]
    pub provider_uri: Option<url::Url>,
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl Attribution {
    /// Creates a new Attribution with the specified provider name.
    ///
    /// Used to construct attribution information with just the provider name. The provider URI can
    /// be added separately if available.
    #[must_use]
    pub fn new(provider: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            provider_uri: None,
        }
    }

    /// Creates a new Attribution with both provider name and URI.
    ///
    /// Used when complete attribution information is available, including a link to the data
    /// provider's website or profile page.
    pub fn with_uri(
        provider: impl Into<String>,
        provider_uri: &str
    ) -> Result<Self, crate::places_new::Error> {
        Ok(Self {
            provider: provider.into(),
            provider_uri: Some(provider_uri.try_into()?),
        })
    }

    /// Returns whether this attribution has a clickable link.
    ///
    /// Used to determine if the attribution should be displayed as a link or plain text, enabling
    /// appropriate UI rendering for attribution credits.
    #[must_use]
    pub const fn has_link(&self) -> bool {
        self.provider_uri.is_some()
    }

    /// Returns whether this attribution has valid provider information.
    ///
    /// Used to validate attribution data before displaying it to users, ensuring that meaningful
    /// attribution information is available.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.provider.trim().is_empty()
    }

    /// Gets a display-friendly attribution text.
    ///
    /// Provides formatted attribution text suitable for displaying in user interfaces, typically in
    /// the format "Data provided by [Provider]".
    #[must_use]
    pub fn display_text(&self) -> String {
        if self.provider.trim().is_empty() {
            "Data provider not specified".to_string()
        } else {
            format!("Data provided by {}", self.provider)
        }
    }

    /// Gets a short attribution text for compact displays.
    ///
    /// Provides brief attribution text suitable for space-constrained interfaces like mobile apps,
    /// map overlays, or footer areas.
    #[must_use]
    pub fn short_display_text(&self) -> String {
        if self.provider.trim().is_empty() {
            "Unknown provider".to_string()
        } else {
            format!("© {}", self.provider)
        }
    }

    /// Generates HTML markup for the attribution.
    ///
    /// Creates appropriate HTML markup with proper links if a provider URI is available, suitable
    /// for web applications and HTML-based displays.
    #[must_use]
    pub fn to_html(&self) -> String {
        if !self.is_valid() {
            return "<span class=\"attribution\">Data provider not specified</span>".to_string();
        }

        self.provider_uri
            .as_ref()
            .map_or_else(|| format!(
                "<span class=\"attribution\">{}</span>",
                html_escape(&self.provider)
            ), |uri| format!(
                "<a href=\"{}\" class=\"attribution-link\" target=\"_blank\" rel=\"noopener noreferrer\">{}</a>",
                html_escape(uri.as_str()),
                html_escape(&self.provider)
            ))
    }

    /// Generates markdown markup for the attribution.
    ///
    /// Creates markdown-formatted attribution text with links where appropriate, useful for
    /// documentation, README files, or markdown-based content.
    #[must_use]
    pub fn to_markdown(&self) -> String {
        if !self.is_valid() {
            return "_Data provider not specified_".to_string();
        }

        self.provider_uri
            .as_ref()
            .map_or_else(|| self.provider.clone(), |uri| format!("[{}]({})", self.provider, uri))
    }

    /// Returns whether this attribution should be prominently displayed.
    ///
    /// Used to determine attribution display priority, with certain providers or data types
    /// requiring more prominent attribution display.
    #[must_use]
    pub fn should_display_prominently(&self) -> bool {
        // This could be enhanced to check for specific providers that require prominent attribution
        // based on their terms of service
        self.is_valid()
    }

    /// Gets the domain from the provider URI if available.
    ///
    /// Extracts the domain portion of the provider URI for display purposes or security checks,
    /// useful for showing compact attribution information.
    #[must_use]
    pub fn provider_domain(&self) -> Option<&str> {
        self.provider_uri.as_ref().and_then(|uri| uri.domain())
    }

    /// Creates a user-friendly link description.
    ///
    /// Generates descriptive text for links that provides context about where the link leads,
    /// useful for accessibility and user experience.
    #[must_use]
    pub fn link_description(&self) -> Option<String> {
        self
            .provider_domain()
            .map_or_else(
                || if self.has_link() {
                    Some(format!("Visit {} website", self.provider))
                } else {
                    None
                },
                |domain| Some(format!("Visit {} on {}", self.provider, domain))
            )
    }
}

// -------------------------------------------------------------------------------------------------
//
// Tests

/// Simple HTML escape function for basic safety.
///
/// Escapes common HTML characters to prevent XSS when generating HTML markup. For production use,
/// consider using a dedicated HTML escaping library.
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_serialization() {
        let attribution = Attribution {
            provider: "Example Data Corp".to_string(),
            provider_uri: Some("https://example.com".try_into().unwrap()),
        };

        let json = serde_json::to_string(&attribution).unwrap();
        assert!(json.contains("Example Data Corp"));
        assert!(json.contains("https://example.com"));
    }

    #[test]
    fn test_deserialization() {
        let json = r#"{
            "provider": "Test Provider",
            "providerUri": "https://test.com"
        }"#;

        let attribution: Attribution = serde_json::from_str(json).unwrap();
        assert_eq!(attribution.provider, "Test Provider");
        assert_eq!(attribution.provider_uri, Some("https://test.com".try_into().unwrap()));
    }

    #[test]
    fn test_constructors() {
        let simple = Attribution::new("Simple Provider");
        assert_eq!(simple.provider, "Simple Provider");
        assert_eq!(simple.provider_uri, None);

        let with_uri = Attribution::with_uri("Provider", "https://example.com").unwrap();
        assert_eq!(with_uri.provider, "Provider");
        assert_eq!(with_uri.provider_uri, Some("https://example.com".try_into().unwrap()));
    }

    #[test]
    fn test_has_link() {
        let without_link = Attribution::new("Provider");
        assert!(!without_link.has_link());

        let with_link = Attribution::with_uri("Provider", "https://example.com").unwrap();
        assert!(with_link.has_link());
    }

    #[test]
    fn test_is_valid() {
        let valid = Attribution::new("Valid Provider");
        assert!(valid.is_valid());

        let empty = Attribution::new("");
        assert!(!empty.is_valid());

        let whitespace = Attribution::new("   ");
        assert!(!whitespace.is_valid());
    }

    #[test]
    fn test_display_text() {
        let attribution = Attribution::new("Test Corp");
        assert_eq!(attribution.display_text(), "Data provided by Test Corp");

        let empty = Attribution::new("");
        assert_eq!(empty.display_text(), "Data provider not specified");
    }

    #[test]
    fn test_short_display_text() {
        let attribution = Attribution::new("Test Corp");
        assert_eq!(attribution.short_display_text(), "© Test Corp");

        let empty = Attribution::new("");
        assert_eq!(empty.short_display_text(), "Unknown provider");
    }

    #[test]
    fn test_to_html() {
        let with_link = Attribution::with_uri("Test Corp", "https://test.com").unwrap();
        let html = with_link.to_html();
        assert!(html.contains("<a href=\"https://test.com/\""));
        assert!(html.contains("Test Corp"));
        assert!(html.contains("target=\"_blank\""));
        assert!(html.contains("rel=\"noopener noreferrer\""));

        let without_link = Attribution::new("Test Corp");
        let html = without_link.to_html();
        assert!(html.contains("<span"));
        assert!(html.contains("Test Corp"));
        assert!(!html.contains("<a"));

        let invalid = Attribution::new("");
        let html = invalid.to_html();
        assert!(html.contains("Data provider not specified"));
    }

    #[test]
    fn test_to_markdown() {
        let with_link = Attribution::with_uri("Test Corp", "https://test.com").unwrap();
        assert_eq!(with_link.to_markdown(), "[Test Corp](https://test.com/)");

        let without_link = Attribution::new("Test Corp");
        assert_eq!(without_link.to_markdown(), "Test Corp");

        let invalid = Attribution::new("");
        assert_eq!(invalid.to_markdown(), "_Data provider not specified_");
    }

    #[test]
    fn test_provider_domain() {
        let https = Attribution::with_uri("Provider", "https://example.com/path").unwrap();
        assert_eq!(https.provider_domain(), Some("example.com"));

        let http = Attribution::with_uri("Provider", "http://test.org").unwrap();
        assert_eq!(http.provider_domain(), Some("test.org"));

        let without_path = Attribution::with_uri("Provider", "https://domain.co.uk").unwrap();
        assert_eq!(without_path.provider_domain(), Some("domain.co.uk"));

        let without_protocol = Attribution::with_uri("Provider", "example.com");
        assert!(without_protocol.is_err());

        let without_uri = Attribution::new("Provider");
        assert_eq!(without_uri.provider_domain(), None);
    }

    #[test]
    fn test_link_description() {
        let with_domain = Attribution::with_uri("Test Corp", "https://test.com").unwrap();
        assert_eq!(
            with_domain.link_description(),
            Some("Visit Test Corp on test.com".to_string())
        );

        let with_uri_no_domain = Attribution::with_uri("Corp", "invalid-uri");
        assert!(with_uri_no_domain.is_err());

        let without_uri = Attribution::new("Corp");
        assert_eq!(without_uri.link_description(), None);
    }

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("normal text"), "normal text");
        assert_eq!(html_escape("<script>"), "&lt;script&gt;");
        assert_eq!(html_escape("A & B"), "A &amp; B");
        assert_eq!(html_escape("\"quoted\""), "&quot;quoted&quot;");
        assert_eq!(html_escape("'single'"), "&#x27;single&#x27;");
    }

    #[test]
    fn test_should_display_prominently() {
        let valid = Attribution::new("Valid Provider");
        assert!(valid.should_display_prominently());

        let invalid = Attribution::new("");
        assert!(!invalid.should_display_prominently());
    }
}