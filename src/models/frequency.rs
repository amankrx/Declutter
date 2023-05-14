use crate::models::{DurationKind, UnitSystem, Weekday};
pub struct Frequency {
    pub duration_kind: DurationKind,
    pub unit: UnitSystem,
    pub target_value: u32,
    pub weekdays: Option<Vec<Weekday>>,
}

impl Default for Frequency {
    fn default() -> Self {
        Self {
            duration_kind: DurationKind::Daily,
            unit: UnitSystem::Count,
            target_value: 1,
            weekdays: Some(
                [
                    Weekday::Monday,
                    Weekday::Tuesday,
                    Weekday::Wednesday,
                    Weekday::Thursday,
                    Weekday::Friday,
                    Weekday::Saturday,
                    Weekday::Sunday,
                ]
                .to_vec(),
            ),
        }
    }
}

impl Frequency {
    pub fn new(
        duration_kind: Option<DurationKind>,
        unit: Option<UnitSystem>,
        target_value: Option<u32>,
        weekdays: Option<Vec<Weekday>>,
    ) -> Self {
        Self {
            duration_kind: duration_kind.unwrap_or(DurationKind::Daily),
            unit: unit.unwrap_or(UnitSystem::Count),
            target_value: target_value.unwrap_or(1),
            // If duration_kind is not daily, then weekdays must be None. If duration_kind is daily, then weekdays must be Some() default.
            weekdays: if duration_kind == Some(DurationKind::Daily) {
                Some(
                    weekdays
                        .unwrap_or(
                            [
                                Weekday::Monday,
                                Weekday::Tuesday,
                                Weekday::Wednesday,
                                Weekday::Thursday,
                                Weekday::Friday,
                                Weekday::Saturday,
                                Weekday::Sunday,
                            ]
                            .to_vec(),
                        )
                        .to_vec(),
                )
            } else {
                None
            },
        }
    }

    pub fn is_one_time(&self) -> bool {
        self.target_value == 1
    }

    pub fn is_abstraction(&self) -> bool {
        self.target_value == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let freq = Frequency::default();
        assert_eq!(freq.duration_kind, DurationKind::Daily);
        assert_eq!(freq.unit, UnitSystem::Count);
        assert_eq!(freq.target_value, 1);
        assert_eq!(
            freq.weekdays,
            Some(vec![
                Weekday::Monday,
                Weekday::Tuesday,
                Weekday::Wednesday,
                Weekday::Thursday,
                Weekday::Friday,
                Weekday::Saturday,
                Weekday::Sunday,
            ])
        );
    }

    #[test]
    fn test_new_monthly() {
        let freq = Frequency::new(
            Some(DurationKind::Monthly),
            Some(UnitSystem::Meters),
            Some(5),
            None,
        );
        assert_eq!(freq.duration_kind, DurationKind::Monthly);
        assert_eq!(freq.unit, UnitSystem::Meters);
        assert_eq!(freq.target_value, 5);
        assert_eq!(freq.weekdays, None);
    }

    #[test]
    fn test_new_daily() {
        let freq = Frequency::new(
            Some(DurationKind::Daily),
            Some(UnitSystem::Meters),
            Some(5),
            None,
        );
        assert_eq!(freq.duration_kind, DurationKind::Daily);
        assert_eq!(freq.unit, UnitSystem::Meters);
        assert_eq!(freq.target_value, 5);
        assert_eq!(
            freq.weekdays,
            Some(vec![
                Weekday::Monday,
                Weekday::Tuesday,
                Weekday::Wednesday,
                Weekday::Thursday,
                Weekday::Friday,
                Weekday::Saturday,
                Weekday::Sunday,
            ])
        );
    }

    #[test]
    fn test_new_daily_with_weekdays() {
        let freq = Frequency::new(
            Some(DurationKind::Daily),
            Some(UnitSystem::Meters),
            Some(5),
            Some(vec![
                Weekday::Monday,
                Weekday::Wednesday,
                Weekday::Thursday,
                Weekday::Sunday,
            ]),
        );
        assert_eq!(freq.duration_kind, DurationKind::Daily);
        assert_eq!(freq.unit, UnitSystem::Meters);
        assert_eq!(freq.target_value, 5);
        assert_eq!(
            freq.weekdays,
            Some(vec![
                Weekday::Monday,
                Weekday::Wednesday,
                Weekday::Thursday,
                Weekday::Sunday,
            ])
        );
    }

    #[test]
    fn test_is_one_time() {
        let freq = Frequency::new(
            Some(DurationKind::Daily),
            Some(UnitSystem::Count),
            Some(1),
            None,
        );
        assert!(freq.is_one_time());
        let freq = Frequency::new(
            Some(DurationKind::Daily),
            Some(UnitSystem::Meters),
            Some(5),
            None,
        );
        assert!(!freq.is_one_time());
    }

    #[test]
    fn test_is_abstraction() {
        let freq = Frequency::new(
            Some(DurationKind::Daily),
            Some(UnitSystem::Count),
            Some(0),
            None,
        );
        assert!(freq.is_abstraction());
        let freq = Frequency::new(
            Some(DurationKind::Daily),
            Some(UnitSystem::Meters),
            Some(5),
            None,
        );
        assert!(!freq.is_abstraction());
    }
}
