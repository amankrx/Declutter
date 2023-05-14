use gtk::glib;

/// A [DurationKind] is a type of habit.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, strum::EnumString, strum::AsRefStr,
)]
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

impl glib::ToValue for DurationKind {
    fn to_value(&self) -> glib::Value {
        self.as_ref().to_value()
    }

    fn value_type(&self) -> glib::Type {
        <String as glib::StaticType>::static_type()
    }
}

impl glib::StaticType for DurationKind {
    fn static_type() -> glib::Type {
        <String as glib::StaticType>::static_type()
    }
}

impl DurationKind {
    pub fn as_str(&self) -> &str {
        self.as_ref()
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
