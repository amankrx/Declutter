use std::str::FromStr;

use gtk::glib;

/// A [DurationKind] is a type of habit.
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
    strum::AsRefStr,
    glib::Boxed,
)]
#[boxed_type(name = "duration_kind")]
#[strum(serialize_all = "snake_case")]
pub enum DurationKind {
    Daily,
    Monthly,
}

impl Default for DurationKind {
    fn default() -> Self {
        Self::Daily
    }
}

impl DurationKind {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl serde::Serialize for DurationKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for DurationKind {
    fn deserialize<D>(deserializer: D) -> Result<DurationKind, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DurationKind::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_duration_kind_default() {
        assert_eq!(DurationKind::default(), DurationKind::Daily);
    }

    #[test]
    fn deserialize_duration_kind() {
        assert_eq!(DurationKind::from_str("daily"), Ok(DurationKind::Daily));
        assert_eq!(DurationKind::from_str("monthly"), Ok(DurationKind::Monthly));
        assert!(DurationKind::from_str("invalid").is_err());
    }

    #[test]
    fn serialize_duration_kind() {
        assert_eq!(DurationKind::Daily.as_str(), "daily");
        assert_eq!(DurationKind::Monthly.as_str(), "monthly");
    }
}
