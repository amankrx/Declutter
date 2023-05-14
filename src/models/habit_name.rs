use crate::models::{Compare, Frequency, UnitSystem};
use gtk::glib;

/// A [HabitName] is a particular habit.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, strum::EnumString, strum::AsRefStr,
)]
#[strum(serialize_all = "snake_case")]
pub enum HabitName {
    Exercise,
    Meditation,
    Reading,
    Writing,
    Programming,
    Learning,
    Drawing,
    Music,
    Journaling,
    Walking,
    Running,
    Cycling,
    Swimming,
    Yoga,
    Cooking,
    Cleaning,
    Gardening,
    Socializing,
    MartialArts,
    LearningLanguage,
    Investing,
    SavingMoney,
    OnlineCourse,
    SleepTracking,
    NoSmoking,
    NoDrinking,
    NoSugar,
    NoFastFood,
    Custom,
}

impl Default for HabitName {
    fn default() -> Self {
        Self::Custom
    }
}

impl glib::ToValue for HabitName {
    fn to_value(&self) -> glib::Value {
        self.as_ref().to_value()
    }

    fn value_type(&self) -> glib::Type {
        <String as glib::StaticType>::static_type()
    }
}

impl glib::StaticType for HabitName {
    fn static_type() -> glib::Type {
        <String as glib::StaticType>::static_type()
    }
}

pub struct HabitInfo {
    pub name: String,
    pub description: String,
    pub is_suitable_for_minors: bool,
    pub icon: Option<String>,
    pub frequency: Frequency,
    pub compare: Compare,
}

impl HabitName {
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }

    pub fn info(&self) -> HabitInfo {
        match self {
            HabitName::Exercise => HabitInfo {
                name: "Exercise".to_string(),
                description: "Perform any physical activity that gets your heart rate up."
                    .to_string(),
                is_suitable_for_minors: true,
                icon: None,
                frequency: Frequency::new(None, Some(UnitSystem::Minutes), Some(30), None),
                compare: Compare::GreaterOrEqual,
            },
            HabitName::Meditation => HabitInfo {
                name: "Meditation".to_string(),
                description: "Practice mindfulness and meditation.".to_string(),
                is_suitable_for_minors: true,
                icon: None,
                frequency: Frequency::new(None, Some(UnitSystem::Minutes), Some(10), None),
                compare: Compare::GreaterOrEqual,
            },
            HabitName::Reading => HabitInfo {
                name: "Reading".to_string(),
                description: "Read any book or blog.".to_string(),
                is_suitable_for_minors: true,
                icon: None,
                frequency: Frequency::new(None, Some(UnitSystem::Minutes), Some(30), None),
                compare: Compare::GreaterOrEqual,
            },
            HabitName::Writing => HabitInfo {
                name: "Writing".to_string(),
                description: "Write a blog post, journal entry, or any other text.".to_string(),
                is_suitable_for_minors: true,
                icon: None,
                frequency: Frequency::new(None, Some(UnitSystem::Minutes), Some(30), None),
                compare: Compare::GreaterOrEqual,
            },
            HabitName::Programming => HabitInfo {
                name: "Programming".to_string(),
                description: "Write code for a project or learn a new programming language."
                    .to_string(),
                is_suitable_for_minors: true,
                icon: None,
                frequency: Frequency::new(None, Some(UnitSystem::Minutes), Some(30), None),
                compare: Compare::GreaterOrEqual,
            },
            HabitName::Learning => HabitInfo {
                name: "Learning".to_string(),
                description: "Learn a new skill or study for a test.".to_string(),
                is_suitable_for_minors: true,
                icon: None,
                frequency: Frequency::new(None, Some(UnitSystem::Minutes), Some(30), None),
                compare: Compare::GreaterOrEqual,
            },
            HabitName::Drawing => HabitInfo {
                name: "Drawing".to_string(),
                description: "Draw a picture or sketch.".to_string(),
                is_suitable_for_minors: true,
                icon: None,
                frequency: Frequency::new(None, Some(UnitSystem::Minutes), Some(30), None),
                compare: Compare::GreaterOrEqual,
            },
            HabitName::Music => HabitInfo {
                name: "Music".to_string(),
                description: "Play an instrument or sing.".to_string(),
                is_suitable_for_minors: true,
                icon: None,
                frequency: Frequency::new(None, Some(UnitSystem::Minutes), Some(30), None),
                compare: Compare::GreaterOrEqual,
            },
            HabitName::Journaling => HabitInfo {
                name: "Journaling".to_string(),
                description: "Write a journal entry or diary.".to_string(),
                is_suitable_for_minors: true,
                icon: None,
                frequency: Frequency::new(None, None, Some(1), None),
                compare: Compare::Equal,
            },
            HabitName::Walking => HabitInfo {
                name: "Walking".to_string(),
                description: "Go for a walk.".to_string(),
                is_suitable_for_minors: true,
                icon: None,
                frequency: Frequency::new(None, Some(UnitSystem::Steps), Some(10000), None),
                compare: Compare::GreaterOrEqual,
            },
            HabitName::Running => HabitInfo {
                name: "Running".to_string(),
                description: "Go for a run or jog.".to_string(),
                is_suitable_for_minors: true,
                icon: None,
                frequency: Frequency::new(None, Some(UnitSystem::Steps), Some(10000), None),
                compare: Compare::GreaterOrEqual,
            },
            HabitName::Cycling => HabitInfo {
                name: "Cycling".to_string(),
                description: "Go for a bike ride.".to_string(),
                is_suitable_for_minors: true,
                icon: None,
                frequency: Frequency::new(None, Some(UnitSystem::Kilometers), Some(5), None),
                compare: Compare::GreaterOrEqual,
            },
            HabitName::Swimming => HabitInfo {
                name: "Swimming".to_string(),
                description: "Go for a swim.".to_string(),
                is_suitable_for_minors: true,
                icon: None,
                frequency: Frequency::new(None, Some(UnitSystem::Minutes), Some(20), None),
                compare: Compare::GreaterOrEqual,
            },
            HabitName::Yoga => HabitInfo {
                name: "Yoga".to_string(),
                description: "Practice yoga or stretching.".to_string(),
                is_suitable_for_minors: true,
                icon: None,
                frequency: Frequency::new(None, Some(UnitSystem::Minutes), Some(20), None),
                compare: Compare::GreaterOrEqual,
            },
            HabitName::Cooking => HabitInfo {
                name: "Cooking".to_string(),
                description: "Cook a meal or bake something.".to_string(),
                is_suitable_for_minors: true,
                icon: None,
                frequency: Frequency::new(None, None, Some(1), None),
                compare: Compare::Equal,
            },
            HabitName::Cleaning => HabitInfo {
                name: "Cleaning".to_string(),
                description: "Clean your home or workspace.".to_string(),
                is_suitable_for_minors: true,
                icon: None,
                frequency: Frequency::new(None, None, Some(1), None),
                compare: Compare::Equal,
            },
            HabitName::Gardening => HabitInfo {
                name: "Gardening".to_string(),
                description: "Plant or tend to a garden.".to_string(),
                is_suitable_for_minors: true,
                icon: None,
                frequency: Frequency::new(None, None, Some(1), None),
                compare: Compare::Equal,
            },
            HabitName::Socializing => HabitInfo {
                name: "Socializing".to_string(),
                description: "Spend time with friends or family.".to_string(),
                is_suitable_for_minors: true,
                icon: None,
                frequency: Frequency::new(None, None, Some(1), None),
                compare: Compare::Equal,
            },
            HabitName::MartialArts => HabitInfo {
                name: "Martial Arts".to_string(),
                description: "Practice martial arts or self-defense.".to_string(),
                is_suitable_for_minors: true,
                icon: None,
                frequency: Frequency::new(None, None, Some(1), None),
                compare: Compare::Equal,
            },
            HabitName::LearningLanguage => HabitInfo {
                name: "Learning a Language".to_string(),
                description: "Learn a new language or practice one you already know.".to_string(),
                is_suitable_for_minors: true,
                icon: None,
                frequency: Frequency::new(None, None, Some(1), None),
                compare: Compare::Equal,
            },
            HabitName::Investing => HabitInfo {
                name: "Investing".to_string(),
                description: "Invest in stocks, bonds, or cryptocurrency.".to_string(),
                is_suitable_for_minors: false,
                icon: None,
                frequency: Frequency::new(None, None, Some(1), None),
                compare: Compare::Equal,
            },
            HabitName::SavingMoney => HabitInfo {
                name: "Saving Money".to_string(),
                description: "Save money for a goal or retirement.".to_string(),
                is_suitable_for_minors: false,
                icon: None,
                frequency: Frequency::new(None, None, Some(1), None),
                compare: Compare::Equal,
            },
            HabitName::OnlineCourse => HabitInfo {
                name: "Online Course".to_string(),
                description: "Take an online course or class.".to_string(),
                is_suitable_for_minors: true,
                icon: None,
                frequency: Frequency::new(None, None, Some(1), None),
                compare: Compare::Equal,
            },
            HabitName::SleepTracking => HabitInfo {
                name: "Sleep Tracking".to_string(),
                description: "Track your sleep or take a nap.".to_string(),
                is_suitable_for_minors: true,
                icon: None,
                frequency: Frequency::new(None, Some(UnitSystem::Hours), Some(8), None),
                compare: Compare::LessOrEqual,
            },
            HabitName::NoSmoking => HabitInfo {
                name: "No Smoking".to_string(),
                description: "Don't smoke or use tobacco products.".to_string(),
                is_suitable_for_minors: false,
                icon: None,
                frequency: Frequency::new(None, None, Some(0), None),
                compare: Compare::LessOrEqual,
            },
            HabitName::NoDrinking => HabitInfo {
                name: "No Drinking".to_string(),
                description: "Don't drink alcohol or use drugs.".to_string(),
                is_suitable_for_minors: false,
                icon: None,
                frequency: Frequency::new(None, None, Some(0), None),
                compare: Compare::LessOrEqual,
            },
            HabitName::NoSugar => HabitInfo {
                name: "No Sugar".to_string(),
                description: "Don't eat any sugar or sweets.".to_string(),
                is_suitable_for_minors: true,
                icon: None,
                frequency: Frequency::new(None, None, Some(0), None),
                compare: Compare::LessOrEqual,
            },
            HabitName::NoFastFood => HabitInfo {
                name: "No Fast Food".to_string(),
                description: "Don't eat any fast food or junk food.".to_string(),
                is_suitable_for_minors: true,
                icon: None,
                frequency: Frequency::new(None, None, Some(0), None),
                compare: Compare::Equal,
            },
            HabitName::Custom => HabitInfo {
                name: "Custom".to_string(),
                description: "Create a custom habit.".to_string(),
                is_suitable_for_minors: true,
                icon: None,
                frequency: Frequency::new(None, None, Some(1), None),
                compare: Compare::Equal,
            },
        }
    }

    pub fn iter() -> impl Iterator<Item = HabitName> {
        vec![
            HabitName::Exercise,
            HabitName::Meditation,
            HabitName::Reading,
            HabitName::Writing,
            HabitName::Programming,
            HabitName::Learning,
            HabitName::Drawing,
            HabitName::Music,
            HabitName::Journaling,
            HabitName::Walking,
            HabitName::Running,
            HabitName::Cycling,
            HabitName::Swimming,
            HabitName::Yoga,
            HabitName::Cooking,
            HabitName::Cleaning,
            HabitName::Gardening,
            HabitName::Socializing,
            HabitName::MartialArts,
            HabitName::LearningLanguage,
            HabitName::Investing,
            HabitName::SavingMoney,
            HabitName::OnlineCourse,
            HabitName::SleepTracking,
            HabitName::NoSmoking,
            HabitName::NoDrinking,
            HabitName::NoSugar,
            HabitName::NoFastFood,
            HabitName::Custom,
        ]
        .into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::DurationKind;
    use std::str::FromStr;

    #[test]
    fn test_habit_name_default() {
        let default_habit = HabitName::default();
        assert_eq!(HabitName::Custom, default_habit);
    }

    #[test]
    fn test_habit_names_count() {
        assert_eq!(29, HabitName::iter().count());
    }

    #[test]
    fn test_default_habit_name() {
        let default_habit = HabitName::default();
        assert_eq!(HabitName::Custom, default_habit);
    }

    #[test]
    fn deserialize_habit_name() {
        assert_eq!(HabitName::from_str("exercise"), Ok(HabitName::Exercise));
        assert_eq!(HabitName::from_str("journaling"), Ok(HabitName::Journaling));
        assert_eq!(
            HabitName::from_str("no_fast_food"),
            Ok(HabitName::NoFastFood)
        );
        assert_eq!(HabitName::from_str("custom"), Ok(HabitName::Custom));
        assert!(HabitName::from_str("invalid").is_err());
    }

    #[test]
    fn serialize_habit_name() {
        assert_eq!(HabitName::Exercise.as_str(), "exercise");
        assert_eq!(HabitName::Journaling.as_str(), "journaling");
        assert_eq!(HabitName::NoFastFood.as_str(), "no_fast_food");
        assert_eq!(HabitName::Custom.as_str(), "custom");
    }

    #[test]
    fn test_habit_info() {
        let habit = HabitName::Exercise;
        let habit_info = habit.info();

        assert_eq!(habit_info.name, "Exercise");
        assert_eq!(
            habit_info.description,
            "Perform any physical activity that gets your heart rate up."
        );
        assert_eq!(habit_info.is_suitable_for_minors, true);
        assert_eq!(habit_info.icon, None);

        let frequency = habit_info.frequency;
        assert_eq!(frequency.duration_kind, DurationKind::Daily);
        assert_eq!(frequency.unit, UnitSystem::Minutes);
        assert_eq!(frequency.target_value, 30);
        assert_eq!(frequency.weekdays, None);

        assert_eq!(habit_info.compare, Compare::GreaterOrEqual);
    }

    #[test]
    fn test_habit_info_journalling() {
        let habit = HabitName::Journaling;
        let habit_info = habit.info();

        assert_eq!(habit_info.name, "Journaling");
        assert_eq!(habit_info.description, "Write a journal entry or diary.");
        assert_eq!(habit_info.is_suitable_for_minors, true);
        assert_eq!(habit_info.icon, None);

        let frequency = habit_info.frequency;
        assert_eq!(frequency.duration_kind, DurationKind::Daily);
        assert_eq!(frequency.unit, UnitSystem::Count);
        assert_eq!(frequency.target_value, 1);
        assert_eq!(frequency.weekdays, None);

        assert!(frequency.is_one_time());
        assert_eq!(habit_info.compare, Compare::Equal);
    }
}
