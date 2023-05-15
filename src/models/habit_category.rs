use std::str::FromStr;

use gtk::glib;

/// A [HabitCategory] is a type of habit.
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
#[boxed_type(name = "habit_category")]
#[strum(serialize_all = "snake_case")]
pub enum HabitCategory {
    Body,
    Mind,
    Health,
    Study,
    Productivity,
    Finance,
    Social,
    Abstraction,
    Other,
}

impl Default for HabitCategory {
    fn default() -> Self {
        Self::Other
    }
}

impl HabitCategory {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl serde::Serialize for HabitCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for HabitCategory {
    fn deserialize<D>(deserializer: D) -> Result<HabitCategory, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let category_str = String::deserialize(deserializer)?;
        HabitCategory::from_str(&category_str).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_habit_category_default() {
        assert_eq!(HabitCategory::default(), HabitCategory::Other);
    }

    #[test]
    fn deserialize_habit_category() {
        assert_eq!(HabitCategory::from_str("body"), Ok(HabitCategory::Body));
        assert_eq!(HabitCategory::from_str("mind"), Ok(HabitCategory::Mind));
        assert_eq!(HabitCategory::from_str("health"), Ok(HabitCategory::Health));
        assert_eq!(HabitCategory::from_str("study"), Ok(HabitCategory::Study));
        assert_eq!(
            HabitCategory::from_str("productivity"),
            Ok(HabitCategory::Productivity)
        );
        assert_eq!(
            HabitCategory::from_str("finance"),
            Ok(HabitCategory::Finance)
        );
        assert_eq!(HabitCategory::from_str("social"), Ok(HabitCategory::Social));
        assert_eq!(
            HabitCategory::from_str("abstraction"),
            Ok(HabitCategory::Abstraction)
        );
        assert_eq!(HabitCategory::from_str("other"), Ok(HabitCategory::Other));
    }

    #[test]
    fn serialize_habit_category() {
        assert_eq!(HabitCategory::Body.as_str(), "body");
        assert_eq!(HabitCategory::Mind.as_str(), "mind");
        assert_eq!(HabitCategory::Health.as_str(), "health");
        assert_eq!(HabitCategory::Study.as_str(), "study");
        assert_eq!(HabitCategory::Productivity.as_str(), "productivity");
        assert_eq!(HabitCategory::Finance.as_str(), "finance");
        assert_eq!(HabitCategory::Social.as_str(), "social");
        assert_eq!(HabitCategory::Abstraction.as_str(), "abstraction");
        assert_eq!(HabitCategory::Other.as_str(), "other");
    }
}
