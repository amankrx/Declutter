use crate::{
    core::{database, serialize},
    models::{Frequency, HabitCategory, HabitName},
    schema::habit,
};
use diesel::prelude::*;
use gtk::{glib, prelude::*, subclass::prelude::*};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub struct UpdateHabit {
    user_id: i32,
    name: String,
    description: Option<String>,
    categories: Option<String>,
    icon: Option<String>,
    frequency: String,
    reminder_times: Option<String>,
    note: Option<String>,
    archived: i32,
    archived_date: Option<String>,
    archived_reason: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = habit)]
struct NewHabit {
    user_id: i32,
    name: String,
    description: Option<String>,
    categories: Option<String>,
    icon: Option<String>,
    frequency: String,
    created_at: String,
    updated_at: Option<String>,
    reminder_times: Option<String>,
    note: Option<String>,
    archived: i32,
    archived_date: Option<String>,
    archived_reason: Option<String>,
}

#[derive(Queryable, Identifiable)]
#[diesel(table_name = habit)]
struct DieselHabit {
    id: i32,
    user_id: i32,
    name: String,
    description: Option<String>,
    categories: Option<String>,
    icon: Option<String>,
    frequency: String,
    created_at: String,
    updated_at: Option<String>,
    reminder_times: Option<String>,
    note: Option<String>,
    archived: i32,
    archived_date: Option<String>,
    archived_reason: Option<String>,
}

#[derive(Clone, glib::Boxed, PartialEq, Eq, Deserialize, Serialize)]
#[boxed_type(name = "Categories")]
pub struct Categories(pub Option<Vec<HabitCategory>>);

impl Default for Categories {
    fn default() -> Self {
        Self(None)
    }
}

#[derive(Clone, glib::Boxed, PartialEq, Eq, Deserialize, Serialize)]
#[boxed_type(name = "reminder_times")]
pub struct ReminderTimes(
    #[serde(serialize_with = "serialize::serialize_option_vec_datetime")]
    #[serde(deserialize_with = "serialize::deserialize_option_vec_datetime")]
    Option<Vec<glib::DateTime>>,
);

impl Default for ReminderTimes {
    fn default() -> Self {
        Self(None)
    }
}

#[doc(hidden)]
mod imp {
    use glib::ParamSpecBoxed;
    use once_cell::sync::Lazy;
    use std::cell::{Cell, RefCell};

    use super::*;

    #[derive(glib::Properties, serde::Deserialize, serde::Serialize, Debug)]
    #[properties(wrapper_type = super::Habit)]
    pub struct Habit {
        #[property(get, set, construct)]
        pub id: Cell<u32>,
        #[property(get, set)]
        pub user_id: Cell<u32>,
        #[property(get, set)]
        pub name: RefCell<HabitName>,
        #[property(get, set, nullable)]
        pub description: RefCell<Option<String>>,
        pub categories: RefCell<Option<Vec<HabitCategory>>>,
        #[property(get, set, nullable)]
        pub icon: RefCell<Option<String>>,
        pub frequency: RefCell<Frequency>,
        #[property(get, set)]
        #[serde(serialize_with = "serialize::serialize_refcell_datetime")]
        #[serde(deserialize_with = "serialize::deserialize_refcell_datetime")]
        pub created_at: RefCell<glib::DateTime>,
        #[property(get, set, nullable)]
        #[serde(serialize_with = "serialize::serialize_refcell_option_datetime")]
        #[serde(deserialize_with = "serialize::deserialize_refcell_option_datetime")]
        pub updated_at: RefCell<Option<glib::DateTime>>,
        #[serde(serialize_with = "serialize::serialize_refcell_option_vec_datetime")]
        #[serde(deserialize_with = "serialize::deserialize_refcell_option_vec_datetime")]
        pub reminder_times: RefCell<Option<Vec<glib::DateTime>>>,
        #[property(get, set, nullable)]
        pub note: RefCell<Option<String>>,
        #[property(get, set)]
        pub archived: Cell<bool>,
        #[property(get, set, nullable)]
        #[serde(serialize_with = "serialize::serialize_refcell_option_datetime")]
        #[serde(deserialize_with = "serialize::deserialize_refcell_option_datetime")]
        pub archived_date: RefCell<Option<glib::DateTime>>,
        #[property(get, set, nullable)]
        pub archived_reason: RefCell<Option<String>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Habit {
        const NAME: &'static str = "Habit";
        type Type = super::Habit;

        fn new() -> Self {
            Self {
                id: Cell::default(),
                user_id: Cell::new(0),
                name: RefCell::new(HabitName::default()),
                description: RefCell::new(None),
                categories: RefCell::new(None),
                icon: RefCell::default(),
                frequency: RefCell::new(Frequency::default()),
                created_at: RefCell::new(glib::DateTime::now_local().unwrap()),
                updated_at: RefCell::new(None),
                reminder_times: RefCell::new(None),
                note: RefCell::default(),
                archived: Cell::new(false),
                archived_date: RefCell::new(None),
                archived_reason: RefCell::new(None),
            }
        }
    }

    impl ObjectImpl for Habit {
        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                let mut props = Habit::derived_properties().to_vec();
                // props.push(ParamSpecBoxed::builder::<HabitName>("name").build());
                props.push(ParamSpecBoxed::builder::<Categories>("categories").build());
                props.push(ParamSpecBoxed::builder::<Frequency>("frequency").build());
                props.push(ParamSpecBoxed::builder::<ReminderTimes>("reminder-times").build());
                props
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            match pspec.name() {
                "categories" => {
                    let categories = value
                        .get::<Categories>()
                        .expect("type conformity checked by `Object::set_property`");
                    self.categories.replace(categories.0);
                }
                "frequency" => {
                    let frequency = value
                        .get::<Frequency>()
                        .expect("type conformity checked by `Object::set_property`");
                    self.frequency.replace(frequency);
                }
                "reminder-times" => {
                    let reminder_times = value
                        .get::<ReminderTimes>()
                        .expect("type conformity checked by `Object::set_property`");
                    self.reminder_times.replace(reminder_times.0);
                }
                _ => self.derived_set_property(id, value, pspec),
            }
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.name() {
                "name" => self.name.borrow().to_value(),
                "categories" => Categories(self.categories.borrow().clone()).to_value(),
                "frequency" => self.frequency.borrow().to_value(),
                "reminder-times" => ReminderTimes(self.reminder_times.borrow().clone()).to_value(),
                _ => self.derived_property(id, pspec),
            }
        }
    }
}

glib::wrapper! {
    pub struct Habit(ObjectSubclass<imp::Habit>);
}

impl Habit {
    pub fn new(
        id: u32,
        user_id: u32,
        name: HabitName,
        description: Option<String>,
        categories: Option<Vec<HabitCategory>>,
        icon: Option<String>,
        frequency: Frequency,
        created_at: glib::DateTime,
        updated_at: Option<glib::DateTime>,
        reminder_times: Option<Vec<glib::DateTime>>,
        note: Option<String>,
        archived: bool,
        archived_date: Option<glib::DateTime>,
        archived_reason: Option<String>,
    ) -> Result<Habit, glib::BoolError> {
        let habit = glib::Object::builder::<Self>()
            .property("id", id)
            .property("user-id", user_id)
            .property("name", name)
            .property("description", description)
            .property("categories", Categories(categories))
            .property("icon", icon)
            .property("frequency", frequency)
            .property("created-at", created_at)
            .property("updated-at", updated_at)
            .property("reminder-times", ReminderTimes(reminder_times))
            .property("note", note)
            .property("archived", archived)
            .property("archived-date", archived_date)
            .property("archived-reason", archived_reason)
            .build();

        Ok(habit)
    }

    pub fn categories(&self) -> Option<Vec<HabitCategory>> {
        self.imp().categories.borrow().clone()
    }

    pub fn frequency(&self) -> Frequency {
        self.imp().frequency.borrow().clone()
    }

    pub fn reminder_times(&self) -> Option<Vec<glib::DateTime>> {
        self.imp().reminder_times.borrow().clone()
    }

    pub fn set_categories(&self, categories: Option<Vec<HabitCategory>>) {
        self.imp().categories.replace(categories);
    }

    pub fn set_frequency(&self, frequency: Frequency) {
        self.imp().frequency.replace(frequency);
    }

    pub fn set_reminder_times(&self, reminder_times: Option<Vec<glib::DateTime>>) {
        self.imp().reminder_times.replace(reminder_times);
    }

    pub fn create(
        user_id: u32,
        name: HabitName,
        description: Option<String>,
        categories: Option<Vec<HabitCategory>>,
        icon: Option<String>,
        frequency: Frequency,
        reminder_times: Option<Vec<glib::DateTime>>,
        note: Option<String>,
        archived: bool,
        archived_date: Option<glib::DateTime>,
        archived_reason: Option<String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let db = database::connection();
        let mut conn = db.get()?;

        let new_habit = NewHabit {
            user_id: user_id as i32,
            name: name.as_str().to_string(),
            description,
            categories: match categories {
                Some(c) => Some(serde_json::to_string(&c).unwrap()),
                None => None,
            },
            icon,
            frequency: serialize::serialize_struct_to_string(&frequency),
            created_at: glib::DateTime::now_local()
                .unwrap()
                .format_iso8601()
                .unwrap()
                .to_string(),
            updated_at: None,
            reminder_times: match reminder_times {
                Some(times) => Some(serialize::serialize_vec_datetime_to_string(&times)),
                None => None,
            },
            note,
            archived: archived as i32,
            archived_date: match archived_date {
                Some(date) => Some(date.format_iso8601().unwrap().to_string()),
                None => None,
            },
            archived_reason,
        };

        diesel::insert_into(habit::table)
            .values(&new_habit)
            .execute(&mut conn)?;

        habit::table
            .order(habit::columns::id.desc())
            .first::<DieselHabit>(&mut conn)
            .map_err(From::from)
            .map(|habit| {
                Self::new(
                    habit.id as u32,
                    habit.user_id as u32,
                    HabitName::from_str(&habit.name).unwrap(),
                    habit.description,
                    habit.categories.map(|c| serde_json::from_str(&c).unwrap()),
                    habit.icon,
                    serialize::deserialize_string_to_struct(&habit.frequency),
                    glib::DateTime::from_iso8601(habit.created_at.as_str(), None).unwrap(),
                    match habit.updated_at {
                        Some(date) => {
                            Some(glib::DateTime::from_iso8601(date.as_str(), None).unwrap())
                        }
                        None => None,
                    },
                    match habit.reminder_times {
                        Some(times) => Some(serialize::deserialize_string_to_vec_datetime(&times)),
                        None => None,
                    },
                    habit.note,
                    habit.archived == 1,
                    match habit.archived_date {
                        Some(date) => {
                            Some(glib::DateTime::from_iso8601(date.as_str(), None).unwrap())
                        }
                        None => None,
                    },
                    habit.archived_reason,
                )
                .unwrap()
            })
    }

    pub fn update(&self, updated_habit: &UpdateHabit) -> Result<(), Box<dyn std::error::Error>> {
        let db = database::connection();
        let mut conn = db.get()?;
        let target = habit::table.filter(habit::columns::id.eq(self.id() as i32));
        let curr_time = glib::DateTime::now_local()
            .unwrap()
            .format_iso8601()
            .unwrap()
            .to_string();
        diesel::update(target)
            .set((
                habit::columns::user_id.eq(updated_habit.user_id as i32),
                habit::columns::name.eq(updated_habit.name.clone()),
                habit::columns::description.eq(updated_habit.description.clone()),
                habit::columns::categories.eq(updated_habit.categories.clone()),
                habit::columns::icon.eq(updated_habit.icon.clone()),
                habit::columns::frequency.eq(updated_habit.frequency.clone()),
                habit::columns::updated_at.eq(curr_time.clone()),
                habit::columns::reminder_times.eq(updated_habit.reminder_times.clone()),
                habit::columns::note.eq(updated_habit.note.clone()),
                habit::columns::archived.eq(updated_habit.archived.clone() as i32),
                habit::columns::archived_date.eq(updated_habit.archived_date.clone()),
                habit::columns::archived_reason.eq(updated_habit.archived_reason.clone()),
            ))
            .execute(&mut conn)?;

        self.set_user_id(updated_habit.user_id as u32);
        self.set_name(HabitName::from_str(&updated_habit.name.as_str()).unwrap());
        self.set_description(updated_habit.description.clone());
        self.set_categories(match updated_habit.categories.clone() {
            Some(c) => Some(serde_json::from_str(&c).unwrap()),
            None => None,
        });
        self.set_icon(updated_habit.icon.clone());
        self.set_frequency(serialize::deserialize_string_to_struct(
            &updated_habit.frequency.clone(),
        ));
        self.set_updated_at(Some(glib::DateTime::from_iso8601(
            curr_time.as_str(),
            None,
        )?));
        self.set_reminder_times(match updated_habit.reminder_times.clone() {
            Some(times) => Some(serialize::deserialize_string_to_vec_datetime(&times)),
            None => None,
        });
        self.set_note(updated_habit.note.clone());
        self.set_archived(updated_habit.archived.clone() == 1);
        self.set_archived_date(match updated_habit.archived_date.clone() {
            Some(date) => Some(glib::DateTime::from_iso8601(date.as_str(), None).unwrap()),
            None => None,
        });
        self.set_archived_reason(updated_habit.archived_reason.clone());

        Ok(())
    }

    pub fn find(id: u32) -> Result<Self, Box<dyn std::error::Error>> {
        let db = database::connection();
        let mut conn = db.get()?;
        habit::table
            .filter(habit::columns::id.eq(id as i32))
            .first::<DieselHabit>(&mut conn)
            .map_err(From::from)
            .map(|habit| {
                Self::new(
                    habit.id as u32,
                    habit.user_id as u32,
                    HabitName::from_str(&habit.name).unwrap(),
                    habit.description,
                    habit.categories.map(|c| serde_json::from_str(&c).unwrap()),
                    habit.icon,
                    serialize::deserialize_string_to_struct(&habit.frequency),
                    glib::DateTime::from_iso8601(habit.created_at.as_str(), None).unwrap(),
                    match habit.updated_at {
                        Some(date) => {
                            Some(glib::DateTime::from_iso8601(date.as_str(), None).unwrap())
                        }
                        None => None,
                    },
                    match habit.reminder_times {
                        Some(times) => Some(serialize::deserialize_string_to_vec_datetime(&times)),
                        None => None,
                    },
                    habit.note,
                    habit.archived == 1,
                    match habit.archived_date {
                        Some(date) => {
                            Some(glib::DateTime::from_iso8601(date.as_str(), None).unwrap())
                        }
                        None => None,
                    },
                    habit.archived_reason,
                )
                .unwrap()
            })
    }

    pub fn find_all() -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let db = database::connection();
        let mut conn = db.get()?;
        habit::table
            .order(habit::columns::id.desc())
            .load::<DieselHabit>(&mut conn)
            .map_err(From::from)
            .map(|habits| {
                habits
                    .into_iter()
                    .map(|habit| {
                        Self::new(
                            habit.id as u32,
                            habit.user_id as u32,
                            HabitName::from_str(&habit.name).unwrap(),
                            habit.description,
                            habit.categories.map(|c| serde_json::from_str(&c).unwrap()),
                            habit.icon,
                            serialize::deserialize_string_to_struct(&habit.frequency),
                            glib::DateTime::from_iso8601(habit.created_at.as_str(), None).unwrap(),
                            match habit.updated_at {
                                Some(date) => {
                                    Some(glib::DateTime::from_iso8601(date.as_str(), None).unwrap())
                                }
                                None => None,
                            },
                            match habit.reminder_times {
                                Some(times) => {
                                    Some(serialize::deserialize_string_to_vec_datetime(&times))
                                }
                                None => None,
                            },
                            habit.note,
                            habit.archived == 1,
                            match habit.archived_date {
                                Some(date) => {
                                    Some(glib::DateTime::from_iso8601(date.as_str(), None).unwrap())
                                }
                                None => None,
                            },
                            habit.archived_reason,
                        )
                        .unwrap()
                    })
                    .collect()
            })
    }

    pub fn delete(&self) -> Result<(), Box<dyn std::error::Error>> {
        let db = database::connection();
        let mut conn = db.get()?;
        diesel::delete(habit::table.filter(habit::columns::id.eq(self.id() as i32)))
            .execute(&mut conn)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::HabitName;

    #[test]
    fn test_habit() {
        let habit = Habit::create(
            1,
            HabitName::Cleaning,
            Some("Cleaning your room".to_string()),
            Some(vec![HabitCategory::Body, HabitCategory::Mind]),
            None,
            Frequency::default(),
            None,
            None,
            false,
            None,
            None,
        )
        .unwrap();

        let habit_from_db = Habit::find(habit.id());

        assert_eq!(habit.user_id(), habit_from_db.as_ref().unwrap().user_id());
        assert_eq!(habit.name(), habit_from_db.as_ref().unwrap().name());
        assert_eq!(
            habit.description(),
            habit_from_db.as_ref().unwrap().description()
        );
        assert_eq!(
            habit.categories(),
            habit_from_db.as_ref().unwrap().categories()
        );
        assert_eq!(habit.icon(), habit_from_db.as_ref().unwrap().icon());
        assert_eq!(
            habit.frequency(),
            habit_from_db.as_ref().unwrap().frequency()
        );
        assert_eq!(
            habit.reminder_times(),
            habit_from_db.as_ref().unwrap().reminder_times()
        );
        assert_eq!(habit.note(), habit_from_db.as_ref().unwrap().note());
        assert_eq!(habit.archived(), habit_from_db.as_ref().unwrap().archived());
        assert_eq!(
            habit.archived_date(),
            habit_from_db.as_ref().unwrap().archived_date()
        );
        assert_eq!(
            habit.archived_reason(),
            habit_from_db.as_ref().unwrap().archived_reason()
        );

        // let habit_count = Habit::find_all().unwrap().len();
        // assert_eq!(habit_count, 1);

        let updated_habit = UpdateHabit {
            user_id: 2,
            name: "cleaning".to_string(),
            description: None,
            categories: None,
            icon: None,
            frequency: serialize::serialize_struct_to_string(&Frequency::default()),
            reminder_times: None,
            note: None,
            archived: 1,
            archived_date: None,
            archived_reason: None,
        };

        habit.update(&updated_habit).unwrap();

        assert_eq!(habit.user_id(), 2);
        assert_eq!(habit.name(), HabitName::Cleaning);
        assert_eq!(habit.description(), None);
        assert_eq!(habit.categories(), None);
        assert_eq!(habit.icon(), None);
        assert_eq!(habit.frequency(), Frequency::default());
        assert_eq!(habit.reminder_times(), None);
        assert_eq!(habit.note(), None);
        assert_eq!(habit.archived(), true);
        assert_eq!(habit.archived_date(), None);
        assert_eq!(habit.archived_reason(), None);

        let _ = habit.delete();
    }
}
