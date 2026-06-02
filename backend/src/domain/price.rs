//! Two-layer price resolution. See `docs/ARCHITECTURE.md` Rule 2.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::money::Kobo;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PriceSource {
    AccountOverride,
    GlobalDefault,
}

impl PriceSource {
    /// Map the `is_override` boolean returned by the resolution query.
    pub fn from_is_override(is_override: bool) -> Self {
        if is_override {
            PriceSource::AccountOverride
        } else {
            PriceSource::GlobalDefault
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedPrice {
    pub item_id:     Uuid,
    pub description: String,
    pub unit:        String,
    pub rate:        Kobo,
    pub source:      PriceSource,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flag_maps_correctly() {
        assert_eq!(PriceSource::from_is_override(true),  PriceSource::AccountOverride);
        assert_eq!(PriceSource::from_is_override(false), PriceSource::GlobalDefault);
    }

    #[test]
    fn serializes_snake_case() {
        assert_eq!(
            serde_json::to_string(&PriceSource::AccountOverride).unwrap(),
            "\"account_override\"",
        );
        assert_eq!(
            serde_json::to_string(&PriceSource::GlobalDefault).unwrap(),
            "\"global_default\"",
        );
    }
}
