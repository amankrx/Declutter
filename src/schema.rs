// @generated automatically by Diesel CLI.

diesel::table! {
    habit (id) {
        id -> Integer,
        user_id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
        categories -> Nullable<Text>,
        icon -> Nullable<Text>,
        frequency -> Text,
        created_at -> Text,
        updated_at -> Nullable<Text>,
        reminder_times -> Nullable<Text>,
        note -> Nullable<Text>,
        archived -> Integer,
        archived_date -> Nullable<Text>,
        archived_reason -> Nullable<Text>,
    }
}

diesel::table! {
    habit_entry (id) {
        id -> Integer,
        user_id -> Integer,
        habit_id -> Integer,
        entry_time -> Text,
        note -> Nullable<Text>,
        value -> Integer,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        name -> Text,
        date_of_birth -> Text,
        created_at -> Text,
    }
}

diesel::joinable!(habit -> user (user_id));
diesel::joinable!(habit_entry -> habit (habit_id));
diesel::joinable!(habit_entry -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(habit, habit_entry, user,);
