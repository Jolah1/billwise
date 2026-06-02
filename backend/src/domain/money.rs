//! Money handling. Money is integer kobo throughout BillWise.
//! See `docs/ARCHITECTURE.md` Rule 1.

use std::fmt;
use std::ops::Add;

use bigdecimal::{BigDecimal, RoundingMode, ToPrimitive};
use serde::{Deserialize, Serialize};

/// An integer number of kobo. 1 naira = 100 kobo.
///
/// Stored as `i64` on the wire and in Postgres `BIGINT`. The `Display`
/// impl is the ONLY place in the backend that formats to `₦`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
         Serialize, Deserialize)]
#[serde(transparent)]
pub struct Kobo(i64);

impl Kobo {
    pub const ZERO: Kobo = Kobo(0);

    pub const fn new(kobo: i64) -> Self { Kobo(kobo) }
    pub const fn as_i64(self) -> i64 { self.0 }

    /// `rate × quantity`, rounded half-up to the nearest kobo.
    /// Returns `None` if the result overflows `i64`.
    pub fn times_quantity(self, quantity: &BigDecimal) -> Option<Kobo> {
        let raw = BigDecimal::from(self.0) * quantity;
        raw.with_scale_round(0, RoundingMode::HalfUp)
            .to_i64()
            .map(Kobo)
    }

    /// Checked sum. `None` on overflow.
    pub fn sum<I: IntoIterator<Item = Kobo>>(iter: I) -> Option<Kobo> {
        iter.into_iter()
            .try_fold(0_i64, |acc, k| acc.checked_add(k.0))
            .map(Kobo)
    }
}

/// Addition only works `Kobo + Kobo`. Panics on overflow — money silently
/// wrapping is never the right behaviour, and the alternative (returning
/// `Option`) ruins ergonomics. Use `Kobo::sum` for big folds.
impl Add for Kobo {
    type Output = Kobo;
    fn add(self, rhs: Kobo) -> Kobo {
        Kobo(self.0.checked_add(rhs.0).expect("kobo overflow"))
    }
}

impl fmt::Display for Kobo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let abs        = self.0.unsigned_abs();
        let naira      = abs / 100;
        let kobo_part  = abs % 100;
        let sign       = if self.0 < 0 { "-" } else { "" };
        write!(f, "{sign}\u{20A6}{}.{kobo_part:02}", group_thousands(naira))
    }
}

fn group_thousands(n: u64) -> String {
    let s = n.to_string();
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(bytes.len() + bytes.len() / 3);
    for (i, b) in bytes.iter().enumerate() {
        let remaining = bytes.len() - i;
        if i > 0 && remaining % 3 == 0 {
            out.push(',');
        }
        out.push(*b as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn add_combines_two_kobo() {
        assert_eq!(Kobo(1_500) + Kobo(2_500), Kobo(4_000));
    }

    #[test]
    #[should_panic(expected = "kobo overflow")]
    fn add_panics_on_overflow() {
        let _ = Kobo(i64::MAX) + Kobo(1);
    }

    #[test]
    fn sum_is_checked() {
        assert_eq!(Kobo::sum([Kobo(100), Kobo(200), Kobo(300)]), Some(Kobo(600)));
        assert_eq!(Kobo::sum([Kobo(i64::MAX), Kobo(1)]), None);
    }

    #[test]
    fn times_quantity_simple() {
        let q = BigDecimal::from_str("2.5").unwrap();
        // 1234 × 2.5 = 3085
        assert_eq!(Kobo(1_234).times_quantity(&q), Some(Kobo(3_085)));
    }

    #[test]
    fn times_quantity_rounds_half_up() {
        // 100 × 0.005 = 0.5 → 1 (half-up)
        let q = BigDecimal::from_str("0.005").unwrap();
        assert_eq!(Kobo(100).times_quantity(&q), Some(Kobo(1)));
        // 100 × 0.004 = 0.4 → 0
        let q = BigDecimal::from_str("0.004").unwrap();
        assert_eq!(Kobo(100).times_quantity(&q), Some(Kobo(0)));
    }

    #[test]
    fn times_quantity_handles_large_values() {
        // ₦1,000,000.00 × 3.5 = ₦3,500,000.00 = 350_000_000 kobo
        let q = BigDecimal::from_str("3.5").unwrap();
        assert_eq!(Kobo(100_000_000).times_quantity(&q), Some(Kobo(350_000_000)));
    }

    #[test]
    fn display_small_values() {
        assert_eq!(Kobo::ZERO.to_string(),    "\u{20A6}0.00");
        assert_eq!(Kobo(1).to_string(),       "\u{20A6}0.01");
        assert_eq!(Kobo(99).to_string(),      "\u{20A6}0.99");
        assert_eq!(Kobo(100).to_string(),     "\u{20A6}1.00");
        assert_eq!(Kobo(12_345).to_string(),  "\u{20A6}123.45");
    }

    #[test]
    fn display_groups_thousands() {
        assert_eq!(Kobo(1_234_567).to_string(),     "\u{20A6}12,345.67");
        assert_eq!(Kobo(1_000_000_000).to_string(), "\u{20A6}10,000,000.00");
    }

    #[test]
    fn display_handles_negative() {
        assert_eq!(Kobo(-100).to_string(),       "-\u{20A6}1.00");
        assert_eq!(Kobo(-1_234_567).to_string(), "-\u{20A6}12,345.67");
    }

    #[test]
    fn serializes_as_plain_integer() {
        assert_eq!(serde_json::to_string(&Kobo(12_345)).unwrap(), "12345");
        let back: Kobo = serde_json::from_str("12345").unwrap();
        assert_eq!(back, Kobo(12_345));
    }
}
