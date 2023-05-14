use gtk::glib;

/// Currently, I've used just u64 and f64 for the values. Later on, we should use appropriate
/// types for each unit system. For eg: for Distance we can use uom::si::f64::Length.
/// A [UnitSystem] is a type of duration.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, strum::EnumString, strum::AsRefStr,
)]
#[strum(serialize_all = "snake_case")]
pub enum UnitSystem {
    // General Count
    Count,
    // Time
    Minutes,
    Hours,
    Days,
    // Pages
    Pages,
    Words,
    // Finance
    Currency,
    // Volume
    Milliliters,
    Liters,
    Gallons,
    // Distance
    Meters,
    Kilometers,
    Yards,
    Miles,
    // Weight
    Grams,
    Kilograms,
    Pounds,
    Ounces,
    // Calories
    Calories,
    Steps,
    // Unit
    Unit,
}

impl Default for UnitSystem {
    fn default() -> Self {
        Self::Unit
    }
}

impl glib::ToValue for UnitSystem {
    fn to_value(&self) -> glib::Value {
        self.as_ref().to_value()
    }

    fn value_type(&self) -> glib::Type {
        <String as glib::StaticType>::static_type()
    }
}

impl glib::StaticType for UnitSystem {
    fn static_type() -> glib::Type {
        <String as glib::StaticType>::static_type()
    }
}

// Implement an iterator for UnitSystem
impl IntoIterator for UnitSystem {
    type Item = UnitSystem;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
}

impl UnitSystem {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_unit_system_default() {
        let unit_system = UnitSystem::default();
        assert_eq!(unit_system, UnitSystem::Unit);
    }

    #[test]
    fn deserialize_unit_system() {
        assert_eq!(UnitSystem::from_str("count"), Ok(UnitSystem::Count));
        assert_eq!(UnitSystem::from_str("minutes"), Ok(UnitSystem::Minutes));
        assert!(UnitSystem::from_str("invalid").is_err());
    }

    #[test]
    fn serialize_duration_kind() {
        assert_eq!(UnitSystem::Count.as_str(), "count");
        assert_eq!(UnitSystem::Minutes.as_str(), "minutes");
    }
}
