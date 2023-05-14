use crate::models::{habit_category::HabitCategory, habit_name::HabitName};
use std::collections::HashMap;

pub struct HabitCategoryMap {
    pub categories_per_habit: HashMap<HabitName, Vec<HabitCategory>>,
    pub habits_per_category: HashMap<HabitCategory, Vec<HabitName>>,
}

impl Default for HabitCategoryMap {
    fn default() -> Self {
        Self::default()
    }
}

impl HabitCategoryMap {
    pub fn new() -> Self {
        let categories_per_habit = HashMap::new();
        let habits_per_category = HashMap::new();

        Self {
            categories_per_habit,
            habits_per_category,
        }
    }

    pub fn default() -> Self {
        let mut categories_per_habit = HashMap::new();
        let mut habits_per_category = HashMap::new();
        let body_habits = vec![HabitName::Exercise, HabitName::Walking, HabitName::Running];
        let mind_habits = vec![HabitName::Meditation, HabitName::Reading];
        let health_habits = vec![
            HabitName::NoSmoking,
            HabitName::NoDrinking,
            HabitName::NoSugar,
            HabitName::NoFastFood,
        ];
        let study_habits = vec![
            HabitName::Learning,
            HabitName::OnlineCourse,
            HabitName::LearningLanguage,
        ];
        let productivity_habits = vec![HabitName::Programming, HabitName::Writing];
        let finance_habits = vec![HabitName::Investing, HabitName::SavingMoney];
        let social_habits = vec![HabitName::Socializing];
        let abstraction_habits = vec![HabitName::Drawing, HabitName::Music, HabitName::Journaling];
        let other_habits = vec![
            HabitName::Cooking,
            HabitName::Cleaning,
            HabitName::Gardening,
            HabitName::Swimming,
            HabitName::Yoga,
        ];

        for &category in &[
            HabitCategory::Body,
            HabitCategory::Mind,
            HabitCategory::Health,
            HabitCategory::Study,
            HabitCategory::Productivity,
            HabitCategory::Finance,
            HabitCategory::Social,
            HabitCategory::Abstraction,
            HabitCategory::Other,
        ] {
            habits_per_category.insert(category, Vec::new());
        }

        for habit_name in body_habits
            .iter()
            .chain(mind_habits.iter())
            .chain(health_habits.iter())
            .chain(study_habits.iter())
            .chain(productivity_habits.iter())
            .chain(finance_habits.iter())
            .chain(social_habits.iter())
            .chain(abstraction_habits.iter())
            .chain(other_habits.iter())
        {
            categories_per_habit
                .entry(*habit_name)
                .or_insert_with(Vec::new);
        }

        for habit_name in body_habits {
            categories_per_habit
                .entry(habit_name)
                .or_default()
                .push(HabitCategory::Body);
            habits_per_category
                .entry(HabitCategory::Body)
                .or_default()
                .push(habit_name);
        }

        for habit_name in mind_habits {
            categories_per_habit
                .entry(habit_name)
                .or_default()
                .push(HabitCategory::Mind);
            habits_per_category
                .entry(HabitCategory::Mind)
                .or_default()
                .push(habit_name);
        }

        for habit_name in health_habits {
            categories_per_habit
                .entry(habit_name)
                .or_default()
                .push(HabitCategory::Health);
            habits_per_category
                .entry(HabitCategory::Health)
                .or_default()
                .push(habit_name);
        }

        for habit_name in study_habits {
            categories_per_habit
                .entry(habit_name)
                .or_default()
                .push(HabitCategory::Study);
            habits_per_category
                .entry(HabitCategory::Study)
                .or_default()
                .push(habit_name);
        }

        for habit_name in productivity_habits {
            categories_per_habit
                .entry(habit_name)
                .or_default()
                .push(HabitCategory::Productivity);
            habits_per_category
                .entry(HabitCategory::Productivity)
                .or_default()
                .push(habit_name);
        }

        for habit_name in finance_habits {
            categories_per_habit
                .entry(habit_name)
                .or_default()
                .push(HabitCategory::Finance);
            habits_per_category
                .entry(HabitCategory::Finance)
                .or_default()
                .push(habit_name);
        }

        for habit_name in social_habits {
            categories_per_habit
                .entry(habit_name)
                .or_default()
                .push(HabitCategory::Social);
            habits_per_category
                .entry(HabitCategory::Social)
                .or_default()
                .push(habit_name);
        }

        for habit_name in abstraction_habits {
            categories_per_habit
                .entry(habit_name)
                .or_default()
                .push(HabitCategory::Abstraction);
            habits_per_category
                .entry(HabitCategory::Abstraction)
                .or_default()
                .push(habit_name);
        }

        for habit_name in other_habits {
            categories_per_habit
                .entry(habit_name)
                .or_default()
                .push(HabitCategory::Other);
            habits_per_category
                .entry(HabitCategory::Other)
                .or_default()
                .push(habit_name);
        }

        Self {
            categories_per_habit,
            habits_per_category,
        }
    }

    pub fn add(&mut self, habit_name: HabitName, habit_category: HabitCategory) {
        if !self.categories_per_habit.contains_key(&habit_name) {
            self.categories_per_habit
                .insert(habit_name, vec![habit_category]);
        } else {
            let categories = self.categories_per_habit.get_mut(&habit_name).unwrap();
            if !categories.contains(&habit_category) {
                categories.push(habit_category);
            }
        }

        if !self.habits_per_category.contains_key(&habit_category) {
            self.habits_per_category
                .insert(habit_category, vec![habit_name]);
        } else {
            let habits = self.habits_per_category.get_mut(&habit_category).unwrap();
            if !habits.contains(&habit_name) {
                habits.push(habit_name);
            }
        }
    }

    pub fn insert_habit_with_categories(
        &mut self,
        habit_name: HabitName,
        habit_categories: Vec<HabitCategory>,
    ) {
        self.categories_per_habit
            .insert(habit_name, habit_categories.clone());

        for category in habit_categories {
            self.habits_per_category
                .entry(category)
                .or_insert_with(Vec::new)
                .push(habit_name.clone());
        }
    }

    pub fn insert_category_with_habits(
        &mut self,
        habit_category: HabitCategory,
        habit_names: Vec<HabitName>,
    ) {
        self.habits_per_category
            .insert(habit_category, habit_names.clone());

        for habit_name in habit_names {
            self.categories_per_habit
                .entry(habit_name)
                .or_insert_with(Vec::new)
                .push(habit_category.clone());
        }
    }

    pub fn remove(&mut self, habit_name: &HabitName, habit_category: HabitCategory) {
        if let Some(categories) = self.categories_per_habit.get_mut(habit_name) {
            categories.retain(|&category| category != habit_category);
        }

        if let Some(habit_names) = self.habits_per_category.get_mut(&habit_category) {
            habit_names.retain(|&name| name != *habit_name);
        }
    }

    pub fn get_categories(&self, habit_name: &HabitName) -> Option<&Vec<HabitCategory>> {
        self.categories_per_habit.get(habit_name)
    }

    pub fn get_habits(&self, category: &HabitCategory) -> Option<&Vec<HabitName>> {
        self.habits_per_category.get(category)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::iter::Iterator;

    #[test]
    fn test_default() {
        let habit_category_map = HabitCategoryMap::default();
        assert_eq!(habit_category_map.categories_per_habit.len(), 25);
        assert_eq!(habit_category_map.habits_per_category.len(), 9);
    }

    #[test]
    fn test_new() {
        let habit_category_map = HabitCategoryMap::new();
        assert_eq!(habit_category_map.categories_per_habit.len(), 0);
        assert_eq!(habit_category_map.habits_per_category.len(), 0);
    }

    #[test]
    fn test_categories_per_habit() {
        let habit_category_map = HabitCategoryMap::default();
        assert_eq!(
            habit_category_map
                .categories_per_habit
                .get(&HabitName::Exercise)
                .unwrap(),
            &[HabitCategory::Body]
        );
        assert_eq!(
            habit_category_map
                .categories_per_habit
                .get(&HabitName::Meditation)
                .unwrap(),
            &[HabitCategory::Mind]
        );
    }

    #[test]
    fn test_habits_per_category() {
        let habit_category_map = HabitCategoryMap::default();
        assert_eq!(
            habit_category_map
                .habits_per_category
                .get(&HabitCategory::Body)
                .unwrap(),
            &[HabitName::Exercise, HabitName::Walking, HabitName::Running]
        );
        assert_eq!(
            habit_category_map
                .habits_per_category
                .get(&HabitCategory::Mind)
                .unwrap(),
            &[HabitName::Meditation, HabitName::Reading]
        );
        assert_eq!(
            habit_category_map
                .habits_per_category
                .get(&HabitCategory::Health)
                .unwrap(),
            &[
                HabitName::NoSmoking,
                HabitName::NoDrinking,
                HabitName::NoSugar,
                HabitName::NoFastFood
            ]
        );
        assert_eq!(
            habit_category_map
                .habits_per_category
                .get(&HabitCategory::Other)
                .unwrap(),
            &[
                HabitName::Cooking,
                HabitName::Cleaning,
                HabitName::Gardening,
                HabitName::Swimming,
                HabitName::Yoga
            ]
        );
    }

    #[test]
    fn test_add_habit_category() {
        let mut habit_category_map = HabitCategoryMap::default();
        habit_category_map.add(HabitName::Exercise, HabitCategory::Body);
        habit_category_map.add(HabitName::Exercise, HabitCategory::Mind);
        habit_category_map.add(HabitName::Exercise, HabitCategory::Health);
        habit_category_map.add(HabitName::Exercise, HabitCategory::Study);
        habit_category_map.add(HabitName::Exercise, HabitCategory::Productivity);
        habit_category_map.add(HabitName::Exercise, HabitCategory::Finance);
        habit_category_map.add(HabitName::Exercise, HabitCategory::Social);
        habit_category_map.add(HabitName::Exercise, HabitCategory::Abstraction);
        habit_category_map.add(HabitName::Exercise, HabitCategory::Other);

        assert!(habit_category_map
            .categories_per_habit
            .get(&HabitName::Exercise)
            .unwrap()
            .iter()
            .all(|&category| {
                vec![
                    HabitCategory::Body,
                    HabitCategory::Mind,
                    HabitCategory::Health,
                    HabitCategory::Study,
                    HabitCategory::Productivity,
                    HabitCategory::Finance,
                    HabitCategory::Social,
                    HabitCategory::Abstraction,
                    HabitCategory::Other,
                ]
                .contains(&category)
            }));

        assert!(habit_category_map
            .habits_per_category
            .get(&HabitCategory::Health)
            .unwrap()
            .iter()
            .all(|&habit_name| {
                vec![
                    HabitName::Exercise,
                    HabitName::NoSmoking,
                    HabitName::NoDrinking,
                    HabitName::NoSugar,
                    HabitName::NoFastFood,
                ]
                .contains(&habit_name)
            }));

        assert!(habit_category_map
            .habits_per_category
            .get(&HabitCategory::Finance)
            .unwrap()
            .iter()
            .all(|&habit_name| {
                vec![
                    HabitName::Exercise,
                    HabitName::Investing,
                    HabitName::SavingMoney,
                ]
                .contains(&habit_name)
            }));

        assert!(habit_category_map
            .habits_per_category
            .get(&HabitCategory::Social)
            .unwrap()
            .iter()
            .all(|&habit_name| {
                vec![HabitName::Exercise, HabitName::Socializing].contains(&habit_name)
            }));
    }

    #[test]
    fn test_insert_habit_with_categories() {
        let mut hcm = HabitCategoryMap::new();
        let habit = HabitName::Exercise;
        let categories = vec![HabitCategory::Body, HabitCategory::Productivity];
        hcm.insert_habit_with_categories(habit, categories.clone());
        assert_eq!(hcm.categories_per_habit.get(&habit).unwrap(), &categories);
        assert_eq!(
            hcm.habits_per_category.get(&HabitCategory::Body).unwrap(),
            &[habit]
        );
        assert_eq!(
            hcm.habits_per_category
                .get(&HabitCategory::Productivity)
                .unwrap(),
            &[habit]
        );
    }

    #[test]
    fn test_insert_category_with_habits() {
        let mut hcm = HabitCategoryMap::new();
        let category = HabitCategory::Health;
        let habits = vec![
            HabitName::NoSmoking,
            HabitName::NoDrinking,
            HabitName::NoSugar,
            HabitName::NoFastFood,
        ];
        hcm.insert_category_with_habits(category, habits.clone());
        let habits_set: HashSet<_> = habits.iter().collect();
        assert_eq!(
            hcm.habits_per_category
                .get(&category)
                .unwrap()
                .iter()
                .collect::<HashSet<_>>(),
            habits_set
        );
        for habit in habits {
            assert_eq!(
                hcm.categories_per_habit.get(&habit).unwrap(),
                &vec![category]
            );
        }
    }

    #[test]
    fn test_remove() {
        let mut habit_category_map = HabitCategoryMap::default();
        habit_category_map.remove(&HabitName::Exercise, HabitCategory::Body);
        assert_eq!(
            habit_category_map
                .categories_per_habit
                .get(&HabitName::Exercise)
                .unwrap(),
            &[]
        );
        assert_eq!(
            habit_category_map
                .habits_per_category
                .get(&HabitCategory::Body)
                .unwrap(),
            &[HabitName::Walking, HabitName::Running]
        );
    }

    #[test]
    fn test_get_categories() {
        let habit_category_map = HabitCategoryMap::default();
        assert_eq!(
            habit_category_map
                .get_categories(&HabitName::Exercise)
                .unwrap(),
            &[HabitCategory::Body]
        );
        assert_eq!(
            habit_category_map
                .get_categories(&HabitName::NoSmoking)
                .unwrap(),
            &[HabitCategory::Health]
        );
        assert_eq!(
            habit_category_map
                .get_categories(&HabitName::Learning)
                .unwrap(),
            &[HabitCategory::Study]
        );
        assert_eq!(
            habit_category_map
                .get_categories(&HabitName::Cooking)
                .unwrap(),
            &[HabitCategory::Other]
        );
    }

    #[test]
    fn test_get_habits() {
        let habit_category_map = HabitCategoryMap::default();
        assert_eq!(
            habit_category_map.get_habits(&HabitCategory::Body).unwrap(),
            &[HabitName::Exercise, HabitName::Walking, HabitName::Running]
        );
        assert_eq!(
            habit_category_map
                .get_habits(&HabitCategory::Study)
                .unwrap(),
            &[
                HabitName::Learning,
                HabitName::OnlineCourse,
                HabitName::LearningLanguage
            ]
        );
        assert_eq!(
            habit_category_map
                .get_habits(&HabitCategory::Productivity)
                .unwrap(),
            &[HabitName::Programming, HabitName::Writing]
        );
        assert_eq!(
            habit_category_map
                .get_habits(&HabitCategory::Abstraction)
                .unwrap(),
            &[HabitName::Drawing, HabitName::Music, HabitName::Journaling]
        );
        assert_eq!(
            habit_category_map
                .get_habits(&HabitCategory::Other)
                .unwrap(),
            &[
                HabitName::Cooking,
                HabitName::Cleaning,
                HabitName::Gardening,
                HabitName::Swimming,
                HabitName::Yoga
            ]
        );
    }
}
