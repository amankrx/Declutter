use crate::models::duration_kind::DurationKind;
use gtk::glib;

// pub struct Habit {
//     pub id: u32,
//     pub name: String,
//     pub description: Option<String>,
//     pub icon: Option<String>,
//     pub category: HabitCategory,
//     pub is_quantifiable: bool,
//     pub frequency: Duration,
//     pub completed_times: u32,
//     pub current_streak: u32,
//     pub last_completed: glib::DateTime,
//     pub created: glib::DateTime,
//     pub updated: glib::DateTime,
//     pub reminder_time: Option<glib::DateTime>,
//     pub target_goal: Option<String>,
//     pub note: Option<String>,
//     pub user_id: u32,
//     pub archived: bool,
//     pub archived_date: Option<glib::DateTime>,
//     pub archived_reason: Option<String>,
// }

pub struct User {
    pub user_id: u32,
    pub user_name: Option<String>,
    pub date_of_birth: Option<glib::DateTime>,
    pub created_at: glib::DateTime,
}

pub struct HabitCategory {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
}

pub struct Habit {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub categories: Vec<HabitCategory>,
    pub frequency: DurationKind,
    pub frequency_value: u32,
    pub frequency_days_of_week: String,
    pub current_streak: u32,
    pub last_completed: glib::DateTime,
    pub created: glib::DateTime,
    pub updated: glib::DateTime,
    pub reminder_time: Option<glib::DateTime>,
    pub target_goal: Option<String>,
    pub note: Option<String>,
    pub user_id: u32,
    pub archived: bool,
    pub archived_date: Option<glib::DateTime>,
    pub archived_reason: Option<String>,
}

pub struct HabitEntry {
    pub id: u32,
    pub habit_id: u32,
    pub date: glib::DateTime,
    pub note: Option<String>,
    pub completed: bool,
}
