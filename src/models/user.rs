use anyhow::Result;
use diesel::prelude::*;
use gtk::{glib, prelude::*, subclass::prelude::*};

use crate::{core::database, schema::user};

#[derive(Insertable)]
#[diesel(table_name = user)]
struct NewUser {
    name: String,
    date_of_birth: String,
    created_at: String,
}

#[derive(Queryable, Identifiable)]
#[diesel(table_name = user)]
struct DieselUser {
    id: i32,
    name: String,
    date_of_birth: String,
    created_at: String,
}

#[doc(hidden)]
mod imp {
    use glib::ParamSpecBoxed;
    use once_cell::sync::Lazy;
    use serde::{Deserialize, Serialize};
    use std::cell::{Cell, RefCell};

    use crate::core::serialize;

    use super::*;

    #[derive(glib::Properties, Serialize, Deserialize)]
    #[properties(wrapper_type = super::User)]
    pub struct User {
        #[property(get, set, construct)]
        pub id: Cell<u32>,
        #[property(get, set = Self::set_name)]
        pub name: RefCell<String>,
        #[serde(serialize_with = "serialize::serialize_refcell_datetime")]
        #[serde(deserialize_with = "serialize::deserialize_refcell_datetime")]
        pub date_of_birth: RefCell<glib::DateTime>,
        #[serde(serialize_with = "serialize::serialize_refcell_datetime")]
        #[serde(deserialize_with = "serialize::deserialize_refcell_datetime")]
        pub created_at: RefCell<glib::DateTime>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for User {
        const NAME: &'static str = "User";
        type Type = super::User;

        fn new() -> Self {
            Self {
                id: Cell::default(),
                name: RefCell::default(),
                date_of_birth: RefCell::new(glib::DateTime::now_local().unwrap()),
                created_at: RefCell::new(glib::DateTime::now_local().unwrap()),
            }
        }
    }

    impl ObjectImpl for User {
        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                let mut props = User::derived_properties().to_vec();
                props.push(ParamSpecBoxed::builder::<glib::DateTime>("date-of-birth").build());
                props.push(ParamSpecBoxed::builder::<glib::DateTime>("created-at").build());
                props
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            match pspec.name() {
                "date-of-birth" => {
                    let date_of_birth = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.date_of_birth.replace(date_of_birth);
                }
                "created-at" => {
                    let created_at = value
                        .get()
                        .expect("type conformity checked by `Object::set_property`");
                    self.created_at.replace(created_at);
                }
                _ => self.derived_set_property(id, value, pspec),
            }
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.name() {
                "date-of-birth" => self.date_of_birth.borrow().to_value(),
                "created-at" => self.created_at.borrow().to_value(),
                _ => self.derived_property(id, pspec),
            }
        }
    }

    impl User {
        fn set_name_inner(&self, id: i32, name: &str) -> Result<()> {
            let db = database::connection();
            let mut conn = db.get()?;

            let target = user::table.filter(user::columns::id.eq(id));
            diesel::update(target)
                .set(user::columns::name.eq(name))
                .execute(&mut conn)?;
            Ok(())
        }

        fn set_name(&self, name: &str) {
            match self.set_name_inner(self.obj().id() as i32, name) {
                Ok(_) => {
                    self.name.replace(name.to_owned());
                }
                Err(err) => {
                    tracing::warn!("Failed to update user name {err}");
                }
            }
        }
    }
}

glib::wrapper! {
    pub struct User(ObjectSubclass<imp::User>);
}

impl User {
    pub fn create(name: &str, date_of_birth: &str, created_at: &str) -> Result<User> {
        let db = database::connection();
        let mut conn = db.get()?;

        diesel::insert_into(user::table)
            .values(&NewUser {
                name: name.to_owned(),
                date_of_birth: date_of_birth.to_owned(),
                created_at: created_at.to_owned(),
            })
            .execute(&mut conn)?;

        user::table
            .order(user::columns::id.desc())
            .first::<DieselUser>(&mut conn)
            .map_err(From::from)
            .map(|user| {
                Self::new(
                    user.id as u32,
                    &user.name,
                    glib::DateTime::from_iso8601(&user.date_of_birth, None).unwrap(),
                    glib::DateTime::from_iso8601(&user.created_at, None).unwrap(),
                )
                .unwrap()
            })
    }

    pub fn new(
        id: u32,
        name: &str,
        date_of_birth: glib::DateTime,
        created_at: glib::DateTime,
    ) -> Result<User> {
        let user = glib::Object::builder::<Self>()
            .property("id", id)
            .property("name", name)
            .property("date-of-birth", date_of_birth)
            .property("created-at", created_at)
            .build();

        Ok(user)
    }

    pub fn find(id: u32) -> Result<User> {
        let db = database::connection();
        let mut conn = db.get()?;

        let result = user::table
            .filter(user::columns::id.eq(id as i32))
            .first::<DieselUser>(&mut conn)?;

        Ok(Self::new(
            result.id as u32,
            &result.name,
            glib::DateTime::from_iso8601(&result.date_of_birth, None).unwrap(),
            glib::DateTime::from_iso8601(&result.created_at, None).unwrap(),
        )?)
    }

    pub fn date_of_birth(&self) -> glib::DateTime {
        self.imp().date_of_birth.borrow().clone()
    }

    // Return the age in years
    pub fn age(&self) -> i32 {
        let now = glib::DateTime::now_local().unwrap();
        let date_of_birth = self.date_of_birth();
        let age = now.year() - date_of_birth.year();
        if now.month() < date_of_birth.month() {
            age - 1
        } else if now.month() == date_of_birth.month()
            && now.day_of_month() < date_of_birth.day_of_month()
        {
            age - 1
        } else {
            age
        }
    }

    pub fn created_at(&self) -> glib::DateTime {
        self.imp().created_at.borrow().clone()
    }

    pub fn find_all() -> Result<Vec<User>> {
        let db = database::connection();
        let mut conn = db.get()?;

        let result = user::table.load::<DieselUser>(&mut conn)?;

        Ok(result
            .into_iter()
            .map(|user| {
                Self::new(
                    user.id as u32,
                    &user.name,
                    glib::DateTime::from_iso8601(&user.date_of_birth, None).unwrap(),
                    glib::DateTime::from_iso8601(&user.created_at, None).unwrap(),
                )
                .unwrap()
            })
            .collect())
    }

    pub fn get_count() -> Result<i64> {
        let db = database::connection();
        let mut conn = db.get()?;

        let result = user::table.count().get_result::<i64>(&mut conn)?;

        Ok(result)
    }

    pub fn update(&self, name: Option<&str>, date_of_birth: Option<&str>) -> Result<()> {
        let db = database::connection();
        let mut conn = db.get()?;

        let binding = self.name();
        let user_name = match name {
            Some(name) => name,
            None => &binding,
        };

        let binding = self.date_of_birth().format_iso8601().unwrap();
        let user_date_of_birth = match date_of_birth {
            Some(date_of_birth) => date_of_birth,
            None => &binding,
        };

        let target = user::table.filter(user::columns::id.eq(self.id() as i32));
        diesel::update(target)
            .set((
                user::columns::name.eq(user_name),
                user::columns::date_of_birth.eq(user_date_of_birth),
            ))
            .execute(&mut conn)?;

        Ok(())
    }

    pub fn delete(&self) -> Result<()> {
        let db = database::connection();
        let mut conn = db.get()?;
        diesel::delete(user::table.filter(user::columns::id.eq(self.id() as i32)))
            .execute(&mut conn)?;
        Ok(())
    }
}
