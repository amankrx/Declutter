use std::str::FromStr;

use gtk::glib;

/// A [Compare] is a comparison operator.
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    strum::EnumString,
    glib::Boxed,
    strum::AsRefStr,
)]
#[boxed_type(name = "compare")]
#[strum(serialize_all = "snake_case")]
pub enum Compare {
    Greater,
    GreaterOrEqual,
    Equal,
    NotEqual,
    LessOrEqual,
    Less,
}

impl Default for Compare {
    fn default() -> Self {
        Self::Equal
    }
}

impl serde::Serialize for Compare {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Compare {
    fn deserialize<D>(deserializer: D) -> Result<Compare, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let compare_str = String::deserialize(deserializer)?;
        Compare::from_str(&compare_str).map_err(serde::de::Error::custom)
    }
}

impl Compare {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }

    pub fn from_symbol_str(s: &str) -> Option<Self> {
        match s {
            ">" => Some(Compare::Greater),
            ">=" => Some(Compare::GreaterOrEqual),
            "==" => Some(Compare::Equal),
            "!=" => Some(Compare::NotEqual),
            "<=" => Some(Compare::LessOrEqual),
            "<" => Some(Compare::Less),
            _ => None,
        }
    }

    pub fn to_symbol_str(&self) -> &str {
        match self {
            Compare::Greater => ">",
            Compare::GreaterOrEqual => ">=",
            Compare::Equal => "==",
            Compare::NotEqual => "!=",
            Compare::LessOrEqual => "<=",
            Compare::Less => "<",
        }
    }

    pub fn compare<U: PartialOrd>(&self, a: U, b: U) -> bool {
        match self {
            Compare::Greater => a > b,
            Compare::GreaterOrEqual => a >= b,
            Compare::Equal => a == b,
            Compare::NotEqual => a != b,
            Compare::LessOrEqual => a <= b,
            Compare::Less => a < b,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn deserialize_compare() {
        assert_eq!(
            Compare::from_str("greater_or_equal"),
            Ok(Compare::GreaterOrEqual)
        );
        assert_eq!(Compare::from_str("greater"), Ok(Compare::Greater));
        assert_eq!(Compare::from_str("equal"), Ok(Compare::Equal));
        assert_eq!(Compare::from_str("not_equal"), Ok(Compare::NotEqual));
        assert_eq!(Compare::from_str("less_or_equal"), Ok(Compare::LessOrEqual));
        assert_eq!(Compare::from_str("less"), Ok(Compare::Less));
        assert!(Compare::from_str("invalid").is_err());
    }

    #[test]
    fn serialize_compare() {
        assert_eq!(Compare::GreaterOrEqual.as_str(), "greater_or_equal");
        assert_eq!(Compare::Greater.as_str(), "greater");
        assert_eq!(Compare::Equal.as_str(), "equal");
        assert_eq!(Compare::NotEqual.as_str(), "not_equal");
        assert_eq!(Compare::LessOrEqual.as_str(), "less_or_equal");
        assert_eq!(Compare::Less.as_str(), "less");
    }

    #[test]
    fn test_compare_from_str() {
        assert_eq!(Compare::from_symbol_str(">"), Some(Compare::Greater));
        assert_eq!(
            Compare::from_symbol_str(">="),
            Some(Compare::GreaterOrEqual)
        );
        assert_eq!(Compare::from_symbol_str("=="), Some(Compare::Equal));
        assert_eq!(Compare::from_symbol_str("!="), Some(Compare::NotEqual));
        assert_eq!(Compare::from_symbol_str("<="), Some(Compare::LessOrEqual));
        assert_eq!(Compare::from_symbol_str("<"), Some(Compare::Less));
        assert_eq!(Compare::from_symbol_str("invalid"), None);
    }

    #[test]
    fn test_compare_to_str() {
        assert_eq!(Compare::Greater.to_symbol_str(), ">");
        assert_eq!(Compare::GreaterOrEqual.to_symbol_str(), ">=");
        assert_eq!(Compare::Equal.to_symbol_str(), "==");
        assert_eq!(Compare::NotEqual.to_symbol_str(), "!=");
        assert_eq!(Compare::LessOrEqual.to_symbol_str(), "<=");
        assert_eq!(Compare::Less.to_symbol_str(), "<");
    }

    #[test]
    fn test_compare_compare() {
        assert_eq!(Compare::Greater.compare(2, 1), true);
        assert_eq!(Compare::Greater.compare(1, 2), false);
        assert_eq!(Compare::Greater.compare(1, 1), false);

        assert_eq!(Compare::GreaterOrEqual.compare(2, 1), true);
        assert_eq!(Compare::GreaterOrEqual.compare(1, 2), false);
        assert_eq!(Compare::GreaterOrEqual.compare(1, 1), true);

        assert_eq!(Compare::Equal.compare(1, 1), true);
        assert_eq!(Compare::Equal.compare(1, 2), false);

        assert_eq!(Compare::NotEqual.compare(1, 2), true);
        assert_eq!(Compare::NotEqual.compare(1, 1), false);

        assert_eq!(Compare::LessOrEqual.compare(1, 2), true);
        assert_eq!(Compare::LessOrEqual.compare(2, 1), false);
        assert_eq!(Compare::LessOrEqual.compare(1, 1), true);

        assert_eq!(Compare::Less.compare(1, 2), true);
        assert_eq!(Compare::Less.compare(2, 1), false);
        assert_eq!(Compare::Less.compare(1, 1), false);
    }
}
