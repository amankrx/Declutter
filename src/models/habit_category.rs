use gtk::glib;

/// A [HabitCategory] is a type of duration.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, strum::EnumString, strum::AsRefStr,
)]
#[strum(serialize_all = "snake_case")]
pub enum HabitCategory {
    Body,
    Mind,
    Health,
    Study,
    Productivity,
    Finance,
    Social,
    Other,
}

impl Default for HabitCategory {
    fn default() -> Self {
        Self::Other
    }
}

impl glib::ToValue for HabitCategory {
    fn to_value(&self) -> glib::Value {
        self.as_ref().to_value()
    }

    fn value_type(&self) -> glib::Type {
        <String as glib::StaticType>::static_type()
    }
}
