use gtk::glib;

/// A [TargetGoalUnit] is a type of duration.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, strum::EnumString, strum::AsRefStr,
)]
#[strum(serialize_all = "snake_case")]
pub enum TargetGoalUnit {
    Pages,
    Words,
    Minutes,
    Hours,
    Days,
    Weeks,
    Meters,
    Kilometers,
    Miles,
    HoursPerDay,
    DaysPerWeek,
    WeeksPerMonth,
    Grams,
    Kilograms,
    Pounds,
    Ounces,
    Calories,
    Unit,
}

impl Default for TargetGoalUnit {
    fn default() -> Self {
        Self::Unit
    }
}

impl glib::ToValue for TargetGoalUnit {
    fn to_value(&self) -> glib::Value {
        self.as_ref().to_value()
    }

    fn value_type(&self) -> glib::Type {
        <String as glib::StaticType>::static_type()
    }
}
