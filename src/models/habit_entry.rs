use crate::{
    core::{database, serialize},
    schema::habit_entry,
};
use diesel::prelude::*;
use gtk::{glib, prelude::*, subclass::prelude::*};

pub struct UpdateHabitEntry {
    user_id: i32,
    habit_id: i32,
    note: Option<String>,
    value: i32,
}

#[derive(Insertable)]
#[diesel(table_name = habit_entry)]
struct NewHabitEntry {
    user_id: i32,
    habit_id: i32,
    entry_time: String,
    note: Option<String>,
    value: i32,
}

#[derive(Queryable, Identifiable)]
#[diesel(table_name = habit_entry)]
struct DieselHabitEntry {
    id: i32,
    user_id: i32,
    habit_id: i32,
    entry_time: String,
    note: Option<String>,
    value: i32,
}
#[doc(hidden)]
mod imp {
    use super::*;
    use std::cell::{Cell, RefCell};

    #[derive(glib::Properties, serde::Deserialize, serde::Serialize, Debug)]
    #[properties(wrapper_type = super::HabitEntry)]
    pub struct HabitEntry {
        #[property(get, set, construct)]
        pub id: Cell<u32>,
        #[property(get, set)]
        pub user_id: Cell<u32>,
        #[property(get, set)]
        pub habit_id: Cell<u32>,
        #[property(get, set)]
        #[serde(serialize_with = "serialize::serialize_refcell_datetime")]
        #[serde(deserialize_with = "serialize::deserialize_refcell_datetime")]
        pub entry_time: RefCell<glib::DateTime>,
        #[property(get, set, nullable)]
        pub note: RefCell<Option<String>>,
        #[property(get, set)]
        pub value: Cell<i32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for HabitEntry {
        const NAME: &'static str = "HabitEntry";
        type Type = super::HabitEntry;

        fn new() -> Self {
            Self {
                id: Cell::default(),
                user_id: Cell::new(0),
                habit_id: Cell::new(0),
                entry_time: RefCell::new(glib::DateTime::now_local().unwrap()),
                note: RefCell::new(None),
                value: Cell::new(0),
            }
        }
    }

    impl ObjectImpl for HabitEntry {
        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            self.derived_property(id, pspec)
        }
    }
}

glib::wrapper! {
    pub struct HabitEntry(ObjectSubclass<imp::HabitEntry>);
}

impl HabitEntry {
    pub fn new(
        id: u32,
        user_id: u32,
        habit_id: u32,
        entry_time: glib::DateTime,
        note: Option<String>,
        value: i32,
    ) -> Result<HabitEntry, glib::BoolError> {
        let habit_entry = glib::Object::builder::<Self>()
            .property("id", id)
            .property("user-id", user_id)
            .property("habit-id", habit_id)
            .property("entry-time", entry_time)
            .property("note", note)
            .property("value", value)
            .build();

        Ok(habit_entry)
    }

    pub fn create(
        user_id: u32,
        habit_id: u32,
        note: Option<String>,
        value: i32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let db = database::connection();
        let mut conn = db.get()?;
        let curr_time = glib::DateTime::now_local()
            .unwrap()
            .format_iso8601()
            .unwrap()
            .to_string();

        let new_habit = NewHabitEntry {
            user_id: user_id as i32,
            habit_id: habit_id as i32,
            entry_time: curr_time.clone(),
            note,
            value,
        };

        diesel::insert_into(habit_entry::table)
            .values(&new_habit)
            .execute(&mut conn)?;

        habit_entry::table
            .order(habit_entry::columns::id.desc())
            .first::<DieselHabitEntry>(&mut conn)
            .map_err(From::from)
            .map(|habit_entry| {
                Self::new(
                    habit_entry.id as u32,
                    habit_entry.user_id as u32,
                    habit_entry.habit_id as u32,
                    glib::DateTime::from_iso8601(habit_entry.entry_time.as_str(), None).unwrap(),
                    habit_entry.note,
                    habit_entry.value,
                )
                .unwrap()
            })
    }

    pub fn update(
        &self,
        updated_habit_entry: &UpdateHabitEntry,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let db = database::connection();
        let mut conn = db.get()?;
        let target = habit_entry::table.filter(habit_entry::columns::id.eq(self.id() as i32));
        diesel::update(target)
            .set((
                habit_entry::columns::user_id.eq(updated_habit_entry.user_id),
                habit_entry::columns::habit_id.eq(updated_habit_entry.habit_id),
                habit_entry::columns::note.eq(updated_habit_entry.note.clone()),
                habit_entry::columns::value.eq(updated_habit_entry.value),
            ))
            .execute(&mut conn)?;

        self.set_user_id(updated_habit_entry.user_id as u32);
        self.set_habit_id(updated_habit_entry.habit_id as u32);
        self.set_note(updated_habit_entry.note.clone());
        self.set_value(updated_habit_entry.value);

        Ok(())
    }

    pub fn find(id: u32) -> Result<Self, Box<dyn std::error::Error>> {
        let db = database::connection();
        let mut conn = db.get()?;
        habit_entry::table
            .filter(habit_entry::columns::id.eq(id as i32))
            .first::<DieselHabitEntry>(&mut conn)
            .map_err(From::from)
            .map(|habit_entry| {
                Self::new(
                    habit_entry.id as u32,
                    habit_entry.user_id as u32,
                    habit_entry.habit_id as u32,
                    glib::DateTime::from_iso8601(habit_entry.entry_time.as_str(), None).unwrap(),
                    habit_entry.note,
                    habit_entry.value,
                )
                .unwrap()
            })
    }

    pub fn find_all() -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let db = database::connection();
        let mut conn = db.get()?;
        habit_entry::table
            .order(habit_entry::columns::id.desc())
            .load::<DieselHabitEntry>(&mut conn)
            .map_err(From::from)
            .map(|habits_entry| {
                habits_entry
                    .into_iter()
                    .map(|habit_entry| {
                        Self::new(
                            habit_entry.id as u32,
                            habit_entry.user_id as u32,
                            habit_entry.habit_id as u32,
                            glib::DateTime::from_iso8601(habit_entry.entry_time.as_str(), None)
                                .unwrap(),
                            habit_entry.note,
                            habit_entry.value,
                        )
                        .unwrap()
                    })
                    .collect()
            })
    }

    pub fn delete(&self) -> Result<(), Box<dyn std::error::Error>> {
        let db = database::connection();
        let mut conn = db.get()?;
        diesel::delete(habit_entry::table.filter(habit_entry::columns::id.eq(self.id() as i32)))
            .execute(&mut conn)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_habit_entry() {
        let habit_entry = HabitEntry::create(1, 1, Some("test".to_string()), 1);
        assert!(habit_entry.is_ok());

        let habit_entry = habit_entry.unwrap();
        assert_eq!(habit_entry.user_id(), 1);
        assert_eq!(habit_entry.habit_id(), 1);
        assert_eq!(habit_entry.note(), Some("test".to_string()));
        assert_eq!(habit_entry.value(), 1);

        let habit_entry_search = HabitEntry::find(habit_entry.id()).unwrap();
        assert_eq!(habit_entry_search.user_id(), 1);
        assert_eq!(habit_entry_search.habit_id(), 1);
        assert_eq!(habit_entry_search.note(), Some("test".to_string()));
        assert_eq!(habit_entry_search.value(), 1);

        let habit_entry_search_all = HabitEntry::find_all().unwrap();
        assert_eq!(habit_entry_search_all.len(), 1);

        let updated_habit_entry = UpdateHabitEntry {
            user_id: 2,
            habit_id: 2,
            note: Some("test2".to_string()),
            value: 2,
        };

        assert!(habit_entry.update(&updated_habit_entry).is_ok());
        assert_eq!(habit_entry.user_id(), 2);
        assert_eq!(habit_entry.habit_id(), 2);
        assert_eq!(habit_entry.note(), Some("test2".to_string()));
        assert_eq!(habit_entry.value(), 2);

        assert!(habit_entry.delete().is_ok());
    }
}
