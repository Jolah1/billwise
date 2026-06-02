//! BOQ item categories and the derived amount calculation.
//! See `docs/ARCHITECTURE.md` Rule 3.

use std::str::FromStr;

use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use super::money::Kobo;

/// Standard BOQ item categories used in Nigerian quantity surveying.
///
/// Stored as a Postgres `item_type` enum. Serialized on the wire as a
/// snake_case JSON string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemType {
    Measured,
    ProvisionalSum,
    PcSum,
    PrimeCost,
}

impl ItemType {
    /// The exact literal stored in the Postgres `item_type` enum.
    /// The DB layer casts to/from `TEXT` using this string.
    pub fn as_str(self) -> &'static str {
        match self {
            ItemType::Measured       => "measured",
            ItemType::ProvisionalSum => "provisional_sum",
            ItemType::PcSum          => "pc_sum",
            ItemType::PrimeCost      => "prime_cost",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownItemType(pub String);

impl std::fmt::Display for UnknownItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown item_type: {}", self.0)
    }
}
impl std::error::Error for UnknownItemType {}

impl FromStr for ItemType {
    type Err = UnknownItemType;
    fn from_str(s: &str) -> Result<ItemType, UnknownItemType> {
        Ok(match s {
            "measured"        => ItemType::Measured,
            "provisional_sum" => ItemType::ProvisionalSum,
            "pc_sum"          => ItemType::PcSum,
            "prime_cost"      => ItemType::PrimeCost,
            other             => return Err(UnknownItemType(other.to_owned())),
        })
    }
}

/// The canonical amount calculation: `quantity × rate`, rounded half-up to
/// the nearest kobo. Returns `None` on overflow.
///
/// The server is the sole authority on amounts — a client-sent amount is
/// always ignored.
pub fn amount(quantity: &BigDecimal, rate: Kobo) -> Option<Kobo> {
    rate.times_quantity(quantity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_type_round_trips() {
        for v in [
            ItemType::Measured,
            ItemType::ProvisionalSum,
            ItemType::PcSum,
            ItemType::PrimeCost,
        ] {
            assert_eq!(ItemType::from_str(v.as_str()).unwrap(), v);
        }
    }

    #[test]
    fn item_type_unknown_errors() {
        assert!(ItemType::from_str("not_a_real_type").is_err());
    }

    #[test]
    fn item_type_serializes_snake_case() {
        assert_eq!(
            serde_json::to_string(&ItemType::ProvisionalSum).unwrap(),
            "\"provisional_sum\"",
        );
        let back: ItemType = serde_json::from_str("\"pc_sum\"").unwrap();
        assert_eq!(back, ItemType::PcSum);
    }

    #[test]
    fn amount_typical_case() {
        // 50 m² × ₦1,234.50 = ₦61,725.00 = 6_172_500 kobo
        let qty = BigDecimal::from_str("50").unwrap();
        assert_eq!(amount(&qty, Kobo::new(123_450)), Some(Kobo::new(6_172_500)));
    }

    #[test]
    fn amount_rounds_half_up() {
        let qty = BigDecimal::from_str("1.5").unwrap();
        assert_eq!(amount(&qty, Kobo::new(1)), Some(Kobo::new(2)));
    }

    #[test]
    fn amount_zero_quantity_is_zero() {
        let qty = BigDecimal::from_str("0").unwrap();
        assert_eq!(amount(&qty, Kobo::new(99_999)), Some(Kobo::ZERO));
    }

    #[test]
    fn amount_lump_sum_convention_quantity_one() {
        // Per docs/ARCHITECTURE.md: provisional/PC sums use quantity=1
        // and put the lump sum in rate_kobo for slice 1.
        let qty = BigDecimal::from_str("1").unwrap();
        assert_eq!(
            amount(&qty, Kobo::new(2_500_000_00)),
            Some(Kobo::new(2_500_000_00)),
        );
    }
}
