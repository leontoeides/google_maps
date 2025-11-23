use crate::places_new::Field;

// -------------------------------------------------------------------------------------------------
//
/// A collection of fields to request from the Places API.
///
/// Field masks tell the API which data to include in responses, helping reduce response size and
/// API costs.
///
/// You can request all available fields with `All`, or specify exactly which fields you need with
/// `Specific`.
///
/// > ⚠️ While the `FieldMask::All` is fine to use in development, Google discourages the use of the
/// > wildcard response field mask in production because of the large amount of data that can be
/// > returned.
///
/// > ℹ️ Further guidance for using `places.iconMaskBaseUri` and `places.iconBackgroundColor` can be
/// > found in [Place Icons](https://developers.google.com/maps/documentation/places/web-service/icons)
/// > section.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FieldMask {
    /// Request all available fields (serializes as "*").
    ///
    /// This triggers the highest applicable SKU charges since all data is returned. Use sparingly
    /// and only when you genuinely need all available information.
    ///
    /// > ⚠️ While the `FieldMask::All` is fine to use in development, Google discourages the use of
    /// > the wildcard response field mask in production because of the large amount of data that
    /// > can be returned.
    #[default] All,

    /// Request specific fields only (serializes as comma-delimited list).
    ///
    /// This is the recommended approach for cost optimization, as you only pay for the SKU tiers
    /// corresponding to the fields you actually request.
    Specific(Vec<Field>),
}

// -------------------------------------------------------------------------------------------------
//
// Method Implementations

impl FieldMask {
    /// Creates a new empty field mask.
    #[must_use]
    pub fn new() -> Self {
        Self::Specific(Vec::default())
    }

    /// Creates a field mask with the specified capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::Specific(Vec::with_capacity(capacity))
    }

    /// Creates a field mask that requests all fields.
    ///
    /// > ⚠️ While the `FieldMask::All` is fine to use in development, Google discourages the use of
    /// > the wildcard response field mask in production because of the large amount of data that
    /// > can be returned.
    #[must_use]
    pub const fn all() -> Self {
        Self::All
    }

    /// Adds a field to the mask.
    ///
    /// If the mask is `All`, this method converts it to `Specific` with just the provided field. If
    /// already `Specific`, appends the field to the list.
    pub fn push(&mut self, field: Field) {
        match self {
            Self::All => *self = Self::Specific(vec![field]),
            Self::Specific(fields) => fields.push(field),
        }
    }

    /// Returns the number of fields in the mask.
    ///
    /// Returns `None` for `All` since the count depends on all available fields.
    #[must_use]
    pub fn len(&self) -> Option<usize> {
        match self {
            Self::All => None,
            Self::Specific(fields) => Some(fields.len()),
        }
    }

    /// Returns whether the mask is empty.
    ///
    /// Always returns `false` for `All` since it represents all fields.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::All => false,
            Self::Specific(fields) => fields.is_empty(),
        }
    }

    /// Returns an iterator over the fields if this is a `Specific` mask.
    ///
    /// Returns `None` for `All` since iterating all possible fields isn't meaningful.
    #[must_use]
    #[allow(clippy::iter_not_returning_iterator, reason = "required to accomdate `All` variant")]
    pub fn iter(&self) -> Option<std::slice::Iter<'_, Field>> {
        match self {
            Self::All => None,
            Self::Specific(fields) => Some(fields.iter()),
        }
    }

    /// Returns whether the mask requests all fields.
    #[must_use]
    pub const fn is_all(&self) -> bool {
        matches!(self, Self::All)
    }

    /// Returns whether the mask contains any fields that trigger the [Text Search Essentials ID
    /// Only SKU](https://developers.google.com/maps/billing-and-pricing/sku-details#text-search-id-only-ess-sku).
    ///
    /// Checks if any fields in the mask belong to the ID Only pricing tier, which includes basic
    /// identifiers and pagination tokens.
    ///
    /// Use this to determine if your request will incur at least the base ID Only SKU charge.
    ///
    /// Returns `true` for `All` since all fields would be requested, including ID Only fields.
    #[must_use]
    pub fn has_text_search_essentials_id_only(&self) -> bool {
        match self {
            Self::All => true,
            Self::Specific(fields) => fields.iter().any(Field::is_text_search_essentials_id_only),
        }
    }

    /// Returns whether the mask contains any fields that trigger the [Text Search Pro
    /// SKU](https://developers.google.com/maps/billing-and-pricing/sku-details#text-search-pro-sku).
    ///
    /// Checks if any fields in the mask belong to the Pro pricing tier, which includes richer place
    /// data like addresses, photos, and location details.
    ///
    /// Use this to verify if your request will incur Pro SKU charges beyond the basic ID Only tier.
    ///
    /// Returns `true` for `All` since all fields would be requested, including Pro fields.
    #[must_use]
    pub fn has_text_search_pro(&self) -> bool {
        match self {
            Self::All => true,
            Self::Specific(fields) => fields.iter().any(Field::is_text_search_pro),
        }
    }

    /// Returns whether the mask contains any fields that trigger the [Text Search Enterprise
    /// SKU](https://developers.google.com/maps/billing-and-pricing/sku-details#text-search-ent-sku).
    ///
    /// Checks if any fields in the mask belong to the Enterprise pricing tier, which includes
    /// business-critical data like contact info, hours, and ratings.
    ///
    /// Use this to determine if your request will incur Enterprise SKU charges, helping manage
    /// costs when higher-tier data isn't required.
    ///
    /// Returns `true` for `All` since all fields would be requested, including Enterprise fields.
    #[must_use]
    pub fn has_text_search_enterprise(&self) -> bool {
        match self {
            Self::All => true,
            Self::Specific(fields) => fields.iter().any(Field::is_text_search_enterprise),
        }
    }

    /// Returns whether the mask contains any fields that trigger the [Text Search Enterprise +
    /// Atmosphere SKU](https://developers.google.com/maps/billing-and-pricing/sku-details#text-search-ent-plus-sku).
    ///
    /// Checks if any fields in the mask belong to the highest pricing tier, which includes detailed
    /// amenities, reviews, and AI-generated content.
    ///
    /// Use this before making requests to identify when you're requesting the most expensive data
    /// tier, allowing you to optimize costs by excluding atmosphere fields when not essential.
    ///
    /// Returns `true` for `All` since all fields would be requested, including Atmosphere fields.
    #[must_use]
    pub fn has_text_search_enterprise_and_atmosphere(&self) -> bool {
        match self {
            Self::All => true,
            Self::Specific(fields) => fields.iter().any(Field::is_text_search_enterprise_and_atmosphere),
        }
    }
}

// -------------------------------------------------------------------------------------------------
//
// Trait Implementations

impl std::fmt::Display for FieldMask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "*"),
            Self::Specific(fields) => {
                let field_strs: Vec<String> = fields
                    .iter()
                    .map(|field| {
                        serde_json::to_string(field)
                            .unwrap_or_default()
                            .trim_matches('"')
                            .to_string()
                    })
                    .collect();

                write!(f, "{}", field_strs.join(","))
            }
        }
    }
}

impl From<Vec<Field>> for FieldMask {
    fn from(fields: Vec<Field>) -> Self {
        Self::Specific(fields)
    }
}

impl From<&[Field]> for FieldMask {
    fn from(fields: &[Field]) -> Self {
        Self::Specific(fields.to_vec())
    }
}

impl FromIterator<Field> for FieldMask {
    fn from_iter<T: IntoIterator<Item = Field>>(iter: T) -> Self {
        Self::Specific(iter.into_iter().collect())
    }
}

impl<const N: usize> From<[Field; N]> for FieldMask {
    /// Creates a `FieldMask` from a fixed-length array of fields.
    ///
    /// Converts an owned array of fields into a specific field mask, useful for compile-time
    /// known field sets and array literals.
    fn from(fields: [Field; N]) -> Self {
        Self::Specific(fields.to_vec())
    }
}

impl<const N: usize> From<&[Field; N]> for FieldMask {
    /// Creates a `FieldMask` from a reference to a fixed-length array of fields.
    ///
    /// Converts an array reference into a specific field mask, providing flexibility when working
    /// with borrowed array data.
    fn from(fields: &[Field; N]) -> Self {
        Self::Specific(fields.to_vec())
    }
}