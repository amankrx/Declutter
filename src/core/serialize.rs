use std::cell::RefCell;

use gtk::glib;
use serde::{Deserialize, Deserializer, Serializer};

pub fn deserialize_datetime<'de, D>(deserializer: D) -> Result<glib::DateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;

    glib::DateTime::from_iso8601(&buf, None).map_err(serde::de::Error::custom)
}

pub fn deserialize_option_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<glib::DateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;
    let datetime = glib::DateTime::from_iso8601(&buf, None).map_err(serde::de::Error::custom)?;
    Ok(Some(datetime))
}

pub fn deserialize_vec_datetime<'de, D>(deserializer: D) -> Result<Vec<glib::DateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = Vec::<String>::deserialize(deserializer)?;
    let vec = buf
        .into_iter()
        .map(|s| glib::DateTime::from_iso8601(&s, None).map_err(serde::de::Error::custom))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(vec)
}

pub fn deserialize_option_vec_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<glib::DateTime>>, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = Vec::<String>::deserialize(deserializer)?;
    let vec = buf
        .into_iter()
        .map(|s| glib::DateTime::from_iso8601(&s, None).map_err(serde::de::Error::custom))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Some(vec))
}

pub fn deserialize_refcell_datetime<'de, D>(
    deserializer: D,
) -> Result<RefCell<glib::DateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;
    let datetime = glib::DateTime::from_iso8601(&buf, None).map_err(serde::de::Error::custom)?;
    Ok(RefCell::new(datetime))
}

pub fn deserialize_refcell_option_datetime<'de, D>(
    deserializer: D,
) -> Result<RefCell<Option<glib::DateTime>>, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;
    let datetime = glib::DateTime::from_iso8601(&buf, None).map_err(serde::de::Error::custom)?;
    Ok(RefCell::new(Some(datetime)))
}

pub fn deserialize_refcell_vec_datetime<'de, D>(
    deserializer: D,
) -> Result<RefCell<Vec<glib::DateTime>>, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = Vec::<String>::deserialize(deserializer)?;
    let vec = buf
        .into_iter()
        .map(|s| glib::DateTime::from_iso8601(&s, None).map_err(serde::de::Error::custom))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(RefCell::new(vec))
}

pub fn deserialize_refcell_option_vec_datetime<'de, D>(
    deserializer: D,
) -> Result<RefCell<Option<Vec<glib::DateTime>>>, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = Vec::<String>::deserialize(deserializer)?;
    let vec = buf
        .into_iter()
        .map(|s| glib::DateTime::from_iso8601(&s, None).map_err(serde::de::Error::custom))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(RefCell::new(Some(vec)))
}

// Serializer functions

pub fn serialize_datetime<S>(d: &glib::DateTime, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(d.format_iso8601().unwrap().as_str())
}

pub fn serialize_option_datetime<S>(d: &Option<glib::DateTime>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(date) = d {
        s.serialize_str(date.format_iso8601().unwrap().as_str())
    } else {
        s.serialize_str("")
    }
}

pub fn serialize_vec_datetime<S>(d: &Vec<glib::DateTime>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let buf = d
        .iter()
        .map(|date| date.format_iso8601().unwrap().to_string())
        .collect::<Vec<_>>();
    s.serialize_some(&buf)
}

pub fn serialize_option_vec_datetime<S>(
    d: &Option<Vec<glib::DateTime>>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(vec) = d {
        let buf = vec
            .iter()
            .map(|date| date.format_iso8601().unwrap().to_string())
            .collect::<Vec<_>>();
        s.serialize_some(&buf)
    } else {
        s.serialize_none()
    }
}

pub fn serialize_refcell_datetime<S>(d: &RefCell<glib::DateTime>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(d.borrow().format_iso8601().unwrap().as_str())
}

pub fn serialize_refcell_option_datetime<S>(
    d: &RefCell<Option<glib::DateTime>>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(date) = d.borrow().as_ref() {
        s.serialize_str(date.format_iso8601().unwrap().as_str())
    } else {
        s.serialize_str("")
    }
}

pub fn serialize_refcell_vec_datetime<S>(
    d: &RefCell<Vec<glib::DateTime>>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let buf = d
        .borrow()
        .iter()
        .map(|date| date.format_iso8601().unwrap().to_string())
        .collect::<Vec<_>>();
    s.serialize_some(&buf)
}

pub fn serialize_refcell_option_vec_datetime<S>(
    d: &RefCell<Option<Vec<glib::DateTime>>>,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(vec) = d.borrow().as_ref() {
        let buf = vec
            .iter()
            .map(|date| date.format_iso8601().unwrap().to_string())
            .collect::<Vec<_>>();
        s.serialize_some(&buf)
    } else {
        s.serialize_none()
    }
}

// Serialize a Vec of DateTime to a String for storing in a database
pub fn serialize_vec_datetime_to_string(d: &Vec<glib::DateTime>) -> String {
    let buf = d
        .iter()
        .map(|date| date.format_iso8601().unwrap().to_string())
        .collect::<Vec<_>>();
    serde_json::to_string(&buf).unwrap()
}

// Serialize a Struct to a String for storing in a database
pub fn serialize_struct_to_string<T>(d: &T) -> String
where
    T: serde::Serialize,
{
    serde_json::to_string(&d).unwrap()
}

// Deserialize a String to a Vec of DateTime for storing in a database
pub fn deserialize_string_to_vec_datetime(s: &str) -> Vec<glib::DateTime> {
    let buf = serde_json::from_str::<Vec<String>>(s).unwrap();
    buf.into_iter()
        .map(|s| glib::DateTime::from_iso8601(&s, None).unwrap())
        .collect::<Vec<_>>()
}

// Deserialize a String to a Struct for storing in a database
pub fn deserialize_string_to_struct<T>(s: &str) -> T
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_str::<T>(s).unwrap()
}
