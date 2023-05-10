use gtk::glib;

/// A [DurationKind] is a type of duration.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, strum::EnumString, strum::AsRefStr,
)]
#[strum(serialize_all = "snake_case")]
pub enum DurationKind {
    Daily,
    Weekly,
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

pub trait ToDuration {
    fn to_duration(&self, quantity: f64) -> std::time::Duration;
}

impl ToDuration for DurationKind {
    fn to_duration(&self, quantity: f64) -> std::time::Duration {
        match self {
            DurationKind::Daily => std::time::Duration::from_secs((quantity * 86400.0) as u64),
            DurationKind::Weekly => std::time::Duration::from_secs((quantity * 604800.0) as u64),
            DurationKind::Monthly => std::time::Duration::from_secs((quantity * 2592000.0) as u64),
        }
    }
}

pub trait FromDuration {
    fn from_duration(&self, duration: std::time::Duration) -> f64;
}

impl FromDuration for DurationKind {
    fn from_duration(&self, duration: std::time::Duration) -> f64 {
        match self {
            DurationKind::Daily => duration.as_secs_f64() / 86400.0,
            DurationKind::Weekly => duration.as_secs_f64() / 604800.0,
            DurationKind::Monthly => duration.as_secs_f64() / 2592000.0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{DurationKind, FromDuration, ToDuration};
    use std::time::Duration;

    #[test]
    fn to_duration() {
        assert_eq!(
            DurationKind::Daily.to_duration(1.25),
            Duration::from_secs(108000)
        );
        assert_eq!(
            DurationKind::Weekly.to_duration(0.5),
            Duration::from_secs(302400)
        );
        assert_eq!(
            DurationKind::Monthly.to_duration(0.1),
            Duration::from_secs(259200)
        );
    }

    #[test]
    fn from_duration() {
        assert_eq!(
            DurationKind::Daily.from_duration(Duration::from_secs(108000)),
            1.25
        );
        assert_eq!(
            DurationKind::Weekly.from_duration(Duration::from_secs(302400)),
            0.5
        );
        assert_eq!(
            DurationKind::Monthly.from_duration(Duration::from_secs(259200)),
            0.1
        );
    }
}
