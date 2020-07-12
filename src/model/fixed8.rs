use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    fmt,
    ops::{Add, Mul, Sub},
};

// Fixed8 represents a fixed-point number with precision 10^-8
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Fixed8(pub i64);

pub const FIXED8_DECIMALS: i64 = 100_000_000; // 10^8

impl Fixed8 {
    pub fn decimal(self) -> i64 {
        self.0 / FIXED8_DECIMALS
    }
}

impl fmt::Debug for Fixed8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}",
            self.0 / FIXED8_DECIMALS,
            self.0 % FIXED8_DECIMALS
        )
    }
}

impl Add for Fixed8 {
    type Output = Fixed8;

    fn add(self, rhs: Self) -> Self::Output {
        Fixed8(self.0 + rhs.0)
    }
}

impl Sub for Fixed8 {
    type Output = Fixed8;

    fn sub(self, rhs: Self) -> Self::Output {
        Fixed8(self.0 - rhs.0)
    }
}

impl Mul<i64> for Fixed8 {
    type Output = Fixed8;

    fn mul(self, rhs: i64) -> Self::Output {
        Fixed8(self.0 * rhs)
    }
}

impl Serialize for Fixed8 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let abs_val = self.0.abs();
        let decimals = abs_val / FIXED8_DECIMALS;
        let residual = abs_val % FIXED8_DECIMALS;

        let sign = if self.0 < 0 { "-" } else { "" };

        let v = format!("{}{}.{:08}", sign, decimals, residual);

        serializer.serialize_str(&v)
    }
}

pub struct Fixed8Visitor {}

impl<'de> Visitor<'de> for Fixed8Visitor {
    type Value = Fixed8;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string with a fixed-point float with 10^-8 precision")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let (v, sign): (&str, i64) = if v.starts_with('-') {
            (&v[1..], -1)
        } else {
            (v, 1)
        };

        let mut parts = v.split('.');

        // Unwrap is safe here bc split always produces at least one value
        let decimals: i64 = parts
            .next()
            .unwrap()
            .parse()
            .map_err(|_| E::custom("couldn't parse decimal part of fixed8 value"))?;
        let residual: i64 = match parts.next() {
            Some(value) => {
                let value = if value.len() > 8 { &value[..8] } else { value };

                value
                    .parse()
                    .map_err(|_| E::custom("couldn't parse float part of fixed8 value"))?
            }
            None => 0,
        };

        Ok(Fixed8(sign * (decimals * FIXED8_DECIMALS + residual)))
    }
}

impl<'de> Deserialize<'de> for Fixed8 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(Fixed8Visitor {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_fixed8() {
        let a = Fixed8(FIXED8_DECIMALS * 52 + 123);
        let b = Fixed8(FIXED8_DECIMALS * 52 + 123 * (FIXED8_DECIMALS / 1000));
        let c = Fixed8(123);
        let d = Fixed8(123 * (FIXED8_DECIMALS / 1000));
        let e = Fixed8(-123 * (FIXED8_DECIMALS / 1000));

        let ser = |x: Fixed8| serde_json::to_string(&x).unwrap();

        assert_eq!(ser(a), "\"52.00000123\"");
        assert_eq!(ser(b), "\"52.12300000\"");
        assert_eq!(ser(c), "\"0.00000123\"");
        assert_eq!(ser(d), "\"0.12300000\"");
        assert_eq!(ser(e), "\"-0.12300000\"");
    }

    #[test]
    fn deserialize_fixed8() {
        let a = Fixed8(FIXED8_DECIMALS * 52 + 123);
        let b = Fixed8(FIXED8_DECIMALS * 52 + 123 * (FIXED8_DECIMALS / 1000));
        let c = Fixed8(123);
        let d = Fixed8(123 * (FIXED8_DECIMALS / 1000));
        let e = Fixed8(-123 * (FIXED8_DECIMALS / 1000));
        let f = Fixed8(FIXED8_DECIMALS * 52);

        let de = |s: &str| serde_json::from_str::<Fixed8>(s).unwrap();

        assert_eq!(de("\"52.00000123\""), a);
        assert_eq!(de("\"52.12300000\""), b);
        assert_eq!(de("\"0.00000123\""), c);
        assert_eq!(de("\"0.12300000\""), d);
        assert_eq!(de("\"-0.12300000\""), e);
        assert_eq!(de("\"52.000001235\""), a);
        assert_eq!(de("\"52\""), f);
    }
}
